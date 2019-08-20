#[allow(clippy::approx_constant)]
pub const AREAS: [&[f64]; 30] = [
    &[
        3.08, 3.18, 3.57, 3.74, 4.23, 4.78, 4.83, 5.00, 5.08, 5.25, 5.96, 6.06, 6.28, 6.42, 6.50,
        6.60, 6.63, 6.65, 6.66, 6.68, 6.70, 6.79, 6.86, 6.86, 6.86, 6.86, 6.87, 6.89, 6.92, 6.92,
        6.93, 6.94, 7.02, 7.03, 7.04, 7.09, 7.10, 7.10, 7.15, 7.24, 7.30, 7.31, 7.32, 7.33, 7.35,
        7.35, 7.36, 7.39, 7.43, 7.50,
    ],
    &[
        2.11, 2.88, 3.02, 3.78, 4.95, 5.15, 5.55, 5.60, 5.79, 5.80, 5.98, 6.01, 6.07, 6.40, 6.41,
        6.45, 6.50, 6.51, 6.54, 6.60, 6.61, 6.66, 6.72, 6.73, 6.75, 6.78, 6.85, 6.89, 6.90, 7.02,
        7.03, 7.03, 7.05, 7.15, 7.15, 7.16, 7.18, 7.22, 7.23, 7.24, 7.24, 7.25, 7.26, 7.27, 7.28,
        7.30, 7.31, 7.31, 7.33, 7.33, 7.35, 7.35, 7.36, 7.42, 7.45, 7.48, 7.49, 7.50, 7.50,
    ],
    &[
        2.35, 3.18, 3.65, 3.78, 4.48, 4.56, 4.83, 5.13, 5.16, 5.49, 5.66, 5.72, 5.79, 5.97, 6.19,
        6.30, 6.35, 6.41, 6.49, 6.49, 6.54, 6.59, 6.72, 6.77, 6.83, 6.85, 6.99, 7.01, 7.06, 7.12,
        7.12, 7.19, 7.20, 7.23, 7.24, 7.30, 7.33, 7.40, 7.41, 7.44, 7.45, 7.47, 7.48, 7.50,
    ],
    &[
        1.22, 2.02, 3.01, 3.79, 5.01, 5.07, 5.34, 5.75, 5.76, 5.78, 6.20, 6.37, 6.47, 5.54, 6.67,
        6.76, 6.80, 6.99, 7.00, 7.02, 7.10, 7.12, 7.17, 7.22, 7.43, 7.45, 7.46, 7.46, 7.47,
    ],
    &[
        2.71, 2.99, 3.37, 4.45, 5.16, 5.30, 5.53, 5.98, 6.02, 6.31, 6.36, 6.71, 6.72, 6.77, 6.80,
        6.90, 6.91, 6.96, 7.00, 7.05, 7.06, 7.07, 7.09, 7.10, 7.11, 7.27, 7.28, 7.38, 7.39, 7.40,
        7.41, 7.44, 7.45, 7.47,
    ],
    &[
        2.06, 2.49, 2.84, 4.66, 5.08, 5.49, 5.56, 5.80, 6.13, 6.14, 6.17, 6.25, 6.25, 6.26, 6.29,
        6.44, 6.47, 6.50, 6.57, 6.59, 6.60, 6.60, 6.67, 6.68, 6.68, 6.69, 6.72, 6.73, 6.74, 6.82,
        6.87, 6.89, 6.89, 7.07, 7.07, 7.10, 7.11, 7.12, 7.12, 7.14, 7.15, 7.19, 7.24, 7.27, 7.33,
        7.37, 7.43, 7.44, 7.45, 7.45, 7.45, 7.49, 7.49, 7.50,
    ],
    &[
        2.47, 3.23, 4.07, 4.23, 4.79, 5.12, 5.17, 5.26, 5.29, 5.36, 5.42, 5.73, 5.95, 5.96, 6.00,
        6.14, 6.19, 6.23, 6.44, 6.47, 6.48, 6.63, 6.69, 6.70, 6.71, 6.72, 6.84, 6.88, 6.92, 6.93,
        6.94, 6.97, 7.01, 7.04, 7.06, 7.08, 7.16, 7.18, 7.23, 7.24, 7.25, 7.25, 7.27, 7.29, 7.30,
        7.32, 7.35, 7.39, 7.43, 7.44, 7.46, 7.49,
    ],
    &[
        0.99, 1.68, 3.00, 4.62, 4.88, 4.95, 5.09, 5.29, 5.43, 5.51, 5.73, 5.84, 6.10, 6.19, 6.27,
        6.29, 6.36, 6.50, 6.55, 6.71, 6.76, 6.77, 6.87, 6.88, 6.95, 7.15, 7.17, 7.19, 7.21, 7.30,
        7.34,
    ],
    &[
        1.41, 2.13, 2.23, 2.56, 3.33, 4.41, 4.78, 5.42, 5.44, 5.48, 5.50, 5.58, 5.73, 5.92, 6.14,
        6.17, 6.27, 6.27, 6.31, 6.40, 6.43, 6.52, 6.61, 6.64, 6.78, 6.81, 6.84, 6.85, 6.95, 7.00,
        7.02, 7.06, 7.07, 7.10, 7.12, 7.12, 7.12, 7.13, 7.13, 7.22, 7.26, 7.30, 7.30, 7.31, 7.33,
        7.34, 7.36, 7.43, 7.43, 7.44, 7.45, 7.48, 7.49,
    ],
    &[
        1.06, 2.74, 3.38, 4.39, 5.77, 5.80, 5.86, 5.92, 5.97, 5.99, 6.12, 6.41, 6.44, 6.63, 6.64,
        6.65, 6.69, 6.83, 6.90, 7.04, 7.06, 7.08, 7.16, 7.19, 7.20, 7.24, 7.25, 7.25, 7.32, 7.33,
        7.34, 7.38, 7.42,
    ],
    &[
        0.16, 2.22, 2.36, 3.04, 3.57, 4.47, 4.51, 4.79, 4.81, 4.93, 5.28, 5.51, 5.67, 5.79, 5.81,
        5.88, 5.9, 6.00, 6.01, 6.04, 6.06, 6.13, 6.13, 6.22, 6.27, 6.32, 6.38, 6.38, 6.40, 6.40,
        6.56, 6.68, 6.70, 6.71, 6.76, 6.77, 6.79, 6.83, 6.84, 6.87, 6.89, 6.94, 6.95, 6.96, 6.96,
        7.01, 7.03, 7.04, 7.12, 7.14, 7.15, 7.17, 7.21, 7.22, 7.22, 7.25, 7.25, 7.25, 7.25, 7.25,
        7.25, 7.25, 7.30, 7.30, 7.30, 7.38, 7.43, 7.43, 7.43, 7.45, 7.45, 7.45, 7.49,
    ],
    &[
        2.61, 2.63, 2.73, 3.55, 5.10, 5.23, 5.39, 5.39, 5.51, 5.53, 5.57, 5.87, 6.25, 6.34, 6.51,
        6.52, 6.54, 6.71, 6.85, 6.87, 6.88, 6.95, 6.96, 6.97, 7.04, 7.13, 7.16, 7.16, 7.19, 7.21,
        7.23, 7.25, 7.26, 7.27, 7.27, 7.28, 7.32, 7.34, 7.35, 7.36, 7.41, 7.42, 7.43, 7.44, 7.47,
        7.48, 7.48, 7.50, 7.50,
    ],
    &[
        3.52, 3.84, 4.32, 4.34, 4.41, 4.98, 5.42, 5.49, 5.56, 5.72, 5.99, 6.01, 6.03, 6.05, 6.10,
        6.17, 6.47, 6.59, 6.62, 6.67, 6.70, 6.89, 6.93, 7.00, 7.01, 7.02, 7.02, 7.03, 7.04, 7.06,
        7.08, 7.19, 7.23, 7.27, 7.29, 7.31, 7.33, 7.34, 7.37, 7.37, 7.38, 7.41, 7.43, 7.44, 7.45,
        7.45, 7.46, 7.46, 7.49,
    ],
    &[
        2.23, 2.49, 3.90, 4.65, 4.73, 4.79, 4.94, 5.06, 5.39, 5.58, 5.64, 5.87, 5.91, 6.04, 6.25,
        6.29, 6.31, 6.34, 6.38, 6.47, 6.48, 6.60, 6.73, 6.74, 6.82, 6.87, 6.90, 6.96, 7.00, 7.02,
        7.02, 7.08, 7.09, 7.10, 7.12, 7.13, 7.23, 7.27, 7.29, 7.30, 7.32, 7.33, 7.34, 7.42, 7.42,
        7.43, 7.44, 7.44, 7.44, 7.47, 7.47,
    ],
    &[
        2.80, 3.14, 3.90, 4.82, 5.07, 5.50, 5.67, 5.82, 5.92, 5.98, 6.06, 6.11, 6.16, 6.17, 6.29,
        6.34, 6.36, 6.36, 6.45, 6.46, 6.58, 6.66, 6.66, 6.74, 6.78, 6.82, 6.85, 6.87, 6.87, 7.00,
        7.02, 7.04, 7.12, 7.17, 7.23, 7.24, 7.35, 7.37, 7.38, 7.39, 7.47, 7.48, 7.49, 7.49, 7.50,
        7.50,
    ],
    &[
        1.76, 1.86, 2.89, 4.67, 5.15, 5.64, 5.79, 5.85, 5.88, 6.11, 6.42, 6.48, 6.55, 6.70, 6.79,
        6.80, 6.81, 6.84, 6.96, 6.98, 6.98, 7.05, 7.06, 7.23, 7.26, 7.28, 7.33, 7.38, 7.47, 7.48,
    ],
    &[
        0.08, 1.90, 2.65, 3.03, 3.73, 3.97, 4.33, 4.52, 5.21, 5.46, 5.64, 5.91, 5.99, 6.09, 6.11,
        6.23, 6.30, 6.30, 6.41, 6.44, 6.47, 6.48, 6.51, 6.54, 6.56, 5.57, 6.58, 6.58, 6.59, 6.60,
        6.63, 6.66, 6.69, 6.75, 6.77, 6.80, 6.81, 6.82, 6.84, 6.86, 6.86, 6.89, 6.93, 6.95, 6.95,
        6.98, 6.98, 7.01, 7.16, 7.19, 7.20, 7.21, 7.24, 7.24, 7.24, 7.24, 7.24, 7.24, 7.24, 7.27,
        7.31, 7.31, 7.31, 7.31, 7.31, 7.31, 7.37, 7.40, 7.40, 7.40, 7.46, 7.46, 7.46, 7.46, 7.46,
        7.50,
    ],
    &[
        2.17, 3.87, 4.10, 4.26, 4.83, 4.87, 4.96, 5.01, 5.04, 5.64, 5.67, 5.94, 5.98, 6.13, 6.13,
        6.39, 6.42, 6.52, 6.55, 6.58, 6.60, 6.64, 6.65, 6.68, 6.68, 6.77, 6.77, 6.84, 6.90, 6.95,
        7.07, 7.14, 7.19, 7.21, 7.23, 7.23, 7.25, 7.26, 7.26, 7.27, 7.27, 7.30, 7.33, 7.43, 7.44,
        7.46, 7.47, 7.48, 7.50,
    ],
    &[
        2.06, 3.65, 3.89, 5.19, 5.50, 5.81, 6.20, 6.33, 6.40, 6.53, 6.70, 7.00, 7.17, 7.22, 7.25,
        7.30, 7.33, 7.41, 7.45, 7.49,
    ],
    &[
        4.03, 4.31, 4.62, 4.77, 5.14, 5.44, 5.47, 5.62, 5.63, 6.00, 6.04, 6.17, 6.17, 6.20, 6.21,
        6.24, 6.25, 6.35, 6.36, 6.38, 6.43, 6.47, 6.61, 6.62, 6.63, 6.64, 6.64, 6.66, 6.69, 6.71,
        6.74, 6.81, 6.82, 6.85, 6.86, 6.88, 6.89, 6.89, 6.92, 6.95, 6.97, 6.98, 6.99, 7.01, 7.03,
        7.05, 7.08, 7.12, 7.12, 7.14, 7.17, 7.27, 7.28, 7.30, 7.30, 7.32, 7.37, 7.37, 7.40, 7.40,
        7.43, 7.43, 7.43, 7.45, 7.47,
    ],
    &[
        1.23, 3.27, 3.68, 3.96, 4.48, 4.72, 5.54, 5.66, 5.98, 6.28, 6.30, 6.35, 6.79, 6.82, 6.97,
        7.05, 7.25, 7.42, 7.45, 7.46, 7.48, 7.50,
    ],
    &[
        0.28, 2.84, 3.29, 3.87, 4.28, 4.43, 4.47, 4.78, 5.46, 5.49, 5.68, 5.68, 5.69, 5.72, 5.82,
        5.96, 5.96, 6.05, 6.15, 6.23, 6.27, 6.35, 6.40, 6.42, 6.46, 6.47, 6.54, 6.68, 6.71, 6.73,
        6.75, 6.76, 6.96, 7.02, 7.04, 7.12, 7.14, 7.14, 7.21, 7.21, 7.22, 7.28, 7.32, 7.32, 7.33,
        7.34, 7.34, 7.37, 7.38, 7.38, 7.41, 7.42, 7.43, 7.43, 7.45, 7.45, 7.47, 7.78,
    ],
    &[
        2.59, 2.66, 2.97, 3.01, 5.21, 5.81, 5.95, 6.40, 6.62, 6.84, 7.06, 7.25, 7.30, 7.41, 7.44,
        7.44, 7.46,
    ],
    &[
        2.61, 2.75, 3.28, 3.92, 4.56, 5.19, 5.64, 5.72, 6.08, 6.14, 6.15, 6.17, 6.19, 6.41, 6.46,
        6.50, 6.63, 6.64, 6.67, 6.75, 6.76, 6.76, 6.80, 6.87, 6.94, 7.07, 7.14, 7.16, 7.19, 7.20,
        7.22, 7.24, 7.25, 7.29, 7.29, 7.32, 7.35, 7.37, 7.38, 7.41, 7.46, 7.49, 7.50,
    ],
    &[
        1.07, 2.29, 3.96, 5.26, 5.40, 5.50, 5.84, 5.92, 6.00, 6.09, 6.15, 6.32, 6.41, 6.47, 6.56,
        6.56, 6.62, 6.85, 6.90, 6.97, 6.98, 7.01, 7.07, 7.13, 7.14, 7.15, 7.26, 7.40, 7.46,
    ],
    &[
        -0.01, 1.91, 2.84, 2.88, 3.76, 3.85, 4.11, 4.85, 5.08, 5.10, 5.11, 5.17, 5.18, 5.29, 5.50,
        5.72, 5.75, 5.77, 5.89, 5.89, 5.95, 5.95, 6.02, 6.07, 6.12, 6.14, 6.16, 6.17, 6.20, 6.20,
        6.21, 6.22, 6.25, 6.25, 6.30, 6.31, 6.33, 6.39, 6.39, 6.42, 6.48, 6.48, 6.50, 6.50, 6.50,
        6.50, 6.57, 6.57, 6.61, 6.70, 6.70, 6.70, 6.75, 6.81, 6.81, 6.81, 6.81, 6.85, 6.85, 6.85,
        6.85, 6.85, 6.85, 6.90, 6.90, 6.95, 6.95, 6.95, 6.95, 7.00, 7.00, 7.00, 7.00, 7.00, 7.05,
        7.10, 7.10, 7.10, 7.10, 7.10, 7.14, 7.14, 7.20, 7.20, 7.20, 7.24, 7.24, 7.24, 7.24, 7.29,
        7.29, 7.34, 7.34, 7.34, 7.34, 7.34, 7.40, 7.40, 7.40, 7.40, 7.40, 7.45, 7.45, 7.45, 7.45,
        7.50,
    ],
    &[
        0.64, 1.31, 1.58, 1.65, 4.31, 4.56, 4.59, 4.61, 4.69, 4.92, 5.50, 5.75, 5.82, 6.04, 6.20,
        6.20, 6.23, 6.42, 6.61, 6.61, 6.66, 6.69, 6.73, 6.74, 6.75, 6.92, 6.93, 6.96, 6.98, 7.07,
        7.11, 7.13, 7.19, 7.19, 7.21, 7.24, 7.26, 7.27, 7.29, 7.31, 7.37, 7.38, 7.40, 7.45, 7.50,
    ],
    &[
        1.67, 1.95, 2.25, 3.84, 3.96, 4.00, 4.33, 5.46, 5.54, 5.78, 5.79, 6.36, 6.36, 6.49, 6.54,
        6.63, 6.72, 6.85, 6.90, 6.93, 6.99, 7.04, 7.08, 7.14, 7.15, 7.16, 7.18, 7.19, 7.25, 7.29,
        7.31, 7.37, 7.38, 7.38, 7.38, 7.38, 7.44, 7.45, 7.46,
    ],
    &[
        2.82, 2.86, 3.26, 4.08, 4.69, 4.74, 5.51, 5.57, 5.67, 5.99, 6.09, 6.36, 6.43, 6.57, 6.59,
        6.65, 6.66, 6.69, 6.69, 6.71, 6.77, 6.81, 6.84, 6.85, 6.86, 6.88, 6.89, 6.89, 6.91, 6.94,
        7.01, 7.09, 7.09, 7.10, 7.13, 7.19, 7.22, 7.22, 7.23, 7.24, 7.26, 7.27, 7.29, 7.30, 7.30,
        7.32, 7.32, 7.37, 7.37, 7.37, 7.38, 7.39, 7.41, 7.46, 7.47, 7.50, 7.50,
    ],
    &[
        1.92, 2.86, 3.42, 3.65, 3.95, 4.23, 4.76, 4.86, 5.12, 5.15, 5.18, 5.61, 5.62, 5.76, 5.92,
        6.09, 6.22, 6.22, 6.28, 6.33, 6.35, 6.36, 6.40, 6.50, 6.59, 6.70, 6.70, 6.73, 6.77, 6.83,
        6.84, 6.86, 6.87, 6.91, 6.92, 6.92, 6.97, 7.00, 7.03, 7.09, 7.10, 7.10, 7.12, 7.15, 7.18,
        7.20, 7.21, 7.23, 7.24, 7.24, 7.27, 7.35, 7.36, 7.41, 7.44, 7.44, 7.47, 7.48, 7.50, 7.50,
    ],
];

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Area(pub usize);

