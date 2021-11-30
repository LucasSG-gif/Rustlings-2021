// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` for hints!!

// I AM DONE

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4];
    if nice_slice == [2, 3, 4] {
        println!("Nice slice");
    } else {
        println!("bad slice: {:?}", nice_slice);
    }

}
