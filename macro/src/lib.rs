extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Fields};

#[allow(clippy::too_many_lines)]
#[proc_macro_derive(UserControl, attributes(parent, state, child, childSelf))]
pub fn user_control_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let mut parent = parse_quote!(());
    let mut state = parse_quote!(());
    let mut child_field = None;

    for attr in &input.attrs {
        if attr.path().is_ident("parent") {
            attr.parse_nested_meta(|meta| {
                parent = meta.path.to_token_stream();
                Ok(())
            })
            .expect("Error parsing parent type attribute");
        } else if attr.path().is_ident("state") {
            attr.parse_nested_meta(|meta| {
                state = meta.path.to_token_stream();
                Ok(())
            })
            .expect("Error parsing state type attribute");
        }
    }

    let expanded = match &input.data {
        Data::Enum(data_enum) => {
            let variants = data_enum.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                match &variant.fields {
                    Fields::Unnamed(fields) => {
                        if fields.unnamed.len() != 1{
                            panic!("Multiple element in enum case");
                        }
                        let field_type = fields.unnamed.first().expect("").ty.to_token_stream();
                        (quote! {
                            #name::#variant_name(el) => UserControl::surface(el.into(), parent, state),
                        },quote! {
                            #name::#variant_name(el) => UserControl::event(el.into(), canvas, event, parent, state),
                        },quote! {
                            #name::#variant_name(el) => UserControl::update(el.into(), canvas, elapsed, parent, state),
                        },quote! {
                            #name::#variant_name(el) => UserControl::draw(el.into(), canvas, parent, state),
                        },quote! {
                            impl #impl_generics From<#field_type> for #name #type_generics #where_clause {
                                fn from(value: #field_type) -> Self {
                                    #name::#variant_name(value)
                                }
                            }
                        })
                    }
                    _ => panic!("Wrong enum format"),
                }
            }).collect::<Vec<(proc_macro2::TokenStream,proc_macro2::TokenStream,proc_macro2::TokenStream,proc_macro2::TokenStream,proc_macro2::TokenStream)>>();
            let surfaces = variants.iter().map(|(s, _, _, _, _)| s);
            let events = variants.iter().map(|(_, s, _, _, _)| s);
            let updates = variants.iter().map(|(_, _, s, _, _)| s);
            let draws = variants.iter().map(|(_, _, _, s, _)| s);
            let froms = variants.iter().map(|(_, _, _, _, s)| s);

            quote! {
                impl #impl_generics UserControl<#parent, #state> for #name #type_generics #where_clause {
                    fn surface(this: Ref<Self>, parent: Ref<#parent>, state: Ref<#state>) -> FRect {
                        match this.as_ref() {
                            #(#surfaces)*
                        }
                    }

                    fn event(
                        mut this: MutRef<Self>,
                        canvas: &Canvas<Window>,
                        event: Event,
                        parent: MutRef<#parent>,
                        state: MutRef<#state>,
                    ) -> Result<()> {
                        match this.as_mut() {
                            #(#events)*
                        }
                    }

                    fn update(
                        mut this: MutRef<Self>,
                        canvas: &Canvas<Window>,
                        elapsed: Duration,
                        parent: MutRef<#parent>,
                        state: MutRef<#state>,
                    ) -> Result<()> {
                        match this.as_mut() {
                            #(#updates)*
                        }
                    }

                    fn draw(this: Ref<Self>, canvas: &mut Canvas<Window>, parent: Ref<#parent>, state: Ref<#state>) -> Result<()> {
                        match this.as_ref() {
                            #(#draws)*
                        }
                    }
                }

                #(#froms)*
            }
        }
        Data::Struct(data_struct) => {
            for field in &data_struct.fields {
                for attr in &field.attrs {
                    if attr.path().is_ident("child") || attr.path().is_ident("childSelf") {
                        if child_field.is_some()
                            || (attr.path().is_ident("child") && attr.path().is_ident("childSelf"))
                        {
                            panic!("the struct can't have multiple child.");
                        }
                        child_field = Some((
                            field.ident.clone().expect("Expected named field"),
                            attr.path().is_ident("childSelf"),
                        ));
                    }
                }
            }
            let (child_field, child_self) =
                child_field.expect("Expected a field with the 'child' attribute");

            let (name_parent, used_parent) = if child_self {
                (
                    quote! {_}.to_token_stream(),
                    quote! {this}.to_token_stream(),
                )
            } else {
                (
                    quote! {parent}.to_token_stream(),
                    quote! {parent}.to_token_stream(),
                )
            };
            quote! {
                impl #impl_generics UserControl<#parent, #state> for #name #type_generics #where_clause {
                    fn surface(this: Ref<Self>, #name_parent: Ref<#parent>, state: Ref<#state>) -> FRect {
                        UserControl::surface((&this.#child_field).into(), #used_parent, state)
                    }

                    fn event( mut this: MutRef<Self>, canvas: &Canvas<Window>, event: Event, #name_parent: MutRef<#parent>, state: MutRef<#state>, ) -> Result<()> {
                        UserControl::event((&mut this.#child_field).into(), canvas, event, #used_parent, state)
                    }

                    fn update( mut this: MutRef<Self>, canvas: &Canvas<Window>, elapsed: Duration, #name_parent: MutRef<#parent>, state: MutRef<#state>, ) -> Result<()> {
                        UserControl::update((&mut this.#child_field).into(), canvas, elapsed, #used_parent, state)
                    }

                    fn draw(this: Ref<Self>, canvas: &mut Canvas<Window>, #name_parent: Ref<#parent>, state: Ref<#state>) -> Result<()> {
                        UserControl::draw((&this.#child_field).into(), canvas, #used_parent, state)
                    }
                }
            }
        }
        _ => panic!(
            "\nHow to use the derive macro:\n\n\
            #[parent(PARENT)]\n\
            #[state(STATE)]\n\
            enum A{{\n\
                Child1(UserControl),\n\
                Child2(UserControl),\n\
                ...\n\
            }}\n\n\
            #[parent(PARENT)]\n\
            #[state(STATE)]\n\
            struct A{{\n\
                #[child] or #[childSelf] for self as parent\n\
                Child: UserControl\n\
            }}"
        ),
    };
    TokenStream::from(expanded)
}
