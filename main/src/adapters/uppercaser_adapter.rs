use domain::Uppercaser as AppUppercaser;
use uppercaser::Uppercaser;

pub struct UppercaserAdapter {
    uppercaser: Uppercaser,
}

impl From<Uppercaser> for UppercaserAdapter {
    fn from(uppercaser: Uppercaser) -> Self {
        Self { uppercaser }
    }
}

impl AppUppercaser for UppercaserAdapter {
    fn to_uppercase(&self, str: String) -> String {
        self.uppercaser.to_uppercase(str)
    }
}
