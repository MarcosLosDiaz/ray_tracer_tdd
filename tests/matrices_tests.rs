use approx::assert_relative_eq;
use cucumber::{World, given, then};
use std::collections::HashMap;
use ray_tracer_tdd::primitives;
use nalgebra::DMatrix;

#[derive(Debug, Default, World)]
pub struct MatrixWorld {
    matrices: HashMap<String, DMatrix<f64>>,
}

fn parse_matrix_table(table: &cucumber::gherkin::Table) -> Vec<f64> {
    let mut data = Vec::new();
    for row in table.rows.iter() {
        for val in row.iter() {
            data.push(val.parse::<f64>().expect("Invalid number in matrix"));
        }
    }
    data
}

#[given(regex = r"^the following \d+x\d+ matrix (\w+):$")]
#[given(expr = "the following matrix {word}:")]
fn given_matrix(world: &mut MatrixWorld, name: String, step: &cucumber::gherkin::Step) {
    let table = step.table.as_ref().expect("Expected data table");
    let data = parse_matrix_table(table);
    let matrix = DMatrix::from_row_slice(table.rows.len(), table.rows[0].len(), &data);
    world.matrices.insert(name, matrix);
}

#[then(expr = "{word}[{int},{int}] = {float}")]
fn check_matrix_value(world: &mut MatrixWorld, name: String, row: usize, col: usize, expected: f64) {
    let matrix = world.matrices.get(&name).expect("Matrix not found");
    let actual = matrix[(row, col)];
    assert!(primitives::float_are_equal(actual, expected));
}

#[then(regex = r"^(\w+) = (\w+)$")]
fn check_matrix_equality(world: &mut MatrixWorld, name1: String, name2: String) {
    let m1 = world.matrices.get(&name1).expect("Matrix 1 not found");
    let m2 = world.matrices.get(&name2).expect("Matrix 2 not found");
    assert_relative_eq!(m1, m2);
}

#[then(expr = "{word} != {word}")]
fn check_matrix_inequality(world: &mut MatrixWorld, name1: String, name2: String) {
    let m1 = world.matrices.get(&name1).expect("Matrix 1 not found");
    let m2 = world.matrices.get(&name2).expect("Matrix 2 not found");
    assert_ne!(m1, m2);
}

#[then(expr = "{word} * {word} is the following 4x4 matrix:")]
fn check_matrix_multiplication(world: &mut MatrixWorld, name1: String, name2: String, step: &cucumber::gherkin::Step) {
    let m1 = world.matrices.get(&name1).expect("Matrix 1 not found");
    let m2 = world.matrices.get(&name2).expect("Matrix 2 not found");    
    let table = step.table.as_ref().expect("Expected data table");

    let result = m1 * m2;
    let data = parse_matrix_table(table);
    let expected = DMatrix::from_row_slice(table.rows.len(), table.rows[0].len(), &data);
    assert_relative_eq!(result, expected);
}

#[given(expr = "{word} ← tuple\\({float}, {float}, {float}, {float}\\)")]
fn given_tuple(world: &mut MatrixWorld, name: String, x: f64, y: f64, z: f64, w: f64) {
    let matrix = DMatrix::from_row_slice(4, 1, &[x, y, z, w]);
    world.matrices.insert(name, matrix);
}

#[then(expr = "{word} * {word} = tuple\\({float}, {float}, {float}, {float}\\)")]
fn check_matrix_tuple_multiplication(world: &mut MatrixWorld, matrix_name: String, tuple_name: String, x: f64, y: f64, z: f64, w: f64) {
    let matrix = world.matrices.get(&matrix_name).expect("Matrix not found");
    let tuple = world.matrices.get(&tuple_name).expect("Tuple not found");
    
    let result = matrix * tuple;
    let expected = DMatrix::from_row_slice(4, 1, &[x, y, z, w]);
    assert_relative_eq!(result, expected);
}

fn add_identity_matrix(world: &mut MatrixWorld, size: usize) {
    let identity = DMatrix::identity(size, size);
    world.matrices.insert("identity_matrix".to_string(), identity);
}

