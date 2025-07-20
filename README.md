# Groth16 zkSNARK implementation

## TODOs

- [x] Understand the `Diffie-Hellman Protocol`.

- [ ] Understand `Tate pairings` in detail.

- [ ] Understand how `asymmetric BiLinear Pairings` in CryptoGraphy are generated on `non-supersingular curves`.
  And why is the `Bilinear Decision Diffie-Hellman` (BDDH) assumption computationally hard to solve.

- [ ] Why is it inefficient to do modulo and comparison operations on Cyclic Groups.

- [ ] What do we really mean by the multiplication of 2 points on Elliptic curves. And properly understand the math behind how Bilinear pairings help us to do that.

## REFERENCEs

**Mathematical pre-requisites** :

- [Number Theory | Fermat's Little Theorem](https://www.youtube.com/watch?v=OkQJGql8os8)

- [Advanced Linear Algebra 11: Bilinear Forms](https://www.youtube.com/watch?v=q1w7QpVhJOk)

- [The Fast Fourier Transform (FFT): Most Ingenious Algorithm Ever?](https://www.youtube.com/watch?v=h7apO7q16V0)
  > Refer to [Mathematical Toolkit | Lecture 12: Polynomial identity lemma, probabilistic method](https://www.youtube.com/watch?v=nzViMRFIa5s) for the proof of `Schwartz-Zippel Lemma`.

- [FFT Example: Unraveling the Recursion](https://www.youtube.com/watch?v=Ty0JcR6Dvis)

**Diffie-Hellman Protocol** :

- [Lecture 13: Diffie-Hellman Key Exchange and the Discrete Log Problem by Christof Paar](https://www.youtube.com/watch?v=aeOzBCbwxUo)

- [Lecture 14: The Generalized Discrete Log Problem and the Security of Diffie-Hellman by Christof Paar](https://www.youtube.com/watch?v=IGqrbM52wtg)

**Bilinear Pairings** :

- [Pairings in Cryptography](https://www.youtube.com/watch?v=8WDOpzxpnTE)

- [Cryptography 101 for Blockchain Developers Part 3/3: Elliptic Curve Pairings](https://www.youtube.com/watch?v=9TFEBuANioo)

Problem statement to **Rank 1 Constraint System (R1CS)** to **Quadratic Arithmetic Program (QAP)** :

- [Zk-SNARK - Part 1 - Problem statement to R1CS](https://www.youtube.com/watch?v=bqSFyULJFtQ)

- [Zk-SNARK part 2 - R1CS to QAP](https://www.youtube.com/watch?v=T2wlGhVFOCw)

- [Quadratic Arithmetic Programs: from Zero to Hero](https://medium.com/@VitalikButerin/quadratic-arithmetic-programs-from-zero-to-hero-f6d558cea649)

**Multi-Party Computation (MPC)** :

- [Multi-Party Computation simplified: Ivan Damgård, Co-founder/Chief Cryptographer-Partisia Blockchain](https://www.youtube.com/watch?v=vRVudJADQLk)

Approaching **Groth 16** :

- [ZK HACK - How to Make SNARKs - Alessandro Chiesa](https://www.youtube.com/watch?v=KjkIQLJk4eQ)

- [M2S4: Groth16 with Guest speaker Ying Tong](https://www.youtube.com/watch?v=Hz_XHfxunck)

- [Powers of Tau | Solidity Fridays](https://www.youtube.com/watch?v=TcQXzGTSXfo)
  > Recording of 39th power of tau generation : [Semaphore Perpetual Powers of Tau #39](https://www.youtube.com/watch?v=wZaqiTldLuk)

- [Groth16](https://alinush.github.io/groth16) explained by Alin Tomescu.

- [An overview of the Groth16 proof system](https://blog.lambdaclass.com/groth16/)

- [Groth16 Explained](https://rareskills.io/post/groth16?utm_source=chatgpt.com) by RareSkills.

- [Groth16](https://xn--2-umb.com/22/groth16/) explained by Remco Bloemen.

## Digging Deeper

- [The 3rd Bar-Ilan Winter School on Cryptography: Bilinear Pairings in Cryptography](https://www.youtube.com/playlist?list=PL8Vt-7cSFnw2V2Wpf4MpwtSJvLvZo1ADB)

- [Proofs, Arguments, and Zero-Knowledge - Study Group](https://www.youtube.com/playlist?list=PLTPK8HRi5qmlIBA7TDTO8hBOprAc1FIQv)

- [On the Size of Pairing-based Non-interactive Arguments](https://eprint.iacr.org/2016/260.pdf)

- [Powers-of-Tau to the People: Decentralizing Setup Ceremonies](https://eprint.iacr.org/2022/1592.pdf)
