#[test]
fn generic_struct() {

  #[derive(Debug, PartialEq)]
  // x and y must be the same type.
  struct Point<T> {
    x: T,
    y: T
  }

  impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
      Point {
        x, 
        y
      }
    }
  }

  assert_eq!(Point { x: 1.0, y: 2.0 }, Point::new(1.0, 2.0));

  #[derive(Debug, PartialEq)]
  struct PointTuple<T, E> (T, E);

  impl<T, E> PointTuple<T, E> {
    fn new(x: T, y: E) -> Self {
      Self(x, y)
    }
  }
  assert_eq!(PointTuple('x', 'y'), PointTuple::new('x', 'y'));
  assert_eq!(PointTuple(1, 3.8), PointTuple::new(1, 3.8));
}