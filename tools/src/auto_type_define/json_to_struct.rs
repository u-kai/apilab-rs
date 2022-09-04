use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
pub enum JsonType {
    String(String),
    Number(f64),
    Array(Vec<JsonType>),
    Object(HashMap<String, JsonType>),
    Boolean(bool),
    Null,
}

enum JsonChar {
    DoubleQuote,
    Colon,
    LeftCurlyBracket,
    RightCurlyBracket,
    LeftSquareBracket,
    RightSquareBracket,
}
#[cfg(test)]
mod json_define_to_struct {
    #[test]
    fn test_json_define_to_struct() {
        let json = r#"{
            "test":"test_value"
        }"#;
        let struct_name = "TestJson";
        let tobe = r#"#[derive(Serialize, Deserialize)]
struct TestJson {
    test:Option<String>
}
"#
        .to_string();
        //assert!(false);
        //assert_eq!(parse_and_define_struct_result, tobe);
    }
}

pub fn parse(s: &str) -> Option<JsonType> {
    json_value(s).and_then(|(value, rest)| {
        if rest.chars().all(|c| c.is_ascii_whitespace()) {
            Some(value)
        } else {
            None
        }
    })
}
fn json_value(s: &str) -> Option<(JsonType, &str)> {
    crate::choice![null, false_, true_, number, json_string, array, object](s)
}
fn object(s: &str) -> Option<(JsonType, &str)> {
    let p = crate::join![
        lcharacter('{'),
        separated(key_value, lcharacter(',')),
        lcharacter('}')
    ];
    let p = map(p, |((_, key_values), _)| {
        let h = HashMap::from_iter(key_values.into_iter());
        JsonType::Object(h)
    });
    p(s)
}

