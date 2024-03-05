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

    //as_much_as_you_want!("dog", "cat"); 
    //i include bc I was worried what would happen if put in a not number since macros don't have an easy way to be type specific
    //but I think the for loop and inner implications imposed that on the times:expr
    
}

