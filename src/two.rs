use std::cmp::Ordering;

#[derive(Eq)]
struct CubeSet {
    red: u32,
    blue: u32,
    green: u32
}

impl PartialEq for CubeSet {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red &&
        self.green == other.green &&
        self.blue == other.blue
    }
}

impl PartialOrd for CubeSet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for CubeSet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.red.cmp(&other.red)
            .then(self.green.cmp(&other.green))
            .then(self.blue.cmp(&other.blue))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_than_cap() {
        let cs_cap = CubeSet {
            red: 12,
            blue: 13,
            green: 14
        };

        let cs_test = CubeSet {
            red: 16,
            blue: 9,
            green: 17
        };

        assert!(cs_cap < cs_test)
    }

    #[test]
    fn smaller_than_cap() {
        let cs_cap = CubeSet {
            red: 12,
            blue: 13,
            green: 14
        };

        let cs_test = CubeSet {
            red: 10,
            blue: 9,
            green: 11
        };

        assert!(cs_cap >= cs_test)
    }

    #[test]
    fn equal_to_cap() {
        let cs_cap = CubeSet {
            red: 12,
            blue: 13,
            green: 14
        };

        let cs_test = CubeSet {
            red: 12,
            blue: 9,
            green: 14
        };

        assert!(cs_cap >= cs_test)
    }
}
