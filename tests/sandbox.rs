use ename::VariantName;
use name_variant::NamedVariant;

#[test]
fn test() {
    #[derive(VariantName)]
    enum TestStruct<T, const N: usize> {
        A(i32),
        B(T),
        C(i32, bool),
        D {},
        E { v: i32 },
    }
    let value = TestStruct::<i32, 123>::A(1);
}