#[given(expr = "identity_matrix ← an identity matrix")]
fn create_identity_matrix(world: &mut MatrixWorld) {
    let size = world.matrices.values().next().expect("No matrices found").nrows();
    add_identity_matrix(world, size);
}

#[then(expr = "identity_matrix * {word} = {word}")]
#[then(expr = "{word} * identity_matrix = {word}")]
fn check_matrix_mult_identity(world: &mut MatrixWorld, matrix_name: String, expected_name: String) {
    assert_eq!(matrix_name, expected_name);
    let matrix = world.matrices.get(&matrix_name).expect("Matrix not found");
    let identity = DMatrix::identity(matrix.ncols(), matrix.ncols());
    let result = matrix * identity;
    assert_relative_eq!(result, *matrix);
}

#[then(expr = "transpose\\({word}\\) is the following matrix:")]
fn check_matrix_transpose(world: &mut MatrixWorld, matrix_name: String, step: &cucumber::gherkin::Step) {
    let matrix = world.matrices.get(&matrix_name).expect("Matrix not found");
    let table = step.table.as_ref().unwrap();
    let data = parse_matrix_table(table);
    let expected_matrix = DMatrix::from_row_slice(table.rows.len(), table.rows[0].len(), &data);

    let transposed_matrix = matrix.transpose();

    assert_relative_eq!(transposed_matrix, expected_matrix);
}

#[then(expr = "determinant\\({word}\\) = {float}")]
fn check_matrix_determinant(world: &mut MatrixWorld, matrix_name: String, expected_determinant: f64) {
    let matrix = world.matrices.get(&matrix_name).expect("Matrix not found");
    let determinant = matrix.determinant();
    assert!(primitives::float_are_equal(determinant, expected_determinant));
}

#[then(expr = "{word} is invertible")]
fn check_matrix_invertible(world: &mut MatrixWorld, matrix_name: String) {
    let matrix = world.matrices.get(&matrix_name).expect("Matrix not found");
    assert!(!primitives::float_are_equal(matrix.determinant(), 0.0));
}

#[then(expr = "{word} is not invertible")]
fn check_matrix_not_invertible(world: &mut MatrixWorld, matrix_name: String) {
    let matrix = world.matrices.get(&matrix_name).expect("Matrix not found");
    assert!(primitives::float_are_equal(matrix.determinant(), 0.0));
}

#[then(expr = "inverse\\({word}\\) is the following 4x4 matrix:")]
fn check_matrix_inverse(world: &mut MatrixWorld, matrix_name: String, step: &cucumber::gherkin::Step) {
    let matrix = world.matrices.get(&matrix_name).expect("Matrix not found");
    let table = step.table.as_ref().expect("Expected data table");
    let data = parse_matrix_table(table);
    let expected = DMatrix::from_row_slice(table.rows.len(), table.rows[0].len(), &data);
    let inverse = matrix.clone().try_inverse().expect("Matrix is not invertible");
    assert_relative_eq!(expected, inverse, epsilon = 1e-3);
}

#[given(expr = "{word} ← {word} * {word}")]
fn assign_matrix_product(world: &mut MatrixWorld, result_name: String, a_name: String, b_name: String) {
    let a = world.matrices.get(&a_name).expect("Matrix A not found");
    let b = world.matrices.get(&b_name).expect("Matrix B not found");
    let result = a * b;
    world.matrices.insert(result_name, result);
}

#[then(expr = "{word} * inverse\\({word}\\) = {word}")]
fn check_product_inverse_equality(world: &mut MatrixWorld, c_name: String, b_name: String, a_name: String) {
    let c = world.matrices.get(&c_name).expect("Matrix C not found");
    let b = world.matrices.get(&b_name).expect("Matrix B not found");
    let a = world.matrices.get(&a_name).expect("Matrix A not found");
    
    let b_inv = b.clone().try_inverse().expect("Matrix B is not invertible");
    let result = c * b_inv;
    assert_relative_eq!(a, &result, epsilon = 1e-3);
}

fn main() {
    futures::executor::block_on(MatrixWorld::run("tests/features/matrices.feature"));
}