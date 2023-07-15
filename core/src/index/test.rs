use super::*;
pub mod ast {
    use super::*;

    #[test]
    fn doc_on_root_attr_set() {
        return;
        let mut fi = FileIndex::default();
        fi.with_nix(
            r#"# foo
{
    bar = "bar";
}"#,
        );
        assert_eq!(fi.docs(), vec![&Doc { doc: "foo".into() }]);
    }
}
