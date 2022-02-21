use proc_macro::{Group, Literal, Ident, TokenStream, TokenTree};

fn map_tokens(token: TokenTree, counter: &mut usize) -> TokenTree {
    match token {
        TokenTree::Ident(v) => {
            let ident = v.to_string();

            let old = counter.clone();
            if ident == "_int_" {
                *counter += 1;
                TokenTree::Literal(Literal::usize_unsuffixed(old))
            } else {
                let newident = ident.replace("_int_", &old.to_string());

                if newident != ident {
                    *counter += 1;
                }

                TokenTree::Ident(Ident::new(&newident, v.span()))
            }
        }

        TokenTree::Group(v) => TokenTree::Group(Group::new(
            v.delimiter(),
            v.stream()
                .into_iter()
                .map(|token| map_tokens(token, counter))
                .collect(),
        )),

        v => v,
    }
}

/// Count without wrapping (panic if counter exceeds usize).
/// Every instance of `_int_` will be replaced with the counter value and then the counter will be increased.
///
/// # Examples
///
/// ## Ident to literal
/// ```rust
///
/// use count_macro::count;
///
/// let a = count!(vec![_int_, _int_, _int_]);
/// assert_eq!(a, vec![0, 1, 2]);
///
/// ```
///
/// ## Ident to ident
/// ```rust
///
/// use count_macro::count;
///
/// count! {
///     let a_int_ = "Hello";
///     let a_int_ = "World";
/// }
///
/// assert_eq!(a0, "Hello");
/// assert_eq!(a1, "World");
///
/// ```
///
/// ## In macro
/// ```rust
///
/// use count_macro::count;
/// 
/// macro_rules! my_macro {
///     ($($v:expr),*) => {
///         count!{
///             $(
///                 let _ = $v; // Ignoring $v
/// 
///                 println!("{}", _int_);
///             )*
///         }
///     };
/// }
/// 
/// my_macro!('@', '@', '@', '@'); // Will print from 0 to 3
///
/// ```
#[proc_macro]
pub fn count(item: TokenStream) -> TokenStream {
    let mut counter = 0;

    item.into_iter()
        .map(|token| map_tokens(token, &mut counter))
        .collect()
}



fn wrapping_map_tokens(token: TokenTree, counter: &mut usize) -> TokenTree {
    match token {
        TokenTree::Ident(v) => {
            let ident = v.to_string();

            let old = counter.clone();
            if ident == "_int_" {
                *counter = counter.wrapping_add(1);
                TokenTree::Literal(Literal::usize_unsuffixed(old))
            } else {
                let newident = ident.replace("_int_", &old.to_string());

                if newident != ident {
                    *counter = counter.wrapping_add(1);
                }

                TokenTree::Ident(Ident::new(&newident, v.span()))
            }
        }

        TokenTree::Group(v) => TokenTree::Group(Group::new(
            v.delimiter(),
            v.stream()
                .into_iter()
                .map(|token| wrapping_map_tokens(token, counter))
                .collect(),
        )),

        v => v,
    }
}

/// Count with wrapping (wraps to 0 if counter exceeds usize).
/// Every instance of `_int_` will be replaced with the counter value and then the counter will be increased.
///
/// # Examples
///
/// ## Ident to literal
/// ```rust
///
/// use count_macro::wrapping_count;
///
/// let a = wrapping_count!(vec![_int_, _int_, _int_]);
/// assert_eq!(a, vec![0, 1, 2]);
///
/// ```
///
/// ## Ident to ident
/// ```rust
///
/// use count_macro::wrapping_count;
///
/// wrapping_count! {
///     let a_int_ = "Hello";
///     let a_int_ = "World";
/// }
///
/// assert_eq!(a0, "Hello");
/// assert_eq!(a1, "World");
///
/// ```
///
/// ## In macro
/// ```rust
///
/// use count_macro::wrapping_count;
/// 
/// macro_rules! my_macro {
///     ($($v:expr),*) => {
///         wrapping_count!{
///             $(
///                 let _ = $v; // Ignoring $v
/// 
///                 println!("{}", _int_);
///             )*
///         }
///     };
/// }
/// 
/// my_macro!('@', '@', '@', '@'); // Will print from 0 to 3
///
/// ```
#[proc_macro]
pub fn wrapping_count(item: TokenStream) -> TokenStream {
    let mut counter = 0;

    item.into_iter()
        .map(|token| wrapping_map_tokens(token, &mut counter))
        .collect()
}
