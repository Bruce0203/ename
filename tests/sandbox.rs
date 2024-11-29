use ename::VariantName;

#[test]
fn test() {
    #[derive(VariantName)]
    enum TestStruct<T, const N: usize> {
        A(i32),
        B(T),
        C(i32, bool),
    }
    let value = TestStruct::<i32, 123>::A(1);
}
