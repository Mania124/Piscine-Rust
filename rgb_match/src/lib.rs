#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        match first {
            v if v == self.r => match second {
                v if v == self.b => {
                    self.r = second;
                    self.b = first;
                }
                v if v == self.g => {
                    self.r = second;
                    self.g = first;
                }
                v if v == self.a => {
                    self.r = second;
                    self.a = first;
                }
                _ => {}
            },

            v if v == self.g => match second {
                v if v == self.r => {
                    self.g = second;
                    self.r = first;
                }
                v if v == self.b => {
                    self.g = second;
                    self.b = first;
                }
                v if v == self.a => {
                    self.g = second;
                    self.a = first;
                }
                _ => {}
            },

            v if v == self.a => match second {
                v if v == self.r => {
                    self.a = second;
                    self.r = first;
                }
                v if v == self.b => {
                    self.a = second;
                    self.b = first;
                }
                v if v == self.g => {
                    self.a = second;
                    self.g = first;
                }
                _ => {}
            },

            v if v == self.b => match second {
                v if v == self.r => {
                    self.b = second;
                    self.r = first;
                }
                v if v == self.g => {
                    self.b = second;
                    self.g = first;
                }
                v if v == self.a => {
                    self.b = second;
                    self.a = first;
                }
                _ => {}
            },
            _ => {}
        }

        Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}
