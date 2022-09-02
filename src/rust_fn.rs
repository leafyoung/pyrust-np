// The rust side functions
// Put it in mod to separate it from the python bindings
// These are just some random operations
// you probably want to do something more meaningful.


use ndarray::{arr1, Array1};
use numpy::ndarray::{ArrayViewD, ArrayViewMutD};
use ordered_float::OrderedFloat;
use rand::Rng;

// If we wanted to do something like this in python
// we probably would want to generate matrices and add them
// together. This can be problematic in terms of memory if working with large
// matrices. And looping is usually painfully slow.
// Rayon could be used here to run the mutation in parallel
// this may be good for huge matrices
pub fn double_and_random_perturbation(x: &mut ArrayViewMutD<'_, f64>, scaling: f64) {
    let mut rng = rand::thread_rng();
    x.iter_mut()
        .for_each(|x| *x = *x * 2. + (rng.gen::<f64>() - 0.5) * scaling);
}

pub fn max_min(x: &ArrayViewD<'_, f64>) -> Array1<f64> {
    if x.len() == 0 {
        return arr1(&[]); // If the array has no elements, return empty array
    }
    let max_val = x
        .iter()
        .map(|a| OrderedFloat(*a))
        .max()
        .expect("Error calculating max value.")
        .0;
    let min_val = x
        .iter()
        .map(|a| OrderedFloat(*a))
        .min()
        .expect("Error calculating min value.")
        .0;
    let result_array = arr1(&[max_val, min_val]);
    result_array
}
