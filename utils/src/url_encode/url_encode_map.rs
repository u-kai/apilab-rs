use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub(super) struct UrlEncodeExceptionMap(pub HashMap<char, &'static str>);
impl UrlEncodeExceptionMap {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert(' ', "+");
        map.insert('!', "%20");
        map.insert('!', "%21");
        map.insert('"', "%22");
        map.insert('#', "%23");
        map.insert('$', "%24");
        map.insert('%', "%25");
        map.insert('&', "%26");
        map.insert('\'', "%27");
        map.insert('(', "%28");
        map.insert(')', "%29");
        map.insert('*', "%2A");
        //map.insert('+', "%2B");
        map.insert(',', "%2C");
        map.insert('-', "-");
        map.insert('.', ".");
        map.insert('/', "%2F");
        map.insert(':', "%3A");
        map.insert(';', "%3B");
        map.insert('<', "%3C");
        map.insert('=', "%3D");
        map.insert('>', "%3E");
        map.insert('?', "%3F");
        map.insert('@', "%40");
        map.insert('[', "%5B");
        map.insert('\\', "%5C");
        map.insert(']', "%5D");
        map.insert('^', "%5E");
        map.insert('_', "_");
        map.insert('`', "%60");
        map.insert('{', "%7B");
        map.insert('|', "%7C");
        map.insert('}', "%7D");
        map.insert('~', "~");
        Self(map)
    }
    pub fn remove(&mut self, c: &char) {
        self.0.remove(c);
    }
}

//
