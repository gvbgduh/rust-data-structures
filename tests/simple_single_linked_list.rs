use rust_data_structures::simple_stack::List as SimpleStack;

#[test]
fn test_new_list() {
    let mut list: SimpleStack<u32> = SimpleStack::new();
    println!("{:?}", list);

    // Check empty list pops None
    assert_eq!(list.pop(), None);

    // Populate list
    list.push(1);
    list.push(2);
    list.push(3);

    // Check pop
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));

    // Add more values
    list.push(4);
    list.push(5);

    // Empty the list
    assert_eq!(list.pop(), Some(5));
    assert_eq!(list.pop(), Some(4));

    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);
}