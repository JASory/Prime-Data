# Prime-Data
Computed data for primes

Reference material for data on primes.

- Prime-2048 - List of the first 2048 odd primes along with multiplicative inverses over 2^64 and 2^128 

- SPRP-2-64 - List of base-2 strong pseudoprimes that are greater than 2^64 and  coprime to the first 65 primes. 
- PI-2-64-n - List of primecounts as computed by a single 2-strong fermat test greater than 2^64
- primesearch.rs 

I strongly encourage you to contribute to these lists, especially the PI-2-64-n lists and Pseudoprime lists as these can permit the construction of much faster primality tests. (i.e only two base checks instead of the standard 12, or even the unproven BSPW with a cost of 5 checks). 

How to contribute : first filter composites by the first 65 primes, then perform a single base-2 strong fermat check, add them all up in an interval (ideally below 2^40) and print it as an element of an array, do this for a multiple of the array. To detect pseudoprimes a sum is computed of the array element values and compared against the primecount value computed by Kim Walisch's primecount. (this greatly speeds up the search). It is imperative that the elements are only the sum of numbers passed prime by a 2-SPRP check, do not change the values output by the check. At some additional cost one can check each number with an additional test (15 or 19 or 60 work well) to see if it is a base-2 pseudoprime and print it, however this must not be subtracted from the primecount. This is a useful trick to reduce the amount of searching one has to do with the primecount software. 

A simple file that performs this computation automatically is provided in primesearch.rs. Use number-theory = "0.0.11" or later. It computes the pi count in intervals of 2^33 and blocks of 2^42. Printing the picount is used for both reference and future verification, error detection. 

The reason for the parameter selection is to produce consistency with Jan Feitsma's table of 2-prps which J.A Sory reduced to a much more efficient list of 2-sprps. Trial division by the first 65 primes was chosen as it is roughly the equivalent of a single SPRP test in the fastest case, allowing it to be near optimally applied to all integers in a primality test. This is important because it considerably reduces the number of odd integers that undergo a 2-SPRP test. Unlike Feitsma's table J.A Sory's table is not a true pseudoprime list because it eliminates pseudoprimes that have small factors, these however are quite rare especially over 2^64 and would get eliminated by trial division* in an optimally efficient primality test anyway therefore it still satisfies the goal of permitting a fast primality test while reducing the amount of computation needed to achieve it. 

* note that trial division  here actually means multiplication by a prime inverse which is 3-5 times faster, similarly the complexity is compared not to a naive SPRP check but a Montgomery arithmetic test over 2^n. This should make the claim that "trial division by the first 65 primes is roughly equal to the best case SPRP test" make more sense. In a naive implementations the bound is actually a bit higher. 

