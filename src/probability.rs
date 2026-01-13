use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::ops::Mul;

/*
 * Custom probability type
 * Functions as a float which can only hold values x: 0.0 <= x <= 1.0
 */
#[derive(Debug, Clone, Copy)]
pub struct Probability {
    value: f64,
}

impl Display for Probability {
    // Function to be able to print Probabilities
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        Debug::fmt(&self.value, fmt)
    }
}

impl Probability {
    /*
     * Creates a new probability with the given float value
     * Probability will be fixed to 0.0 if the given value is NaN or < 0
     * Probability will be fixed to 1.0 if the given value is infinity or > 1.0
     */
    pub fn new(value: f64) -> Probability {
        if value.is_nan() || value < 0.0 {
            return Probability { value: 0.0 };
        } else if value > 1.0 {
            return Probability { value: 1.0 };
        }
        Probability { value }
    }

    /*
     * Calculates the absolute difference between two probabilities
     */
    pub fn difference(&self, other: &Probability) -> Probability {
        Probability {
            value: (self.value - other.value).abs(),
        }
    }

    /*
     * Raises a probability to an integer power
     */
    pub fn powi(&self, n: i32) -> Probability {
        Probability {
            value: self.value.powi(n),
        }
    }

    /*
     * Returns the underlying float value of the probability
     */
    pub fn get_value(&self) -> f64 {
        self.value
    }
}

#[derive(Debug, Clone)]
pub struct Preferences {
    preferences: Vec<Probability>,
}

impl Preferences {
    /*
     * Creates a new Preferences object
     * This function will turn each f64 into a Probability
     * see the Probability struct for the relevant conversion rules
     */
    pub fn new(preferences: &[f64]) -> Preferences {
        if preferences.len() == 0 {
            return Preferences {
                preferences: Vec::new(),
            };
        }

        // Convert the floats to Probabilities and return the Preferences
        Preferences {
            preferences: preferences.iter().map(|p| Probability::new(*p)).collect(),
        }
    }

    /*
     * Creates a new Preferences object from an iterator
     * This function will turn each f64 into a Probability
     * see the Probability struct for the relevant conversion rules
     */
    pub fn from_iter<I>(preferences: I) -> Preferences
    where
        I: Iterator<Item = f64>,
    {
        // Convert the floats to Probabilities and return the Preferences
        Preferences {
            preferences: preferences.map(|p| Probability::new(p)).collect(),
        }
    }

    /*
     * Creates a new Preferences object where all probabilities sum up to 1.0,
     * unless the given values sum to 0.
     * This function will turn each f64 into a Probability
     * see the Probability struct for the relevant conversion rules
     * This function will snap all negative floats, NaN, and infinite to 0.0
     */
    pub fn new_normalize<T>(preferences: &[T]) -> Preferences
    where
        T: Copy + Into<f64>,
    {
        if preferences.len() == 0 {
            return Preferences {
                preferences: Vec::new(),
            };
        }

        // Convert <0.0, NaN, and infinite values to 0.0
        let preferences: Vec<f64> = preferences
            .iter()
            .map(|value| {
                let value: f64 = (*value).into();
                if value < 0.0 || value.is_infinite() || value.is_nan() {
                    0.0
                } else {
                    value
                }
            })
            .collect();

        // Calculate the sum for normalizing
        let total: f64 = preferences.iter().sum();
        if total == 0.0 {
            return Preferences {
                preferences: preferences
                    .iter()
                    .map(|value| Probability::new(*value))
                    .collect(),
            };
        }

        // Convert the floats to Probabilities, normalize, and return the Preferences
        Preferences {
            preferences: preferences
                .iter()
                .map(|p| Probability::new(p / total))
                .collect(),
        }
    }

    /*
     * Gets the Probability at index
     */
    pub fn get_preference(&self, index: usize) -> Probability {
        self.preferences[index]
    }

    /*
     * Calculates the distance between two sets of preferences
     * This function requires that both preference vectors are of the same dimension
     */
    pub fn distance(&self, other: &Preferences) -> Result<f64, String> {
        // Check that both Preferences are of the same dimension
        if self.preferences.len() != other.preferences.len() {
            Err(format!(
                "Preference vectors are not of the same dimension: {} != {}",
                self.preferences.len(),
                other.preferences.len()
            ))
        } else {
            // Calculate the Euclidian distance of the Preferences
            let mut distance: f64 = 0.0;
            for i in 0..self.preferences.len() {
                distance += (self.preferences[i].difference(&other.preferences[i]))
                    .powi(2)
                    .get_value();
            }
            Ok(distance.sqrt())
        }
    }

    /*
     * Return a random number in [0, length(self) - 1]
     * taking the preferences as a weighted list of probabilities
     * If preferences is a list [0.1, 0.3, 0.6],
     * then 0 will be returned with probability 0.1,
     * 1 with probability 0.3, and 2 with probability 0.6
     */
    pub fn choose(&self) -> usize {
        let total: f64 = self
            .preferences
            .iter()
            .fold(0.0, |acc, probability| acc + probability.get_value());
        let n = rand::random_range(0.0..=total);

        let mut partial_sum = 0.0;
        for (i, preference) in self.preferences.iter().enumerate() {
            partial_sum += preference.get_value();
            if n <= partial_sum {
                return i;
            }
        }

        // If no preference was returned, returned the last index
        self.preferences.len() - 1
    }

    pub fn len(&self) -> usize {
        self.preferences.len()
    }
}

impl Mul for Preferences {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Preferences::from_iter(
            self.preferences
                .iter()
                .enumerate()
                .map(|(i, lhs)| lhs.value * rhs.preferences[i].value),
        )
    }
}
