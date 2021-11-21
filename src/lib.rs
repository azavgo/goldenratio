#[derive(Debug)]
pub struct GoldenRatio {
    fi: f64, 
}

impl GoldenRatio {
    fn new(fi: f64) -> Self {
        Self {fi: fi}
    }

    pub fn fi(&self) -> f64 {
        self.fi
    }

    pub fn gr_const() -> Self {
        Self::new(1.618033988749895_f64)
    }

    pub fn gr_sqrt() -> Self {
        Self::new(0.5_f64 + 0.5_f64 * 5.0_f64.sqrt())
    }
    
    pub fn gr_sin() -> Self {
        Self::new(((54.0_f64 * std::f64::consts::PI / 180.0_f64).sin()) * 2.0_f64)
    }

    pub fn gr_algo() -> Self {
        let mut v: f64 = 2.0; 
        for _i in 0..36 {
            v = 1.0_f64 + 1.0_f64 / v
        }
        Self::new(v)
    }
    
}
