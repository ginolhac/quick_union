
## Quick Union

Coursera Algorithms Part 1 - Week 1 - Quick Union

https://www.coursera.org/learn/algorithms-part1/home/module/2

Example of output for `cargo run tinyUF.txt` for Weighted Quick Union with Path Compression:

```bash

length of numbers: 11, max value: 9
index 0, 1st: 4, 2nd: 3
Connected 4 and 3
idx: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
id:  [0, 1, 2, 4, 4, 5, 6, 7, 8, 9]
sz:  [1, 1, 1, 1, 2, 1, 1, 1, 1, 1]
index 1, 1st: 3, 2nd: 8
Connected 3 and 8
idx: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
id:  [0, 1, 2, 4, 4, 5, 6, 7, 4, 9]
sz:  [1, 1, 1, 1, 3, 1, 1, 1, 1, 1]
index 2, 1st: 6, 2nd: 5
Connected 6 and 5
idx: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
id:  [0, 1, 2, 4, 4, 6, 6, 7, 4, 9]
sz:  [1, 1, 1, 1, 3, 1, 2, 1, 1, 1]
index 3, 1st: 9, 2nd: 4
Connected 9 and 4
idx: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
id:  [0, 1, 2, 4, 4, 6, 6, 7, 4, 4]
sz:  [1, 1, 1, 1, 4, 1, 2, 1, 1, 1]
index 4, 1st: 2, 2nd: 1
Connected 2 and 1
idx: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
id:  [0, 2, 2, 4, 4, 6, 6, 7, 4, 4]
sz:  [1, 1, 2, 1, 4, 1, 2, 1, 1, 1]
index 5, 1st: 8, 2nd: 9
8 and 9 are already connected
Connected 8 and 9
idx: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
id:  [0, 2, 2, 4, 4, 6, 6, 7, 4, 4]
sz:  [1, 1, 2, 1, 4, 1, 2, 1, 1, 1]
index 6, 1st: 5, 2nd: 0
Connected 5 and 0
idx: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
id:  [6, 2, 2, 4, 4, 6, 6, 7, 4, 4]
sz:  [1, 1, 2, 1, 4, 1, 3, 1, 1, 1]
index 7, 1st: 7, 2nd: 2
Connected 7 and 2
idx: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
id:  [6, 2, 2, 4, 4, 6, 6, 2, 4, 4]
sz:  [1, 1, 3, 1, 4, 1, 3, 1, 1, 1]
index 8, 1st: 6, 2nd: 1
Connected 6 and 1
idx: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
id:  [6, 2, 6, 4, 4, 6, 6, 2, 4, 4]
sz:  [1, 1, 3, 1, 4, 1, 6, 1, 1, 1]
index 9, 1st: 1, 2nd: 0
1 and 0 are already connected
Connected 1 and 0
idx: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
id:  [6, 6, 6, 4, 4, 6, 6, 2, 4, 4]
sz:  [1, 1, 3, 1, 4, 1, 6, 1, 1, 1]
index 10, 1st: 6, 2nd: 7
6 and 7 are already connected
Connected 6 and 7
idx: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
id:  [6, 6, 6, 4, 4, 6, 6, 6, 4, 4]
sz:  [1, 1, 3, 1, 4, 1, 6, 1, 1, 1]
Final id vector: 
idx: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
id:  [6, 6, 6, 4, 4, 6, 6, 6, 4, 4]
sz:  [1, 1, 3, 1, 4, 1, 6, 1, 1, 1]
```