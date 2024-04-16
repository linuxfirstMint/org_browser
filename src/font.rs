pub const FONT_PATH: &str = "./fonts/static/SourceCodePro-LightItalic.ttf";

#[cfg(test)]
mod tests {
    use super::*;

    mod fonts {

        use super::*;
        use std::path::Path;

        #[test]
        fn font_path_exist() {
            assert_eq!(Path::new(FONT_PATH).exists(), true)
        }
    }
}
