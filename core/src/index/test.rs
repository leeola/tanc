use super::*;
pub mod ast {
    use super::*;

    #[test]
    fn doc_on_root_attr_set() {
        let mut ti = TancIndex::default();
        ti.insert(
            "foo.nix",
            r#"# foo
# bar
{
    bar = "bar";
}"#,
        );
        assert_eq!(
            ti.docs("foo.nix"),
            vec![&Doc {
                doc: Some("foo".into())
            }]
        );
    }
}
