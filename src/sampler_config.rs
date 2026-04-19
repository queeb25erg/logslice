/// Configuration for log sampling parsed from CLI args
#[derive(Debug, Clone, PartialEq)]
pub enum SamplerConfig {
    None,
    EveryN(usize),
    Percent(f64),
}

impl SamplerConfig {
    pub fn from_args(every_n: Option<usize>, percent: Option<f64>) -> Result<Self, String> {
        match (every_n, percent) {
            (Some(_), Some(_)) => Err("Cannot specify both --every-n and --sample-percent".into()),
            (Some(n), None) => {
                if n == 0 {
                    Err("--every-n must be greater than 0".into())
                } else {
                    Ok(SamplerConfig::EveryN(n))
                }
            }
            (None, Some(p)) => {
                if p <= 0.0 || p > 100.0 {
                    Err("--sample-percent must be in range (0, 100]".into())
                } else {
                    Ok(SamplerConfig::Percent(p))
                }
            }
            (None, None) => Ok(SamplerConfig::None),
        }
    }

    pub fn is_active(&self) -> bool {
        !matches!(self, SamplerConfig::None)
    }
}
