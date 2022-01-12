#[cfg(test)]
mod test{
    use super::*;

    struct Tester{
        x:u32,

        y:u32
    }

    #[test]
    fn testing_testing_123(){
        let tstr = Tester{
            x:0,
            y:3
        };

        assert!(tstr.y > tstr.x);
        assert!(tstr.x < 3);
    }

}