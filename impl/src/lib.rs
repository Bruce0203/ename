#[proc_macro_derive(VariantName)]
pub fn var_name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::ItemEnum);
    let enum_name = &input.ident;
    let variant_names: Vec<_> = input
        .variants
        .iter()
        .map(|variant| &variant.ident)
        .collect();
    quote::quote! {
    impl ename::VariantName for #enum_name {
            fn variant_name(&self) -> &'static str {
                match self {
                    #(#enum_name::#variant_names => stringify!(#variant_names),)*
                }
            }
        }
    }
    .into()
}
