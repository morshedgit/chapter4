fn main() {
    // Ownership
    // // Move
    // let s1 = String::from("Hello");

    // let s2 = s1;

    // println!("{}",s1);

    
    // // Clone
    // let s1 = String::from("Hello");

    // let s2 = s1.clone();

    // println!("{}",s1);

    // // References and Borrowing
    // let mut s1 = String::from("Hello");

    // let len = calculate_length(&mut s1);

    // println!("The length of '{}' is {}.",s1,len);

    // // Restriction
    // let mut s = String::from("Hello");

    // let r1 = &mut s;
    // let r2 = &mut s; // Error

    // println!("{}, {}",r1,r2);

    // // Workaround with Scope
    // let mut s = String::from("hello");

    // {
    //     let r1 = &mut s;
    // }

    // let r2 = &mut s;
    
    // // Mutable and immutable refs
    // let mut s = String::from("hello");

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // println!("{} and {}", r1, r2);
    // // variables r1 and r2 will not be used after this point

    // let r3 = &mut s; // no problem
    // println!("{}", r3);
    
    // let s1 = String::from("Hello");

    // let s2 = &s1;

    // test_borrowing_ref(s2);

    // println!("s2 {s2}");
    
    // Summary:
    
    // At any given time, you can have either one mutable reference or any number of immutable references.
    // References must always be valid.

    // // The slice type

    // // String Slices
    // let s = String::from("Hello world");

    // let hello = &s[0..5];
    // let world = &s[6..11];
    

}

// fn first_world(s:&String)->usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
fn first_world_slice_version(s:&String) -> &str {
        let bytes = s.as_bytes();
    
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }
    &s[..]
}

// fn calculate_length(s:&mut String)->usize {
//     s.push_str(",world!");
//     s.len()
// }

// fn test_borrowing_ref(s:&String){
//     println!("Borrowd {s}");
// }
