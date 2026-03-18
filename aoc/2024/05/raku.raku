#!/usr/bin/env raku

my (@rs, @us) := slurp.split("\n\n")>>.lines;

sub mid(@u) {
	my @s = @u.sort({ "$^a|$^b" !(elem) @rs });
	return (@u Z== @s).all.so ?? ($_, 0) !! (0, $_) with @s[@s/2];
}

(("silver: ", "gold: ") Z~ [Z+] @us>>.split(",").map(&mid)).map(&say);
