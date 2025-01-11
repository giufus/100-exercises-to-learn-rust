// Given a number `n`, return the `n+1`th number in the Fibonacci sequence.
//
// The Fibonacci sequence is defined as follows:
//
// - The first number of the sequence is 0.
// - The second number of the sequence is 1.
// - Every subsequent number is the sum of the two preceding numbers.
//
// So the sequence goes: 0, 1, 1, 2, 3, 5, 8, 13, 21, and so on.
//
// We expect `fibonacci(0)` to return `0`, `fibonacci(1)` to return `1`,
// `fibonacci(2)` to return `1`, and so on.
pub fn fibonacci_rec(n: u32) -> u32 {
    
    if n < 2 {
        return n;
    }
    
    fibonacci_rec(n-1) + fibonacci_rec(n-2)
}

pub fn fibonacci(n: u32) -> u32 {
    let mut serie: Vec<u32> = Vec::with_capacity(2);
    for i in 0..=n {
        if i < 2 {
            serie.push(i);
        } else {
            serie.push(serie[i as usize - 2] + serie[i as usize - 1]);
        }
    }
    serie[n as usize]
}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirtieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
