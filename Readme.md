
  # Merge Sort in Rust

  This is an implementation of the Merge Sort algorithm in Rust. Merge Sort is an efficient, stable, and comparison-based sorting algorithm known for its performance.

  ## Usage

  To use the `sort` function provided in this module, simply call it with a mutable reference to a vector of comparable elements.

  ```rust
  use prksort::*;
  
  let mut arr = vec![5, 2, 3, 1, 4];
  mergesort::sort(&mut arr);
  println!("Sorted array: {:?}", arr); // Output: Sorted array: [1, 2, 3, 4, 5]
  ```

  This will sort the elements of the vector in ascending order.

  The `sort` function works with any type `T` that implements the `PartialEq`, `PartialOrd`, and `Copy` traits. This means that the elements of the vector must support comparisons and be copyable.

  ```rust
  use prksort::*;
  
  let mut arr = vec!['e', 'b', 'c', 'a', 'd'];
  mergesort::sort(&mut arr);
  println!("Sorted array: {:?}", arr); // Output: Sorted array: ['a', 'b', 'c', 'd', 'e']
  ```

  ## Functionality

  The `sort` function sorts the input vector in-place using the Merge Sort algorithm.

  The `merge` function is a helper function used by the `sort` function to merge two sorted slices into one sorted slice.

  ## Examples

  To run the examples provided, clone the repository and execute the following command:

  ```
    cargo run --example basic_sort
    ```

      This will run the basic example provided in the `examples` directory, sorting a vector of integers.

      ## License

      This code is provided under the MIT License. You can find the full license text in the LICENSE file.

      ## Contribution

      Contributions are welcome! If you find any issues or have suggestions for improvements, feel free to open an issue or create a pull request on GitHub.

     
   

# Quick Sort in Rust

This is an implementation of the Quick Sort algorithm in Rust. Quick Sort is a widely used sorting algorithm known for its efficiency and simplicity.

## Usage

To use the `sort` function provided in this module, simply call it with a mutable reference to an array or slice of comparable elements. 

```rust
use prksort::*;

let mut arr = vec![5, 2, 3, 1, 4];
quicksort::sort(&mut arr);
println!("Sorted array: {:?}", arr); // Output: Sorted array: [1, 2, 3, 4, 5]
```
