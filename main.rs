macro_rules! thrice {
    //have it print input 3 times
    ( $( $x:expr),*) => {
        {
            $(
            println!("{}", $x);
            println!("{}", $x);
            println!("{}", $x);
            )*
        }
    };
}

macro_rules! as_much_as_you_want {
    //have it print input as many times as requested
    ($times:expr, $( $x:expr),*) => {
        {    
            for _ in 0..$times {
                $(
                println!("{}", $x);
            )*
            }
            
        }
    };
}

fn main() {
    thrice!("Hi");
    as_much_as_you_want!(4, "wassup");
    
}

