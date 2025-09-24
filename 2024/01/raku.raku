#!/usr/bin/env raku

my $ids := ([Z] lines>>.words)>>.sort>>.list;
say "silver: ", ([Z-] $ids)>>.abs.sum;
say "gold: ", ([<<*>>] $ids>>.Bag).kxxv.sum;
