use std::fmt; 
// Derive the `fmt::Debug` implementation for `Structure`. `Structure`
// is a structure which contains a single `i32`.
#[derive(Debug)]
struct Structure(i32);
// Put a `Structure` inside of the structure `Deep`. Make it printable
// also.
#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}:{}", count, v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}
fn main() {
    println!("Hello, world!");
    println!("I'm a new Rustacean!!");
 
        /* -------------------------Examples--------------------------- */
        // In general, the `{}` will be automatically replaced with any
        // arguments. These will be stringified.
        println!("{} days", 31);
    
        // Positional arguments can be used. Specifying an integer inside `{}`
        // determines which additional argument will be replaced. Arguments start
        // at 0 immediately after the format string
        println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    
        // As can named arguments.
        println!("{subject} {verb} {object}",
                 object="the lazy dog",
                 subject="the quick brown fox",
                 verb="jumps over");
    
        // Different formatting can be invoked by specifying the format character after a
        // `:`.
        println!("Base 10:               {}",   69420); //69420
        println!("Base 2 (binary):       {:b}", 69420); //10000111100101100
        println!("Base 8 (octal):        {:o}", 69420); //207454
        println!("Base 16 (hexadecimal): {:x}", 69420); //10f2c
        println!("Base 16 (hexadecimal): {:X}", 69420); //10F2C
    
    
        // You can right-justify text with a specified width. This will
        // output "    1". (Four white spaces and a "1", for a total width of 5.)
        println!("{number:>5}", number=1);
    
        // You can pad numbers with extra zeroes,
        //and left-adjust by flipping the sign. This will output "10000".
        println!("{number:0<5}", number=1);
    
        // You can use named arguments in the format specifier by appending a `$`
        println!("{number:0>width$}", number=1, width=5);

          // For Rust 1.58 and above, you can directly capture the argument from a
        // surrounding variable. Just like the above, this will output
        // "     1". 5 white spaces and a "1".
        let number: f64 = 1.0;
        let width: usize = 5;
        println!("{number:>width$}");
        /* -------------------------Examples--------------------------- */


    
       /* -------------------------Debug Activity--------------------------- */

        // This will not compile because `Structure` does not implement
        // fmt::Display
        //println!("This struct `{}` won't print...", Structure(3)); 
        println!("Now {:?} will print!", Structure(3));
        // The problem with `derive` is there is no control over how
       // the results look. What if I want this to just show a `7`?
        println!("Now {:?} will print!", Deep(Structure(7)));

        // {:#?} - pretty print
        let name = "Peter";
        let age = 27;
        let peter = Person { name, age };
    
        // Pretty print
        println!("{:#?}", peter);

       /* -------------------------Debug Activity--------------------------- */

    
       /* -------------------------Activity : Formatted print--------------------------- */

        // Rust even checks to make sure the correct number of arguments are
        // used.
        println!("My name is {0}, {1} {0}", "Bond", "James");
        // FIXME ^ Add the missing argument: "James" --> Done
    
        // Only types that implement fmt::Display can be formatted with `{}`. User-
        // defined types do not implement fmt::Display by default
    
        println!("Pi is roughly {pi:.prec$}", prec = 3, pi = 3.141592 );

        /* -------------------------Activity : Formatted print--------------------------- */

        
       /* -------------------------Activity : Formatting--------------------------- */


        //RGB (128, 255, 90) 0x80FF5A
        //RGB (0, 3, 254) 0x0003FE
        //RGB (0, 0, 0) 0x000000
        for color in [
            Color { red: 128, green: 255, blue: 90 },
            Color { red: 0, green: 3, blue: 254 },
            Color { red: 0, green: 0, blue: 0 },
        ].iter() {
            // Switch this to use {} once you've added an implementation
            // for fmt::Display.
            println!("RBG ({0}, {1}, {2}) 0x{0:02X}{1:02X}{2:02X}", color.red, color.green, color.blue);
        }
       /* -------------------------Activity : Formatting--------------------------- */

    
      /* -------------------------Activity : Display--------------------------- */


        let point = Complex { real: 3.3, imag: 7.2 };

        // Display: 3.3 + 7.2i
        // Debug: Complex { real: 3.3, imag: 7.2 }
        println!("Display: {}", point);
        println!("Debug: {:?}", point);

        // Testcase: List
        let v = List(vec![1, 2, 3]);
        println!("{}", v);
        /* -------------------------Activity : Display--------------------------- */

        
}
