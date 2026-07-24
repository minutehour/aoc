#!/usr/bin/env raku

my @rs = lines.map: *.words.Array;

my &dec = &{ .rotor(2=>-1).map({ ([-] $_) (elem) (1..3)}).all };
my &safe = &{ .&dec || .reverse.&dec };

say "silver: ", +@rs.grep(&safe);
say "gold: ", +@rs.grep({ .combinations($_-1).grep(&safe).any });
