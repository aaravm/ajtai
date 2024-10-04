# Research-snarkify-the-state
The two research paths of the project:
1. Either using a STARK to generate the inclusion proof in Ethereum.
2. Or use a pq-secure commitment scheme like Ajtai to replace the polynomial one currently used.

This repository is for playing around with Ajtai, and to see how it can be used to securely use Ajtai instead.

# Objectives:
The 3 main requirements of such a scheme:
1. Small proof size.
2. Less time needed to generate proof.
3. Less time needed to generate state commitment.

# Resources:
1. A nice introduction to different types of commitment schemes: [Commitment Schemes](https://medium.com/iovlabs-innovation-stories/commitment-schemes-4f3590be8c5)
2. A nice introduction to KZG and its use-cases: [KZG](https://scroll.io/blog/kzg)
3. More formal definition of commitment schemes: [Cryptographic Primitives](https://nglaeser.github.io/crypto-glossary/Cryptographic-Primitives/commitments/#__tabbed_2_1)
4. A nice video series on lattice-based crypto: [Lattice-based Crypto](https://youtube.com/playlist?list=PLasTV9KvJPBukwWUGoLJHCNG9IPJUA5Sg&feature=shared)
5. Verkle Tree Paper: [Verkle Tree Paper](https://math.mit.edu/research/highschool/primes/materials/2018/Kuszmaul.pdf)
6. Verkle Tree For Ethereum: [Verkle Tree For Ethereum](https://blog.ethereum.org/2021/12/02/verkle-tree-structure)
7. Proving Scheme: [Proving Scheme](https://dankradfeist.de/ethereum/2021/07/27/inner-product-arguments.html)
8. MultiProof Scheme: [MultiProof Scheme](https://dankradfeist.de/ethereum/2021/06/18/pcs-multiproofs.html#evaluating-a-polynomial-in-evaluation-form-on-a-point-outside-the-domain)
9. Few More Documents:
    - [Document 1](https://hackmd.io/@kevaundray/HJOAXULw9)
    - [Document 2](https://hackmd.io/@kevaundray/Skgv758D5)
10. https://newtpqc.org/public/ngoc_khahn_nguyen.pdf
11. https://www.icms.org.uk/sites/default/files/downloads/Workshops/2024/Sep-2024/Ngoc%20Khanh%20Nguyen%20-%20Polynomial%20Commitment.pdf 