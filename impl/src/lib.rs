#[proc_macro_derive(VariantName)]
pub fn var_name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::ItemEnum);
    let enum_name = &input.ident;
    let match_branches = input
        .variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;
            let field_names = (0..variant.fields.len())
                .map(|_| quote::quote! {_})
                .collect::<Vec<_>>();
            println!("{:?}", field_names);
            quote::quote! {
                #enum_name::#variant_name(#(#field_names),*) => stringify!(#variant_name)
            }
        })
        .collect::<Vec<_>>();
    quote::quote! {
    impl ename::VariantName for #enum_name {
            fn variant_name(&self) -> &'static str {
                match self {
                    #(#match_branches),*
                }
            }
        }
    }
    .into()
}
