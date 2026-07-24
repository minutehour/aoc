#!/usr/bin/env raku

my @x = ([Z] lines>>.words).map: *.sort.cache;
say "silver: ", ([Z-] @x)>>.abs.sum;
say "gold: ",  ([(.)] @x).kxxv.sum;
