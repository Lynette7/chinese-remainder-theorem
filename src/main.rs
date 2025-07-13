// Implementation of the Extended Euclidean Algorithm used to find the modular multiplicative inverse of  M_i
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        return (b, 0, 1);
    }

    let (gcd, x1, y1) = extended_gcd(b % a, a);

    let x = y1 - (b / a) * x1;
    let y = x1;

    (gcd, x, y)
}

// Function to find the modular multiplicative inverse of'a' modulo 'm', which only exists if 'a' and 'm' are coprime
fn mod_inverse(a: i64, m: i64) -> Option<i64> {
    let (gcd, x, _y) = extended_gcd(a, m);

    if gcd != 1 {
        None
    } else {
        Some((x % m + m) % m)
    }
}

// The Chinese Remainder Theorem that solves a system of simulateneous congruences
// It returns Some(solution) if a solution is found, or None if any modular inverse cannot be computed (which implies moduli are not pairwise coprime).
fn chinese_remainder_theorem(remainders: &[i64], moduli: &[i64]) -> Option<i64> {
    // Find M, the product of all moduli
    let m_product: i64 = moduli.iter().product();

    let mut result: i64 = 0;

    // Iterate through each congruence equation to compute the sum of terms
    for i in 0..moduli.len() {
        let a_i = remainders[i];
        let m_i = moduli[i];

        // Find M_i = M / m_i
        let m_product_i = m_product / m_i;

        // Find M_i inverse, which is teh modular multiplicative inverse of M_i mod m_i
        let m_product_i_inverse = match mod_inverse(m_product_i, m_i) {
            Some(inverse) => inverse,
            None => return None,
        };

        let term = a_i.checked_mul(m_product_i)?.checked_mul(m_product_i_inverse)?;
        result = result.checked_add(term)?;
    }

    Some(result % m_product)
}

fn main() {
    // Example 1 from Neso Academy: Find x such that:
    // x ≡ 2 (mod 3)
    // x ≡ 3 (mod 5)
    // x ≡ 2 (mod 7)
    let remainders = vec![2, 3, 2];
    let moduli = vec![3, 5, 7];

    println!("Solving congruence equations:");
    for i in 0..remainders.len() {
        println!("x ≡ {} (mod {})", remainders[i], moduli[i]);
    }

    match chinese_remainder_theorem(&remainders, &moduli) {
        Some(solution) => {
            println!("\nThe solution is: {}", solution);
        }
        None => {
            println!("Could not find a solution. The moduli might not be pairwise coprime.");
        }
    }
}
