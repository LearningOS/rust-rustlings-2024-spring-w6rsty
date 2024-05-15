/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn partition<T: Ord + Clone>(arr: &mut [T]) -> usize {
    let pivot = arr[0].clone();
    let mut l = 1;
    let mut r = arr.len() - 1;
    while l <= r {
        if arr[l] > pivot && arr[r] < pivot {
            arr.swap(l, r);
            l += 1;
            r -= 1;
        } else {
            if arr[l] <= pivot {
                l += 1;
            } else {
                r -= 1;
            }
        }
    }
    arr.swap(0, r);
    r
}

fn sort<T: Ord + Clone>(array: &mut [T]){
    let length =  array.len();
    if length <= 1 {
        return;
    }
    let p = partition(array);
    sort(&mut array[0..p]);
    sort(&mut array[p + 1..length]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}