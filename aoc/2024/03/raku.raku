#!/usr/bin/env raku

my $p = slurp;

sub p2 (($on, $tot), ($i, |n)) { 
	given $i {
		when "mul"   { return ($on , $tot + [*] $on, |n.list) }
		when "do"    { return (1   , $tot) }
		when "don't" { return (0   , $tot) }
	}
};

$p ~~ m:g/ mul\((\d ** 1..3)\,(\d ** 1..3)\) /;
say "silver: ", $/.map({ [*] $_.list }).sum;

$p ~~ m:g/ (mul)\((\d ** 1..3)\,(\d ** 1..3)\) | (do)\(\) | (don\'t)\(\) /;
say "gold: ", ([[&p2]] <1 0>, |$/)[1];
