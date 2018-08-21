extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(Positionable)]
pub fn positionable_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name: syn::Ident = ast.ident;

    let gen = quote! {
        impl Positionable for #name {
            fn x(&self) -> f64 {
                js_get!(self, x)
            }

            fn set_x(&self, new_x: f64) {
                js_set!(self, x, new_x);
            }

            fn y(&self) -> f64 {
                js_get!(self, y)
            }

            fn set_y(&self, new_y: f64) {
                js_set!(self, y, new_y);
            }
        }        
    };

    gen.into()
}

#[proc_macro_derive(Sizable)]
pub fn sizable_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name: syn::Ident = ast.ident;

    let gen = quote! {
        impl Sizable for #name {
            fn width(&self) -> f64 {
                js_get!(self, width)
            }

            fn set_width(&self, new_width: f64) {
                js_set!(self, width, new_width);
            }

            fn height(&self) -> f64 {
                js_get!(self, height)
            }

            fn set_height(&self, new_height: f64) {
                js_set!(self, height, new_height);
            }
        }     
    };

    gen.into()
}

#[proc_macro_derive(Rotatable)]
pub fn rotatable_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name: syn::Ident = ast.ident;

    // Build the impl
    let gen = quote! {
        impl Rotatable for #name {
            fn angle(&self) -> f64 {
                js_get!(self, rotation)
            }

            fn set_angle(&self, angle: f64) {
                js_set!(self, rotation, angle);
            }
        }  
    };

    gen.into()
}