mod n44;

#[cfg(test)]
mod tests {
    use super::n44::is_match;

    #[test]
    fn n44_test() {
        let input = "aab".into();
        let pat = "c*a*b".into();

        println!("{}", is_match(input, pat));
    }
}
