/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
// I AM NOT DONE

fn sort<T>(array: &mut [T]){
	//TODO
    let len = array.len();  
    for i in 1..len {  
        let key = array[i];  
        let j = i as isize;  
        let mut i = (j - 1) as usize;  
  
        // Move elements of array[0..i-1], that are greater than key,  
        // to one position ahead of their current position  
        while i > 0 && array[i as isize as usize] > key {  
            array[(i as isize + 1) as usize] = array[i as usize];  
            i -= 1;  
        }  
        array[(i as isize + 1) as usize] = key;  
    }  
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