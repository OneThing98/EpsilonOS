pub fn min<T: PartialOrd>(a: T, b: T) -> T{
    if a < b { a } else { b }
}

pub fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a < b { b } else { a }
}

//Integer division that rounds up

pub fn ceil_div<T>(a: T, b: T) -> T 
where
    T: Copy
    + std::ops::Div<Output = T> //T must implement the Div trait and when you divide two Ts, the result must also be a T
    + std::ops::Rem<Output = T>
    + std::ops::Add<Output = T>
    + PartialEq
    + From<u8>,
{
    let result = a / b;
    //Both sides are type T now
    if (a % b) != T::from(0u8) {
        result + T::from(1u8)
    } else {
        result
    }

}

