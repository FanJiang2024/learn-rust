// 数组和元组是复合类型（compound Types）；Vec和Map是集合数据类型（Collection Types）
// 数组和元组长度是固定的

// Tuples：内部的元素可以是不同的类型
// Array： 内部的元素必须是相同类型

use std::ptr::{addr_of, addr_of_mut};



#[test]
fn tuple_array_test() {
    // create array
    let arr1 = [1,2,3];
    let arr2 = [6; 9];
    println!("arr1: {:?}, arr2: {:?}. length of arr2: {}", arr1, arr2, arr2.len());

    // Tuples: 固定长度的异构集合, 没有len()方法
    let t1 = ();    // empty tuple, default return value of a function.
    let t2 = (1, 'a', "string");
    println!("t1: {:?}, t2: {:?}, 1st element of t2: {}", t1, t2, t2.0);

}

#[test]
fn tuple_test() {
    let mut tup1 = (1, 'c', "UI");
    tup1.0 = 33;

    let tmp = tup1;     // copy
    assert_ne!(addr_of!(tmp), addr_of_mut!(tup1));

    let mut tup2 = tup1;    // copy
    assert_ne!(addr_of_mut!(tup1), addr_of_mut!(tup2));
    assert_ne!(addr_of_mut!(tup1.2), addr_of_mut!(tup2.2));
    
    let t1 = ("hello", 23);
    let t2 = t1;    // copy
    assert_ne!(addr_of!(t1), addr_of!(t2));
}