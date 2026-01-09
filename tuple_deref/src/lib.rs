use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn tuple_deref(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let field_ty = match &input.fields {
        syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
            &fields.unnamed.first().unwrap().ty
        }
        _ => {
            return syn::Error::new_spanned(
                &input.ident,
                "tuple_deref only supports tuple structs with exactly one field",
            )
            .to_compile_error()
            .into();
        }
    };

    let expanded = quote! {
        #input

        impl #impl_generics ::std::ops::Deref for #name #ty_generics #where_clause {
            type Target = #field_ty;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl #impl_generics ::std::ops::DerefMut for #name #ty_generics #where_clause {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };

    expanded.into()
}
