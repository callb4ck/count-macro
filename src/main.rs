use count_macro::count;

macro_rules! stuff {
    ($single:expr, $($v:expr),*) => {
        println!("{}", $single);
        count!{
            $(
                println!("{} {}", _i32_, $v);
            )*
        }
    };
}

fn main() {
    stuff!(0,1,2,3);
}
