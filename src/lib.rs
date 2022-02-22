use std::collections::HashMap;

use once_cell::sync::Lazy;
use proc_macro::{Group, Ident, Literal, TokenStream, TokenTree};
use regex::Regex;

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"_int(?:_([^_ \s]+))?_").unwrap());

fn map_tokens(token: TokenTree, counters: &mut HashMap<String, usize>) -> TokenTree {
    match token {
        TokenTree::Ident(v) => {
            let mut ident = v.to_string();
            let mut has_changed = false;

            #[allow(clippy::redundant_clone)]
            for captures in RE.captures_iter(&ident.clone()) {
                let id = match captures.get(1) {
                    Some(id) => id.as_str().to_string(),

                    _ => "!@".to_string(),
                };

                let counter = match counters.get_mut(&id) {
                    Some(v) => v,

                    _ => {
                        counters.insert(id.clone(), 0);
                        counters.get_mut(&id).unwrap()
                    }
                };

                let full_match = &captures[0];
                if full_match == ident {
                    let to_ret = TokenTree::Literal(Literal::usize_unsuffixed(*counter));
                    *counter += 1;
                    return to_ret;
                } else {
                    has_changed = true;
                    ident = ident.replace(&captures[0], &counter.to_string());
                    *counter += 1;
                }
            }

            if has_changed {
                return TokenTree::Ident(Ident::new(&ident, v.span()));
            }

            TokenTree::Ident(v)
        }

        TokenTree::Group(v) => TokenTree::Group(Group::new(
            v.delimiter(),
            v.stream()
                .into_iter()
                .map(|token| map_tokens(token, counters))
                .collect(),
        )),

        v => v,
    }
}

/// Count without wrapping (panic if counter exceeds usize).
/// Every instance of `_int_` and `_int_countername_` will be replaced with the counter value and then the counter will be increased.
///
/// # Examples
///
/// ## Ident to literal
/// ```rust
/// use count_macro::count;
///
/// let a = count!(vec![_int_, _int_, _int_]);
/// assert_eq!(a, vec![0, 1, 2]);
///
/// ```
///
/// ## Ident to ident
/// ```rust
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
/// ## Multiple counters
/// ```rust
/// use count_macro::count;
///
/// // With two different counters
/// // _int_ does not increment _int_name_
/// count! {
///     let a_int_ = _int_name_;
///     let a_int_ = _int_name_;
///     let a_int_ = _int_name_;
/// }
///
/// assert_eq!(a0, 0);
/// assert_eq!(a1, 1);
/// assert_eq!(a2, 2);
///
/// ```
#[proc_macro]
pub fn count(item: TokenStream) -> TokenStream {
    let mut counters = HashMap::new();

    item.into_iter()
        .map(|token| map_tokens(token, &mut counters))
        .collect()
}

fn wrapping_map_tokens(token: TokenTree, counters: &mut HashMap<String, usize>) -> TokenTree {
    match token {
        TokenTree::Ident(v) => {
            let mut ident = v.to_string();
            let mut has_changed = false;

            #[allow(clippy::redundant_clone)]
            for captures in RE.captures_iter(&ident.clone()) {
                let id = match captures.get(1) {
                    Some(id) => id.as_str().to_string(),

                    _ => "!@".to_string(),
                };

                let counter = match counters.get_mut(&id) {
                    Some(v) => v,

                    _ => {
                        counters.insert(id.clone(), 0);
                        counters.get_mut(&id).unwrap()
                    }
                };

                let full_match = &captures[0];
                if full_match == ident {
                    let to_ret = TokenTree::Literal(Literal::usize_unsuffixed(*counter));
                    *counter = counter.wrapping_add(1);
                    return to_ret;
                } else {
                    has_changed = true;
                    ident = ident.replace(&captures[0], &counter.to_string());
                    *counter = counter.wrapping_add(1);
                }
            }

            if has_changed {
                return TokenTree::Ident(Ident::new(&ident, v.span()));
            }

            TokenTree::Ident(v)
        }

        TokenTree::Group(v) => TokenTree::Group(Group::new(
            v.delimiter(),
            v.stream()
                .into_iter()
                .map(|token| map_tokens(token, counters))
                .collect(),
        )),

        v => v,
    }
}


/// Count with wrapping (wraps to 0 if counter exceeds usize).
/// Every instance of `_int_` and `_int_countername_` will be replaced with the counter value and then the counter will be increased.
///
/// # Examples
///
/// ## Ident to literal
/// ```rust
/// use count_macro::wrapping_count;
///
/// let a = wrapping_count!(vec![_int_, _int_, _int_]);
/// assert_eq!(a, vec![0, 1, 2]);
///
/// ```
///
/// ## Ident to ident
/// ```rust
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
///
/// ## Multiple counters
/// ```rust
/// use count_macro::wrapping_count;
///
/// // With two different counters
/// // _int_ does not increment _int_name_
/// wrapping_count! {
///     let a_int_ = _int_name_;
///     let a_int_ = _int_name_;
///     let a_int_ = _int_name_;
/// }
///
/// assert_eq!(a0, 0);
/// assert_eq!(a1, 1);
/// assert_eq!(a2, 2);
///
/// ```
#[proc_macro]
pub fn wrapping_count(item: TokenStream) -> TokenStream {
    let mut counters = HashMap::new();

    item.into_iter()
        .map(|token| wrapping_map_tokens(token, &mut counters))
        .collect()
}
