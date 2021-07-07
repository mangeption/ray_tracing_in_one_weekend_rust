pub fn clamp<T>(input: T, min: T, max: T) -> T
where
    T: PartialOrd<T>,
{
    if input < min {
        return min;
    }

    if input > max {
        return max;
    }

    input
}
