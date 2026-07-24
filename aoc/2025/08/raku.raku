#!/usr/bin/env raku

my @c = lines.map(&{ $_.comb(/\w+/).Array });
my %j = (0..^@c)>>.Set.pairs;

sub dist (($x, $y)) { ([Z-] @c[$x, $y])>>.&{$_**2}.sum }
my @ps = ([X(|)] (0..^@c) xx 2).unique.grep(* == 2)>>.keys.map(&cache).sort(&dist);

for @ps -> @p {
	my $m = [(|)] %j{@p};
	$m.keys.map(&{ %j{$_} := $m });
	if (++$ == (@c < 100 ?? 10 !! 1000)) { say "silver: ", [*] %j.values.unique.sort(&{-$_})[^3]}
	last if $m == @c;
	LAST { say "gold: ", [*] @c[@p][*;0] }
}
