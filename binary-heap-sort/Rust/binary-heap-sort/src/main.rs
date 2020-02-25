fn left(i: usize) -> usize {
    2_usize * i + 1
}

fn right(i: usize) -> usize {
    2_usize * i + 2
}

fn heapify(a: &mut Vec<i32>, i: usize, heap_size: Option<usize>) {
    let l = left(i);
    let r = right(i);

    let bound = heap_size.unwrap_or(a.len()) - 1;
    let mut largest = if l <= bound && a[l] > a[i] { l } else { i };

    if r <= bound && a[r] > a[largest] {
        largest = r;
    }

    if largest != i {
        a.swap(i, largest);
        heapify(a, largest, Some(bound));
    }
}

fn build_heap(a: &mut Vec<i32>) {
    let len = a.len();

    for i in (0..(len / 2)).rev() {
        heapify(a, i, None);
    }
}

fn heap_sort(a: &mut Vec<i32>) {
    build_heap(a);

    let mut heap_size = a.len();

    for i in (1..a.len()).rev() {
        a.swap(0, i);
        heap_size -= 1;
        heapify(a, 0, Some(heap_size));
    }
}

fn main() {
    let mut v = vec![
        23_i32, 12_i32, 123_i32, 214_i32, 32_i32, 5_i32, 2_i32, 657_i32,
    ];
    heap_sort(&mut v);

    println!("{:?}", v);
}
