//! An interface for dealing with the kinds of parallel computations involved in
//! `halo2`. It's currently just a (very!) thin wrapper around [`rayon`] but may
//! be extended in the future to allow for various parallelism strategies.

//<<<<<<< HEAD
//pub use rayon::{current_num_threads, scope, Scope, join};
//=======
pub use rayon::{
    current_num_threads,
    iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator},
    join, scope, Scope,
};
//>>>>>>> 6ae9f77e04d471c64b31b86486fb6ae974dc31a1
