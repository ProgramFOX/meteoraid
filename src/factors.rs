pub fn cloud_factor(cloud_estimates: Vec<(u8, f64)>) -> f64 {
    let teff = cloud_estimates.iter().fold(0f64, |acc, &item| acc + item.1);
    let k = (1f64 / teff)
        * (cloud_estimates
            .iter()
            .fold(0f64, |acc, &item| acc + ((item.0 as f64) / 100f64) * item.1));
    ((1f64 / (1f64 - k)) * 100f64).round() / 100f64
}

pub fn limiting_magnitude(limiting_magnitude_measures: Vec<(f64, f64)>) -> f64 {
    let teff = limiting_magnitude_measures
        .iter()
        .fold(0f64, |acc, &item| acc + item.1);
    (((1f64 / teff)
        * (limiting_magnitude_measures
            .iter()
            .fold(0f64, |acc, &item| acc + (item.0 as f64) * item.1)))
        * 100f64)
        .round()
        / 100f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cloud_factor_1() {
        let ce: Vec<(u8, f64)> = vec![(5, 1.0)];
        assert_eq!(cloud_factor(ce), 1.05);
    }

    #[test]
    fn test_cloud_factor_2() {
        let ce: Vec<(u8, f64)> = vec![(10, 0.60), (5, 0.24), (0, 0.90), (15, 0.20)];
        assert_eq!(cloud_factor(ce), 1.06);
    }

    #[test]
    fn test_limiting_magnitude_1() {
        let lms: Vec<(f64, f64)> = vec![(5.64, 1.5)];
        assert_eq!(limiting_magnitude(lms), 5.64);
    }

    #[test]
    fn test_limiting_magnitude_2() {
        let lms: Vec<(f64, f64)> = vec![(5.64, 0.5), (5.12, 0.8), (6.14, 0.4)];
        assert_eq!(limiting_magnitude(lms), 5.51);
    }
}
