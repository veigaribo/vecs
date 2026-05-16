use quote::quote;
use syn::{DeriveInput, parse_macro_input};

// I originally imagined doing something more complex, so that's why it's a derive macro.
// I didn't really want to delete it since it is more ergonomic like this.
#[proc_macro_derive(DisplayHash)]
pub fn my_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = parse_macro_input!(input as DeriveInput);

  let name = &input.ident;
  let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

  let output = quote! {
    #[automatically_derived]
    impl #impl_generics std::hash::Hash for #name #ty_generics #where_clause {
      fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut writer = display_hash::HashWriter { hasher: state };
        let mut fmtter = std::fmt::Formatter::new(&mut writer, std::fmt::FormattingOptions::new());
        let _ = std::fmt::Display::fmt(&self, &mut fmtter);
      }
    }
  };

  output.into()
}
