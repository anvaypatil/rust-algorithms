#[macro_export]
macro_rules! vecmac {
    () => {
        Vec::new()
    };
    ($($element:expr),+ $(,)?)=>{{
        let mut vec = Vec::new();
        $(
            vec.push($element);
        )+
        vec
    }}
}



#[cfg(test)]
mod tests {
    #[test]
    fn empty_vec() {
        let x: Vec<u32> = vecmac![];
        assert!(x.is_empty());
    }

    #[test]
    fn with_single_element() {
        let x: Vec<u32> = vecmac![42];
        assert_eq!(x.len(), 1);
        assert_eq!(x[0], 42);
    }

    #[test]
    fn with_multiple_element() {
        let x: Vec<u32> = vecmac![42,21];
        assert_eq!(x.len(), 2);
        assert_eq!(x[1], 21);
    }

    #[test]
    fn with_multiple_element_comma() {
        let x: Vec<u32> = vecmac![42,21,];
        assert_eq!(x.len(), 2);
        assert_eq!(x[1], 21);
    }
}
