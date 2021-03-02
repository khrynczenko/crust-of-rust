#[macro_export]
macro_rules! avec {
    () => {
        Vec::new()
    };
    ($($v:expr),+ $(,)*) => {
        {
            const C: usize = $crate::count![@COUNT; $($v),*];
            let mut vec = Vec::with_capacity(C);
            $(vec.push($v);)+
            vec
        }
    };
    ($v:expr; $count:expr) => {
        {
            let mut vec = Vec::with_capacity($count);
            let x = $v;
            vec.extend(std::iter::repeat(x).take($count));
            //for _ in 0..$count {
                //vec.push(x.clone());
            //}
            vec
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! count  {
    (@COUNT; $($element:expr),*) => {
        [$($crate::count![@SUBST; $element]),*].len()
    };
    (@SUBST; $element:expr) => {
        ()
    }
}

#[test]
fn empty_vec() {
    let x: Vec<u32> = avec![];
    assert!(x.is_empty());
}

#[test]
fn single() {
    let x: Vec<u32> = avec![42];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 42);
}

#[test]
fn double() {
    let x: Vec<u32> = avec![42, 43];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 43);
}

#[test]
fn trailing_comma() {
    let x: Vec<u32> = avec![1, 2, 3, 4, 5, 6, 7, 8, 9,];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 9);
}

#[test]
fn reserve() {
    let x: Vec<u32> = avec![1; 43];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 43);
}
