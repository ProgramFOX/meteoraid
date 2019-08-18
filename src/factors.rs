pub fn cloud_factor(cloud_estimates: &Vec<(u8, u32)>) -> f64 {
    let teff = cloud_estimates.iter().fold(0u32, |acc, &item| acc + item.1);
    let k = (1f64 / teff as f64)
        * (cloud_estimates.iter().fold(0f64, |acc, &item| {
            acc + (item.0 as f64 / 100f64) * item.1 as f64
        }));
    ((1f64 / (1f64 - k)) * 100f64).round() / 100f64
}

pub fn limiting_magnitude(limiting_magnitude_measures: &Vec<(f64, u32)>) -> f64 {
    let teff = limiting_magnitude_measures
        .iter()
        .fold(0u32, |acc, &item| acc + item.1);
    (((1f64 / teff as f64)
        * (limiting_magnitude_measures
            .iter()
            .fold(0f64, |acc, &item| acc + item.0 * item.1 as f64)))
        * 100f64)
        .round()
        / 100f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cloud_factor_1() {
        let ce: Vec<(u8, u32)> = vec![(5, 60)];
        assert_eq!(cloud_factor(&ce), 1.05);
    }

    #[test]
    fn test_cloud_factor_2() {
        let ce: Vec<(u8, u32)> = vec![(10, 36), (5, 15), (0, 54), (15, 12)];
        assert_eq!(cloud_factor(&ce), 1.06);
    }

    #[test]
    fn test_limiting_magnitude_1() {
        let lms: Vec<(f64, u32)> = vec![(5.64, 90)];
        assert_eq!(limiting_magnitude(&lms), 5.64);
    }

    #[test]
    fn test_limiting_magnitude_2() {
        let lms: Vec<(f64, u32)> = vec![(5.64, 30), (5.12, 48), (6.14, 24)];
        assert_eq!(limiting_magnitude(&lms), 5.51);
    }
}
