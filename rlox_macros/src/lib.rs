use proc_macro::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{parse_macro_input, Data, DeriveInput, Fields, FieldsUnnamed};

#[proc_macro_derive(Visitor)]
#[allow(unused)]
pub fn derive_visitor(item: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, vis, data, ..
    } = parse_macro_input!(item as DeriveInput);

    let Data::Enum(data_enum) = data else {
        return quote_spanned! { ident.span()=> compile_error!("Visitor can only be derived for enums.") }.into();
    };

    let variants = data_enum.variants;
    let mut methods = Vec::new();
    let mut method_names = Vec::new();
    let mut ref_impls = Vec::new();
    let mut match_arms = Vec::new();

    for variant in variants {
        let ident_lower = format_ident!("{}", camel_to_snake(variant.ident.to_string()));
        let variant_ident = &variant.ident;
        let Fields::Unnamed(FieldsUnnamed { unnamed, .. }) = variant.fields else {
            return quote_spanned! { variant.ident.span()=> compile_error!("Missing inner type") }
                .into();
        };
        let inner_type = &unnamed.first().unwrap().ty;
        let method_name = format_ident!("visit_{}", ident_lower);

        let method = quote! {
            fn #method_name(&self, inner: &#inner_type) -> Self::Return
        };
        methods.push(method);

        let ref_impl = quote! {
            fn #method_name(&self, inner: &#inner_type) -> Self::Return {
                (*self).#method_name(inner)
            }
        };
        ref_impls.push(ref_impl);

        let match_arm = quote! {
            #ident::#variant_ident(inner) => visitor.#method_name(inner)
        };
        match_arms.push(match_arm);
        method_names.push(method_name);
    }

    let trait_name = format_ident!("{}Visitor", ident);

    quote! {
        #vis trait #trait_name {
            type Return;
            #(#methods;)*
        }

        impl<T: #trait_name> #trait_name for &T {
            type Return = T::Return;
            #(
                #[inline]
                #ref_impls
            )*
        }

        impl #ident {
            pub fn accept<V: #trait_name>(&self, visitor: V) -> V::Return {
                match self {
                    #(#match_arms),*
                }
            }
        }
    }
    .into()
}

fn camel_to_snake(s: String) -> String {
    let mut snake = String::with_capacity(s.len());
    let mut iter = s.chars().rev().peekable();
    while let Some(c) = iter.next() {
        if c.is_uppercase() {
            snake.push(c.to_ascii_lowercase());
            if iter.peek().is_some() {
                snake.push('_')
            }
        } else {
            snake.push(c);
        }
    }
    snake.chars().rev().collect()
}
