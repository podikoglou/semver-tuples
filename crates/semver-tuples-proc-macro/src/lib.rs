use proc_macro::{TokenStream, TokenTree};
use proc_macro2::Group;
use syn::parse::Parse;

struct Repeat {
    /// The pattern to repeat
    pattern: TokenTree,

    /// Amount of time to repeat
    repetitions: usize,
}

impl Parse for Repeat {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let pattern: Group = input.parse()?;
    }
}

/// Repeats a series of tokens (first argument) an N amount of times (second argument)
///
/// # Example
/// ```
/// use semver_tuples_proc_macro::repeat;
///
/// assert_eq!(vec![repeat!(3,; 6)], [3, 3, 3, 3, 3, 3]);
/// ````
#[proc_macro]
pub fn repeat(stream: TokenStream) -> TokenStream {
    // TODO ;replace with syn
    if stream.is_empty() {
        panic!("invalid tokens!")
    }

    let mut tokens: Vec<TokenTree> = stream.into_iter().collect();

    // parse last argument (repetitions count)
    let repetitions_token = tokens.remove(tokens.len() - 1);

    let repetitions = match repetitions_token {
        TokenTree::Literal(literal) => literal.to_string().parse::<usize>().ok(),
        _ => None,
    }
    .expect("couldn't parse reptitions count");

    let mut stream = TokenStream::new();

    for _ in 0..repetitions {
        stream.extend(tokens.clone());
    }

    stream
}
