// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/02/Inc8.tst

load Inc8.hdl,
output-file Inc8.out,
compare-to Inc8.cmp,
output-list in%B1.16.1 out%B1.16.1;

set in %B00000000,  // in = 0
eval,
output;

set in %B11111111,  // in = -1
eval,
output;

set in %B00000101,  // in = 5
eval,
output;

set in %B11111011,  // in = -5
eval,
output;