pub fn get_limiting_magnitude(stars: usize, area: Area) -> Option<f64> {
    let Area(area_number) = area;
    AREAS
        .get(area_number - 1)
        .and_then(|a| a.get(stars - 1).copied())
}

pub fn get_limiting_magnitude_avg(counts: &[(usize, Area)]) -> Option<f64> {
    let mut lms: Vec<f64> = counts
        .iter()
        .map(|count| get_limiting_magnitude(count.0, count.1))
        .flatten()
        .collect();
    lms.sort_by(|a, b| a.partial_cmp(b).unwrap());

    if lms.is_empty() || lms.len() != counts.len() {
        return None;
    } else if lms.len() == 1 {
        return Some(lms[0]);
    }

    // IMO handbook page 55:
    // "Whenever your limiting magnitude lies in a ‘gap’ wider than 0.3 mag, you should ignore this field"
    let mut selected: Vec<f64> = vec![];
    for i in 0..lms.len() {
        let left_gap = match i {
            0 => true,
            _ => lms[i] - lms[i - 1] > 0.3,
        };
        let right_gap = match lms.get(i + 1) {
            Some(next_lm) => next_lm - lms[i] > 0.3,
            None => true,
        };
        if !left_gap || !right_gap {
            selected.push(lms[i]);
        }
    }

    if selected.is_empty() {
        // Every magnitude lies in such a gap so let's take the overall average.
        Some(((lms.iter().sum::<f64>() / (lms.len() as f64)) * 100_f64).round() / 100_f64)
    } else {
        Some(((selected.iter().sum::<f64>() / (selected.len() as f64)) * 100_f64).round() / 100_f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_limiting_magnitude_1() {
        assert_eq!(get_limiting_magnitude(11, Area(14)), Some(5.64));
    }

    #[test]
    pub fn test_limiting_magnitude_2() {
        assert_eq!(get_limiting_magnitude(10, Area(7)), Some(5.36));
    }

    #[test]
    pub fn test_limiting_magnitude_3() {
        assert_eq!(get_limiting_magnitude(123, Area(11)), None);
    }

    #[test]
    pub fn test_limiting_magnitude_4() {
        assert_eq!(get_limiting_magnitude(2, Area(35)), None);
    }

    #[test]
    pub fn test_limiting_magnitude_average_1() {
        assert_eq!(get_limiting_magnitude_avg(&[(11, Area(14))]), Some(5.64));
    }

    #[test]
    pub fn test_limiting_magnitude_average_2() {
        assert_eq!(
            get_limiting_magnitude_avg(&[(11, Area(14)), (10, Area(7)), (8, Area(2))]),
            Some(5.53)
        );
    }

    #[test]
    pub fn test_limiting_magnitude_average_3() {
        assert_eq!(
            get_limiting_magnitude_avg(&[(11, Area(14)), (10, Area(7)), (15, Area(2))]),
            Some(5.50)
        );
    }

    #[test]
    pub fn test_limiting_magnitude_average_4() {
        assert_eq!(
            get_limiting_magnitude_avg(&[(1, Area(14)), (10, Area(7)), (15, Area(2))]),
            Some(4.67)
        );
    }

    #[test]
    pub fn test_limiting_magnitude_average_5() {
        assert_eq!(
            get_limiting_magnitude_avg(&vec![
                (1, Area(14)),
                (10, Area(7)),
                (15, Area(2)),
                (3, Area(35))
            ]),
            None
        );
    }

    #[test]
    pub fn test_limiting_magnitude_average_6() {
        assert_eq!(
            get_limiting_magnitude_avg(&[
                (11, Area(14)),
                (10, Area(7)),
                (15, Area(2)),
                (100, Area(3))
            ]),
            None
        );
    }

    #[test]
    pub fn test_limiting_magnitude_average_7() {
        assert_eq!(
            get_limiting_magnitude_avg(&[(110, Area(14)), (10, Area(70)), (153, Area(2))]),
            None
        );
    }

    #[test]
    pub fn test_limiting_magnitude_average_8() {
        assert_eq!(
            get_limiting_magnitude_avg(&[(12, Area(14)), (12, Area(7)), (10, Area(6))]),
            Some(5.91),
        );
    }
}
