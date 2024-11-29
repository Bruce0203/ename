#[proc_macro_derive(VariantName)]
pub fn var_name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item_enum = syn::parse_macro_input!(input as syn::ItemEnum);
    let enum_name = &item_enum.ident;
    let match_branches = item_enum
        .variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;
            let fields = match variant.fields {
                syn::Fields::Named(_) => quote::quote! { {..} },
                syn::Fields::Unnamed(_) => quote::quote! { (..) },
                syn::Fields::Unit => quote::quote! {},
            };
            quote::quote! { #enum_name::#variant_name #fields => stringify!(#variant_name) }
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
