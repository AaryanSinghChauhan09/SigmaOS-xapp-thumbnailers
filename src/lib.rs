pub struct ThumbnailerEngine;
impl ThumbnailerEngine {
    pub fn can_thumbnail(mime: &str) -> bool {
        matches!(mime, "application/pdf" | "image/jpeg" | "image/png" | "video/mp4")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_thumbnailer_mime_support() {
        assert!(ThumbnailerEngine::can_thumbnail("application/pdf"));
        assert!(!ThumbnailerEngine::can_thumbnail("application/x-executable"));
    }
}
