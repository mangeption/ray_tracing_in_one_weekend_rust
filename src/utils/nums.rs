pub fn clamp<T>(input: T, min: T, max: T) -> T
where
    T: PartialOrd<T>,
{
    match input {
        x if x < min => min,
        x if x > max => max,
        x => x,
    }
}
