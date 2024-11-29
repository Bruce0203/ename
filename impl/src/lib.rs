#[proc_macro_derive(VariantName)]
pub fn var_name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item_enum = syn::parse_macro_input!(input as syn::ItemEnum);
    let enum_name = &item_enum.ident;
    let match_branches = item_enum
        .variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;
            let field_names = (0..variant.fields.len())
                .map(|_| quote::quote! {_})
                .collect::<Vec<_>>();
            let field_names = if field_names.is_empty() {
                None
            } else {
                Some(quote::quote! { (#(#field_names),*) })
            };
            quote::quote! { #enum_name::#variant_name #field_names => stringify!(#variant_name) }
        })
        .collect::<Vec<_>>();
    let enum_generic_params_without_bounds = &item_enum
        .generics
        .params
        .iter()
        .map(|param| match param {
            syn::GenericParam::Lifetime(lifetime_param) => &lifetime_param.lifetime.ident,
            syn::GenericParam::Type(type_param) => &type_param.ident,
            syn::GenericParam::Const(const_param) => &const_param.ident,
        })
        .collect::<Vec<_>>();
    let enum_generic_params = &item_enum.generics.params.iter().collect::<Vec<_>>();
    let enum_generic_where_cluase = &item_enum.generics.where_clause;
    quote::quote! {
    impl<#(#enum_generic_params),*> ename::VariantName
        for #enum_name<#(#enum_generic_params_without_bounds),*> #enum_generic_where_cluase {

            fn variant_name(&self) -> &'static str {
                match self {
                    #(#match_branches),*
                }
            }
        }
    }
    .into()
}
