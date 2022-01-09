use proc_macro::TokenStream;

fn replace_dollarsign(string: &str) -> String {
    string.replace("_dollarsign_", "$")
}

#[proc_macro] pub fn wrapping_count(item: TokenStream) -> TokenStream {
    let mut item = item.to_string();

    // Gonna eventually use generics?
    // Couldn't find a way to have generic numbers (AKA initialize a generic with 0)
    macro_rules! replace {
        ($($T:tt),*) => {
            $(
                {
                    let placeholder = concat!("_", stringify!($T), "_");

                    if item.contains(placeholder) {
                        let mut new_string = String::new();
                        let mut counter: $T = 0;

                        let mut split = item.split(placeholder);

                        if let Some(v) = split.next() {
                            new_string.push_str(v);
                        }

                        for to_replace in split {
                            new_string.push_str(&counter.to_string());
                            new_string.push_str(to_replace);
                            counter = counter.wrapping_add(1);
                        }

                        item = new_string;
                    }
                }
            )*
        };
    }

    replace!(
        i8, i16, i32, i64, i128, isize,
        u8, u16, u32, u64, u128, usize
    );

    replace_dollarsign(&item).parse().unwrap()
}

#[proc_macro] pub fn count(item: TokenStream) -> TokenStream {
    let mut item = item.to_string();

    macro_rules! replace {
        ($($T:tt),*) => {
            $(
                {
                    let placeholder = concat!("_", stringify!($T), "_");

                    if item.contains(placeholder) {
                        let mut new_string = String::new();
                        let mut int_counter: $T = 0;

                        let mut split = item.split(placeholder);

                        if let Some(v) = split.next() {
                            new_string.push_str(v);
                        }

                        for to_replace in split {
                            new_string.push_str(&int_counter.to_string());
                            new_string.push_str(to_replace);
                            match int_counter.checked_add(1) {
                                Some(_) => int_counter += 1,
                                _ => return r#"compile_error!("Counter overflown, try using wrapping_count or changing type")"#.parse().unwrap()
                            }
                        }

                        item = new_string;
                    }
                }
            )*
        };
    }

    replace!(
        i8, i16, i32, i64, i128, isize,
        u8, u16, u32, u64, u128, usize
    );

    replace_dollarsign(&item).parse().unwrap()
}
