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
    sgf_path: String,
    go_km: f32,
    go_sz: i32,
    go_pb: String,
    go_pw: String,
}

impl Config {
    pub fn new(
        wind_width: u32,
        wind_height: u32,
        scale_factor: f32,
        sgf_path: String,
        go_km: f32,
        go_sz: i32,
        go_pb: String,
        go_pw: String,
    ) -> Self {
        Config {
            wind_width,
            wind_height,
            scale_factor,
            sgf_path,
            go_km,
            go_sz,
            go_pb,
            go_pw,
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

    pub fn go_pb(&self) -> String {
        self.go_pb.clone()
    }

    pub fn go_pw(&self) -> String {
        self.go_pw.clone()
    }

    pub fn sgf_path(&self) -> String {
        self.sgf_path.clone()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::new(
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            SCALE_FACTOR,
            String::default(),
            GO_KM,
            GO_SZ,
            String::default(),
            String::default())
    }
}

impl From<Vec<String>> for Config {
    fn from(args: Vec<String>) -> Self {
        let mut config = Config::default();

        for mut _i in 0..args.len() {
            let arg = args[_i].as_str();
            if arg.eq("-w") {
                config.wind_width = args[_i + 1].parse().unwrap();
                _i += 1;
            } else if arg.eq("-h") {
                config.wind_height = args[_i + 1].parse().unwrap();
                _i += 1;
            } else if arg.eq("-sf") {
                config.scale_factor = args[_i + 1].parse().unwrap();
                _i += 1;
            } else if arg.eq("-km") {
                config.go_km = args[_i + 1].parse().unwrap();
                _i += 1;
            } else if arg.eq("-sz") {
                config.go_sz = args[_i + 1].parse().unwrap();
                _i += 1;
            } else if arg.eq("-sgf") {
                config.sgf_path = args[_i + 1].clone();
                _i += 1;
            } else if arg.eq("-pb") {
                config.go_pb = args[_i + 1].clone();
                _i += 1;
            } else if arg.eq("-pw") {
                config.go_pw = args[_i + 1].clone();
                _i += 1;
            }
        }

        config
    }
}