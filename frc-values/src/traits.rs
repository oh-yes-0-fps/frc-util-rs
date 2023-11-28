use crate::FrcValue;



pub trait IntoFrcValue {
    fn into_frc_value(self) -> FrcValue;
}

impl<T: Into<FrcValue>> IntoFrcValue for T {
    fn into_frc_value(self) -> FrcValue {
        self.into()
    }
}