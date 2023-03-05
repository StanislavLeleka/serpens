#[derive(PartialEq)]
pub enum Dimension {
    OneDim,
    TwoDim,
    ThreeDim,
}

impl Clone for Dimension {
    fn clone(&self) -> Self {
        match self {
            Self::OneDim => Self::OneDim,
            Self::TwoDim => Self::TwoDim,
            Self::ThreeDim => Self::ThreeDim,
        }
    }
}
