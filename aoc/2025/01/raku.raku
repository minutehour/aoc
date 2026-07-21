#!/usr/bin/env raku

# mmm... symbol soup
my @ps=produce {(my $x=$^a%1+$^b).abs.Int+($^a%1&&0>=$x)+$x%1},(.5,|lines>>.&{TR/LR/- //100});
say "silver: ", @ps>>.&{!($_%1)}.sum;
say "gold: ", @ps>>.Int.sum;
