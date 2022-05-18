#![no_main]
#![feature(restricted_std)]

use cheapalloc as _;
use std::collections::BTreeMap;
use std::ffi::CString;
use std::mem;

// print routine using printf from c
extern "C" {
    pub fn puts(s: *const i8) -> i32;
}

extern "C" {
    pub fn putchar(c: i32) -> i32;
}

fn print(s: String) {
    let cs = CString::new(s).unwrap();

    unsafe {
        puts(cs.as_ptr());
        //putchar(0x0A);
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

// A Rectangle can be specified by where its top left and bottom right
// corners are in space
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    // Allocate this point on the heap, and return a pointer to it
    Box::new(Point { x: 0.0, y: 0.0 })
}

fn box_example() {
    // (all the type annotations are superfluous)
    // Stack allocated variables
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    };

    // Heap allocated rectangle
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });

    // The output of functions can be boxed
    let boxed_point: Box<Point> = Box::new(origin());

    // Double indirection
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    print(format!(
        "Point occupies {} bytes on the stack",
        mem::size_of_val(&point)
    ));
    print(format!(
        "Rectangle occupies {} bytes on the stack",
        mem::size_of_val(&rectangle)
    ));

    // box size == pointer size
    print(format!(
        "Boxed point occupies {} bytes on the stack",
        mem::size_of_val(&boxed_point)
    ));
    print(format!(
        "Boxed rectangle occupies {} bytes on the stack",
        mem::size_of_val(&boxed_rectangle)
    ));
    print(format!(
        "Boxed box occupies {} bytes on the stack",
        mem::size_of_val(&box_in_a_box)
    ));

    // Copy the data contained in `boxed_point` into `unboxed_point`
    let unboxed_point: Point = *boxed_point;
    print(format!(
        "Unboxed point occupies {} bytes on the stack",
        mem::size_of_val(&unboxed_point)
    ));
}

fn btreemap_example() {
    // type inference lets us omit an explicit type signature (which
    // would be `BTreeMap<&str, &str>` in this example).
    let mut movie_reviews = BTreeMap::new();

    // review some movies.
    movie_reviews.insert("Office Space", "Deals with real issues in the workplace.");
    movie_reviews.insert("Pulp Fiction", "Masterpiece.");
    movie_reviews.insert("The Godfather", "Very enjoyable.");
    movie_reviews.insert("The Blues Brothers", "Eye lyked it a lot.");

    // check for a specific one.
    if !movie_reviews.contains_key("Les Misérables") {
        print(format!(
            "We've got {} reviews, but Les Misérables ain't one.",
            movie_reviews.len()
        ));
    }

    // oops, this review has a lot of spelling mistakes, let's delete it.
    movie_reviews.remove("The Blues Brothers");

    // look up the values associated with some keys.
    let to_find = ["Up!", "Office Space"];
    for movie in &to_find {
        match movie_reviews.get(movie) {
            Some(review) => print(format!("{movie}: {review}")),
            None => print(format!("{movie} is unreviewed.")),
        }
    }

    // Look up the value for a key (will panic if the key is not found).
    print(format!("Movie review: {}", movie_reviews["Office Space"]));

    // iterate over everything.
    for (movie, review) in &movie_reviews {
        print(format!("{movie}: \"{review}\""));
    }
}

fn vector_example() {
    // Iterators can be collected into vectors
    let collected_iterator: Vec<i32> = (0..10).collect();
    print(format!("Collected (0..10) into: {:?}", collected_iterator));

    // The `vec!` macro can be used to initialize a vector
    let mut xs = vec![1i32, 2, 3];
    print(format!("Initial vector: {:?}", xs));

    // Insert new element at the end of the vector
    print("Push 4 into the vector".to_string());
    xs.push(4);
    print(format!("Vector: {:?}", xs));

    // The `len` method yields the number of elements currently stored in a vector
    print(format!("Vector length: {}", xs.len()));

    // Indexing is done using the square brackets (indexing starts at 0)
    print(format!("Second element: {}", xs[1]));

    // `pop` removes the last element from the vector and returns it
    print(format!("Pop last element: {:?}", xs.pop()));

    // `Vector`s can be easily iterated over
    print("Contents of xs:".to_string());
    for x in xs.iter() {
        print(format!("> {}", x));
    }

    // A `Vector` can also be iterated over while the iteration
    // count is enumerated in a separate variable (`i`)
    for (i, x) in xs.iter().enumerate() {
        print(format!("In position {} we have value {}", i, x));
    }

    // Thanks to `iter_mut`, mutable `Vector`s can also be iterated
    // over in a way that allows modifying each value
    for x in xs.iter_mut() {
        *x *= 3;
    }
    print(format!("Updated vector: {:?}", xs));
}

#[no_mangle]
pub extern "C" fn rust_main(_argc: i32, _argv: *const *const u8) -> i32 {
    vector_example();

    box_example();

    btreemap_example();

    0 // return 0 for success
}
