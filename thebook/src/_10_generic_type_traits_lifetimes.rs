pub fn main() {
    dbg!();
    dangling_reference();
    lifetime_check();
    lifetime_annotations_in_struct_definitions();
    longest_with_an_announcement("xxx", "yyy", "displayable");
}

/// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#preventing-dangling-references-with-lifetimes
fn dangling_reference() {
    println!("--- dangling_reference ---");

    let r;
    {
        let x = 5;
        r = &x;
    }
    // won't compile because x's lifetime is shorter than r so that reference r will be an invalid ref.
    // println!("r: {}", r);

    let r;
    let x = 5;
    r = &x;
    // can compile because r and x are the same lifetime
    println!("r: {}", r);
}

/// This function is to to understand lifetime concept.
fn lifetime_check<'a>() {
    println!("--- lifetime check ---");

    let arg1 = format!("aaa");
    let arg2 = format!("bbb");
    let longest_ref = longest(&arg1, &arg2);
    // can compile because each ref's lifetime is the same.
    dbg!(longest_ref);

    let outer_arg = format!("aaa");
    let outer_longest: &str;
    {
        let inner_arg: String = format!("a");
        outer_longest = longest(&outer_arg, &inner_arg);
        // can compile because the required lifetime bound is this inner block and all refs can live as long as or longer than the block.
        dbg!(outer_longest);
    }
    // won't compile because inner_arg cannot live as long as outer_longest ref.
    // dbg!(outer_longest);

    let outer_longest: &str;
    {
        let inner_arg1 = "a";
        let inner_arg2 = "bb";
        outer_longest = longest(&inner_arg1, &inner_arg2);
    }
    // can compile because each inner_arg is created with string literals which can live during running this program.
    dbg!(outer_longest);
    return;
}

/// The compiler cannot figure out the lifetimes of the args and the returned reference.
/// A dangling reference occurs if the compiler compiles this code in the above situation.
/// Borrow Checker must prevent it.
/// Therefore we must annotate Lifetime to tell Rust that each reference lives at least as long as lifetime 'a.
///
/// Lifetime 'a is equal to the smaller of the lifetimes of x and y.
// fn longest(x: &str, y: &str) -> &str {
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// This can compile.
/// `y` does not have any relationship with `x` or the returned reference.
///
/// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#thinking-in-terms-of-lifetimes
fn longest_without_lifetime_annotation_on_y<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

/// This cannot compile.
/// The reason is that `result` is dropped at the end of the function whereas the returned reference to `result` lives longer.
/// If Rust can compile it, the pointer will be a dangling reference.
/// Therefore Rust disallows code like this to prevent it.
///
/// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#thinking-in-terms-of-lifetimes
// fn longest_cannot_compile<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

/// This can compile because the return value is moved, not borrowed.
fn longest_can_compile(x: &str, y: &str) -> String {
    String::from("really long string")
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

/// This function can compile because the novel and the same lifetime.
fn lifetime_annotations_in_struct_definitions() {
    let novel = "novel1, novel2".to_string();
    let first_sentence: &str = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence
    };
}

/// This function cannot compile.
/// The novel is owned by the function.
/// But it returns the reference, which means the returned reference tries to live longer than the lifetime of the novel.
// fn lifetime_annotations_in_struct_definitions_a<'a>() -> ImportantExcerpt<'a> {
//     let novel = "novel1, novel2".to_string();
//     let first_sentence: &str = novel.split('.').next().expect("Could not find a '.'");
//     let i = ImportantExcerpt {
//         part: first_sentence
//     };
//     i
// }


/// The Compiler applies the first and second rules and knows to enable to treat the function as `first_word_with_lifetime_annotations`.
///
/// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision
fn first_word_without_lifetime_annotation(s: &str) -> &str {
    s
}

#[allow(clippy::needless_lifetimes)]
fn first_word_with_lifetime_annotations<'a>(s: &'a str) -> &'a str {
    s
}

/// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotations-in-method-definitions
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }

    // This cannot compile.
    // The value referenced by the returned value is owned by the function.
    // But the lifetime of the returned reference is the same as &self (lifetime elision rule 3).
    // fn try_to_return_reference_to_string_owned_by_function(&self) -> &str {
    //     String::from("foo").as_str()
    // }
}

/// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#generic-type-parameters-trait-bounds-and-lifetimes-together
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
    where T: std::fmt::Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_string(x: String, y: String) -> String {
    if x.len() < y.len() {
        x
    } else {
        y
    }
}
