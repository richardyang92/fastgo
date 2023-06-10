pub const WINDOW_WIDTH: u32 = 1024;
pub const WINDOW_HEIGHT: u32 = 768;
pub const SCALE_FACTOR: f32 = 3.0 / 4.0;
pub const GO_KM: f32 = 7.5;
pub const GO_SZ: i32 = 19;

#[derive(Debug)]
pub struct Config {
    wind_width: u32,
    wind_height: u32,
    scale_factor: f32,
    go_km: f32,
    go_sz: i32,
}

impl Config {
    pub fn new(
        wind_width: u32,
        wind_height: u32,
        scale_factor: f32,
        go_km: f32,
        go_sz: i32,
    ) -> Self {
        Config {
            wind_width,
            wind_height,
            scale_factor,
            go_km,
            go_sz,
        }
    }

    pub fn window_width(&self) -> u32 {
        self.wind_width
    }

    pub fn window_height(&self) -> u32 {
        self.wind_height
    }

    pub fn scale_factor(&self) -> f32 {
        self.scale_factor
    }

    pub fn go_km(&self) -> f32 {
        self.go_km
    }

    pub fn go_sz(&self) -> i32 {
        self.go_sz
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::new(
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            SCALE_FACTOR,
            GO_KM,
            GO_SZ)
    }
}

impl From<Vec<String>> for Config {
    fn from(args: Vec<String>) -> Self {
        let mut config = Config::default();

        for mut i in 0..args.len() {
            let arg = args[i].as_str();
            if arg.eq("-w") {
                config.wind_width = args[i + 1].parse().unwrap();
                i += 1;
            } else if arg.eq("-h") {
                config.wind_height = args[i + 1].parse().unwrap();
                i += 1;
            } else if arg.eq("-sf") {
                config.scale_factor = args[i + 1].parse().unwrap();
                i += 1;
            } else if arg.eq("-km") {
                config.go_km = args[i + 1].parse().unwrap();
                i += 1;
            } else if arg.eq("-sz") {
                config.go_sz = args[i + 1].parse().unwrap();
                i += 1;
            }
        }

        config
    }
}