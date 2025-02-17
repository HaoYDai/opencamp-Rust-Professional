/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn partition<T: Ord>(array: &mut [T]) -> usize {
    let len = array.len();
    let pivot = len - 1;
    let mut store_index = 0;
    
    // 将小于pivot的元素移到左边
    for i in 0..len-1 {
        if array[i] <= array[pivot] {
            array.swap(i, store_index);
            store_index += 1;
        }
    }
    
    // 将pivot放到最终位置
    array.swap(pivot, store_index);
    store_index
}

fn sort<T: Ord>(array: &mut [T]) {
    if array.len() <= 1 {
        return;
    }
    
    let pivot = partition(array);
    let (left, right) = array.split_at_mut(pivot);
    
    sort(left);
    sort(&mut right[1..]); // 跳过pivot元素
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