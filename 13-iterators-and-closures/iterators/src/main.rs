// iterators are lazy - they have no effect until methods consume them

fn main(){
    let v1: Vec<i32> = vec![1, 2, 3];

    // collect consumes the iterator and collects it into an appropriate type
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}