use kits::*;

//Types

#[test]
fn type_sizes() {
    assert_eq!(std::mem::size_of::<Byte>(), 1);
    assert_eq!(std::mem::size_of::<Word>(), 2);
    assert_eq!(std::mem::size_of::<DWord>(), 4);
    assert_eq!(std::mem::size_of::<QWord>(), 8);

    assert_eq!(std::mem::size_of::<SignedByte>(), 1);
    assert_eq!(std::mem::size_of::<SignedWord>(), 2);
    assert_eq!(std::mem::size_of::<SignedDWord>(), 4);
    assert_eq!(std::mem::size_of::<SignedQWord>(), 8);
}

#[test]
fn size_constants() {
    assert_eq!(KB, 1024);
    assert_eq!(MB, 1024 * 1024);
    assert_eq!(GB, 1024 * 1024 * 1024)
}

//Assertions

#[test]
fn kits_assert_passes_on_true() {
    kits_assert!(true);
    kits_assert!(1+1 == 2);
}

#[test]
#[should_panic]
fn kits_assert_panics_on_false() {
    kits_assert!(false);
}

#[test]
#[should_panic(expected = "ASSERT_NOT_REACHED")]
fn assert_not_reached_panics() {
    assert_not_reached!();
}

#[test]
#[should_panic(expected="NOT_IMPLEMENTED")]
fn not_implemented_panics() {
    not_implemented();
}


//stdlib

#[test]
fn min_works() {
    assert_eq!(min(3, 7), 3);
    assert_eq!(min(10, 2), 2);
    assert_eq!(min(5, 5), 5);
}

#[test]
fn max_works() {
    assert_eq!(max(3, 7), 7);
    assert_eq!(max(10, 2), 10);
    assert_eq!(max(5, 5), 5);
}

#[test]
fn ceil_div_exact() {
    assert_eq!(ceil_div(10u32, 5), 2)
}

#[test]
fn ceil_div_rounds_up() {
    assert_eq!(ceil_div(10u32, 3), 4);

    assert_eq!(ceil_div(1u32, 2), 1);
}

#[test]
fn ceil_div_bitmap_use_case() {
    assert_eq!(ceil_div(8u32, 8), 1);
    assert_eq!(ceil_div(9u32, 8), 2);
    assert_eq!(ceil_div(16u32, 8), 2);
    assert_eq!(ceil_div(17u32, 8), 3);
}