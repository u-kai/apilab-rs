use tools::auto_type_define::json_to_struct::parse;

fn main() {
    let s = r#"
    {
        "data":[
            {
                "id":103,
                "name":"kai"
            }
        ]
    }
    "#;
    println!("{:#?}", parse(s));
}
