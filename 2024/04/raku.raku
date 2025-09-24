#!/usr/bin/env raku

my @g = lines.map: *.comb.Array;

@g.map: *.say;
say "---";

say @g[.[0]; .[1] .. .[1]+3] for (^@g X ^@g[0])[^4];
