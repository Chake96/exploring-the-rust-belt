#[cfg(test)]

use myGrep::search;

mod tests{
    use super::*;

    mod setup{
        pub let contents = "\
    In the beginning the Universe was created.
    This has made a lot of people very angry and been widely regarded as a bad move.
        ";
    }

    #[test]
    fn one_result(){
        let query = "ed";
        assert_eq!(vec!["In the beginning the Universe was created.","This has made a lot of people very angry and been widely regarded as a bad move."], search(query, setup::contents))
    }

    #[test]
    fn case_sensitive(){
        let query = "universe";
        assert_eq!(
            vec![],
            search_case_insensitive(query, setup::contents)
        );
    }
    #[test]
    fn case_insensitive(){
        let query = "Universe";
        assert_eq!(
            vec!["In the beginning the Universe was created."],
            search_case_insensitive(query, setup::contents)
        );
    }
}