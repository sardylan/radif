use proc_macro::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Data, DataEnum, DeriveInput, Lit, Variant, parse_macro_input};

#[proc_macro_derive(AdifData, attributes(adif))]
pub fn adif_enum_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let variants = if let Data::Enum(data_enum) = input.data {
        data_enum.variants
    } else {
        panic!("AdifEnum can only be derived for enums");
    };

    let serialize_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let adif_value = get_adif_value(variant)
            .expect("Missing `adif` attribute on variant for AdifEnum derive macro");

        quote! {
            Self::#variant_name => #adif_value.to_string(),
        }
    });

    let deserialize_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let adif_value = get_adif_value(variant)
            .expect("Missing `adif` attribute on variant for AdifEnum derive macro");

        quote! {
            #adif_value => Ok(Self::#variant_name),
        }
    });

    let expanded = quote! {
        impl AdifData for #name {
            fn serialize(&self) -> String {
                match self {
                    #(#serialize_arms)*
                }
            }

            fn deserialize(value: &str) -> crate::result::Result<Self> {
                match value {
                    #(#deserialize_arms)*
                    _ => Err(AdifError::DeserializeError("Invalid value for deserialization".to_string())),
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(AutoDisplay)]
pub fn auto_display_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let variants = if let Data::Enum(DataEnum { variants, .. }) = input.data {
        variants
    } else {
        panic!("AutoDisplay can only be derived for enums");
    };

    // Generate match arms for each variant
    let display_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let variant_string = variant_name.to_string();

        quote! {
            #name::#variant_name => write!(f, "{}", #variant_string),
        }
    });

    let expanded = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    #(#display_arms)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(AutoFromStr)]
pub fn auto_from_str_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let variants = if let Data::Enum(DataEnum { variants, .. }) = input.data {
        variants
    } else {
        panic!("AutoFromStr can only be derived for enums");
    };

    // Generate match arms for each variant
    let display_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let variant_string = variant_name.to_string();

        quote! {
            #variant_string => Ok(Self::#variant_name),
        }
    });

    let expanded = quote! {
        impl FromStr for #name {
            type Err = AdifError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #(#display_arms)*
                    _ => Err(AdifError::ParseError("".to_string())),
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(AutoTestEnum)]
pub fn auto_test_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let variants = if let Data::Enum(DataEnum { variants, .. }) = input.data {
        variants
    } else {
        panic!("AutoTestEnum can only be derived for enums");
    };

    let test_cases = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let variant_string = variant_name.to_string();
        let adif_value = get_adif_value(variant)
            .expect("Missing `adif` attribute on variant for AdifEnum derive macro");

        let fn_serialize_name = format_ident!("test_serialize_{}", &variant_string);
        let fn_deserialize_name = format_ident!("test_deserialize_{}", &variant_string);

        quote! {
            #[test]
            fn #fn_serialize_name() {
                assert_eq!(#name::#variant_name.serialize(), #adif_value.to_string());
            }

            #[test]
            fn #fn_deserialize_name() {
                assert_eq!(#name::deserialize(#adif_value).unwrap(), #name::#variant_name);
            }
        }
    });

    let expanded = quote! {
        #[cfg(test)]
        mod tests {
            use super::*;

            #(#test_cases)*
        }
    };

    TokenStream::from(expanded)
}

fn get_adif_value(variant: &Variant) -> Option<String> {
    for attr in &variant.attrs {
        if attr.path().is_ident("adif") {
            match attr.parse_args::<Lit>() {
                Ok(Lit::Str(lit_str)) => {
                    return Some(lit_str.value());
                }
                Ok(other) => {
                    println!(
                        "Expected a string literal for `adif`, but found: {}",
                        other.to_token_stream()
                    );
                }
                Err(e) => {
                    println!("Failed to parse `adif` attribute: {}", e);
                }
            }
        }
    }
    Some(variant.ident.to_string())
}
