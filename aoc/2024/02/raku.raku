#!/usr/bin/env raku

my @rs = lines>>.words.map(&cache);

sub incr ($r) { $r.rotor(2=>-1).map({ ([-] $_) (elem) (1..3)}).all };
sub safe ($r) { so $r.&incr || $r.reverse.&incr };

say "silver: ", @rs.map(&safe).sum;
say "gold: ", @rs.map({ $_.combinations($_-1).map(&safe).any.so }).sum;
