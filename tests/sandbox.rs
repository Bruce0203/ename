#[test]
fn test() {
    #[derive(name_variant::NamedVariant)]
    enum TestStruct<T, const N: usize> {
        A(i32),
        B(T),
        C(i32, bool),
    }
    let value = TestStruct::<i32, 123>::A(1);
    TestStruct::<i32, 123>::variant_name(&value);
}
