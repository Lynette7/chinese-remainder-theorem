# Chinese Remainder Theorem (CRT)

CRT enables efficient computation with large numbers by solving systems of congruent equations with apirwise coprime moduli.

Given a set of different congruent equations as follows:

$$
\begin{cases}
x \equiv a_1 \pmod{m_1} \\
x \equiv a_2 \pmod{m_2} \\
\vdots \\
x \equiv a_n \pmod{m_n}
\end{cases}
$$

where $a_i$ and $m_i$ are integers and $m_i$ are pairwise coprime, the Chinese Remainder Theorem guarantees the existence of a unique solution $x$ modulo $M = \prod_{i=1}^{n} m_i$.

To get the unique solution, CRT requires two steps:

1. Compute the product of all moduli, $M = \prod_{i=1}^{n} m_i$.

2. Compute the product of the modular inverses of all moduli, $M_i = M / m_i$.

3. Compute the solution $x$ as follows:

$$
x = \sum_{i=1}^{n} a_i \cdot M_i \cdot m_i^{-1} \pmod{M}
$$
where $m_i^{-1}$ is the modular inverse of $m_i$ under modulo $M$, or more specifically, $m_i^{-1} \equiv M_i \pmod{m_i}$.

CRT is widely used in Fully Homomorphic Encryption (FHE), zkSNARKs and other cryptographic applications.
