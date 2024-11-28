use ename::VariantName;

#[test]
fn test() {
    #[derive(VariantName)]
    enum TestStruct {
        A,
        B,
        C,
    }
    let value = TestStruct::A;
    value.variant_name();
}
