use proffer::*;

fn normalize_whitespace(s: &str) -> String {
    s.split("\n")
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
        .collect::<String>()
}

#[test]
fn basic_gen() {
    let struct_ = Struct::new("Basic")
        .set_is_pub(true)
        .add_field(
            Field::new("field1", "String")
                .set_is_pub(true)
                .add_annotation("#[serde = w]")
                .add_doc("/// Some example documentation")
                .add_docs(vec!["/// Another line", "/// and another"]),
        )
        .add_field(Field::new("field2", "usize"));
    let expected = r#"
        pub struct Basic {
            /// Some example documentation
            /// Another line
            /// and another
            #[serde = w]
            pub field1: String,
            field2: usize,
         }
        "#
    .to_owned();
    let src_code = struct_.generate();
    println!("{}", &src_code);
    assert_eq!(
        normalize_whitespace(&src_code),
        normalize_whitespace(&expected)
    );
}

#[test]
fn generic_gen() {
    let s = Struct::new("Generic")
        .set_is_pub(true)
        .add_generic(Generic::new("T", vec!["ToString"]))
        .add_generic(Generic::new("S", vec!["ToString", "Number"]))
        .add_field(Field::new("field1", "S"))
        .add_field(Field::new("field2", "T"));
    let src_code = s.generate();
    println!("{}", &src_code);
    let expected = r#"
        pub struct Generic<T, S>
            where
                T: ToString,
                S: ToString + Number,
        {
            field1: S,
            field2: T,
        }
    "#;
    let src_code = s.generate();
    assert_eq!(
        normalize_whitespace(&src_code),
        normalize_whitespace(&expected)
    );
}
