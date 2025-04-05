#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Light {
	pub alias: String,
	pub brightness: u8,
}

impl Light {
	pub fn new(alias: &str) -> Self {
        Self{
            alias:alias.to_string(),
            brightness:0,
        }
	}
}

pub fn change_brightness(lights: &mut [Light], alias: &str, value: u8) {
    for light in lights{
        if alias.to_string()==light.alias{
            light.brightness=value;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut lights = ["living_room", "bedroom", "rest_room"].map(Light::new);
        let result1 = lights[0].brightness;
        assert_eq!(result1, 0);
        change_brightness(&mut lights, "living_room", 200);
        let result2 = lights[0].brightness;
        assert_eq!(result2, 200);
    }
}