fn key_value(s: &str) -> Option<((String, JsonType), &str)> {
    // key_value = string ':' json_value
    let p = crate::join![json_string_raw, lcharacter(':'), json_value];
    let p = map(p, |((key, _), value)| (key, value));
    p(s)
}
fn array(s: &str) -> Option<(JsonType, &str)> {
    let p = crate::join![
        lcharacter('['),
        separated(json_value, lcharacter(',')),
        lcharacter(']')
    ];
    let p = map(p, |((_, values), _)| JsonType::Array(values));
    p(s)
}
fn null(s: &str) -> Option<(JsonType, &str)> {
    let p = lstring("null");
    let p = map(p, |_| JsonType::Null);
    p(s)
}
fn false_(s: &str) -> Option<(JsonType, &str)> {
    let p = lstring("false");
    let p = map(p, |_| JsonType::Boolean(false));
    p(s)
}
fn true_(s: &str) -> Option<(JsonType, &str)> {
    let p = lstring("true");
    let p = map(p, |_| JsonType::Boolean(true));
    p(s)
}
fn number(s: &str) -> Option<(JsonType, &str)> {
    let re = Regex::new(r"^-?(0|[1^9][0-9]*)(\.[0-9]+)?([eE][+-]?[0-9]+)?").unwrap();
    let p = regex(&re, |s| s.parse::<f64>().ok());
    let p = lexeme(p);
    let p = map(p, |x| JsonType::Number(x));
    p(s)
}
fn json_string(s: &str) -> Option<(JsonType, &str)> {
    map(json_string_raw, JsonType::String)(s)
}
fn json_string_raw(s: &str) -> Option<(String, &str)> {
    let p = crate::join!(character('"'), many(json_character), character('"'));
    let p = lexeme(p);
    let p = map(p, |((_, chars), _)| chars.into_iter().collect());
    p(s)
}
fn json_character(s: &str) -> Option<(char, &str)> {
    // character = <Any codepoint except " or \ or control characters>
    //           | '\u' <4 hex digits>
    //           | '\"' | '\\' | '\/' | '\b' | '\f' | '\n' | '\r' | '\t'
    crate::choice![
        crate::regex!(r#"^[^"\\[:cntrl:]]"#, |s| s.chars().next()),
        crate::regex!(r#"^\\u[0-9a-fA-F]{4}"#, hex_code),
        crate::regex!(r#"^\\."#, escape)
    ](s)
}
#[macro_export]
macro_rules! regex {
    ($pattern:expr, $f:expr) => {{
        use once_cell::sync::Lazy;
        use regex::Regex;
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new($pattern).unwrap());
        $crate::auto_type_define::json_to_struct::regex(&RE, $f)
    }};
}

fn hex_code(code: &str) -> Option<char> {
    code.strip_prefix(r"\u").and_then(|hex| {
        u32::from_str_radix(hex, 16)
            .ok()
            .and_then(|cp| char::from_u32(cp))
    })
}

fn escape(s: &str) -> Option<char> {
    match s {
        "\\\"" => Some('"'),
        "\\\\" => Some('\\'),
        "\\/" => Some('/'),
        "\\b" => Some('\x08'),
        "\\f" => Some('\x0C'),
        "\\n" => Some('\n'),
        "\\r" => Some('\r'),
        "\\t" => Some('\t'),
        _ => None, // undefined escape sequence
    }
}

fn lstring(target: &'static str) -> impl Parser<()> {
    lexeme(string(target))
}
fn lcharacter(c: char) -> impl Parser<()> {
    lexeme(character(c))
}
fn digits(s: &str) -> Option<(i64, &str)> {
    let end = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
    match s[..end].parse() {
        Ok(value) => Some((value, &s[end..])),
        Err(_) => None,
    }
}
fn many<T>(parser: impl Parser<T>) -> impl Parser<Vec<T>> {
    generalize_lifetime(move |mut s| {
        let mut ret = vec![];
        while let Some((value, rest)) = parser(s) {
            ret.push(value);
            s = rest;
        }
        Some((ret, s))
    })
}
fn separated<T>(parser: impl Parser<T>, separator: impl Parser<()>) -> impl Parser<Vec<T>> {
    generalize_lifetime(move |mut s| {
        let mut ret = vec![];
        match parser(s) {
            Some((value, rest)) => {
                ret.push(value);
                s = rest;
            }
            None => {
                return Some((ret, s));
            }
        }
        while let Some((_, rest)) = separator(s) {
            s = rest;
            match parser(s) {
                Some((value, rest)) => {
                    ret.push(value);
                    s = rest;
                }
                None => return None,
            }
        }
        Some((ret, s))
    })
}
fn choice<T>(parser1: impl Parser<T>, parser2: impl Parser<T>) -> impl Parser<T> {
    generalize_lifetime(move |s| parser1(s).or_else(|| parser2(s)))
}
#[macro_export]
macro_rules! choice {
    ($parser0:expr, $($parser:expr),*) => {{
        let p = $parser0;
        $(
            let p = $crate::auto_type_define::json_to_struct::choice(p, $parser);
        )*
        p
    }};
}
fn regex<'a, T>(re: &'a Regex, f: impl Fn(&str) -> Option<T> + 'a) -> impl Parser<T> + 'a {
    generalize_lifetime(move |s| {
        re.find(s).and_then(|matched| {
            f(matched.as_str()).map(|value| {
                let rest = &s[matched.end()..];
                (value, rest)
            })
        })
    })
}
fn join<A, B>(parser1: impl Parser<A>, parser2: impl Parser<B>) -> impl Parser<(A, B)> {
    generalize_lifetime(move |s| {
        parser1(s).and_then(|(value1, rest1)| {
            parser2(rest1).map(|(value2, rest2)| ((value1, value2), rest2))
        })
    })
}
#[macro_export]
macro_rules! join {
    ($parser0:expr, $($parser:expr),*) => {{
        let p = $parser0;
        $(
            let p = $crate::auto_type_define::json_to_struct::join(p, $parser);
        )*
        p
    }};
}

fn character(c: char) -> impl Parser<()> {
    generalize_lifetime(move |s| {
        let mut chars = s.chars();
        if chars.next() == Some(c) {
            Some(((), chars.as_str()))
        } else {
            None
        }
    })
}

fn lexeme<T>(parser: impl Parser<T>) -> impl Parser<T> {
    generalize_lifetime(move |s| parser(s.trim_start()))
}
fn string(target: &'static str) -> impl Parser<()> {
    generalize_lifetime(move |s| s.strip_prefix(target).map(|rest| ((), rest)))
}
fn map<A, B>(parser: impl Parser<A>, f: impl Fn(A) -> B) -> impl Parser<B> {
    generalize_lifetime(move |s| parser(s).map(|(value, rest)| (f(value), rest)))
}
pub trait Parser<T>: Fn(&str) -> Option<(T, &str)> {}
impl<T, F> Parser<T> for F where F: Fn(&str) -> Option<(T, &str)> {}
fn generalize_lifetime<T, F>(f: F) -> F
where
    F: Fn(&str) -> Option<(T, &str)>,
{
    f
}
