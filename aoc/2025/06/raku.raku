#!/usr/bin/env raku

sub infix:<C> ($o, @n) { reduce CORE::{"&infix:<$o>"}, @n }
sub snip(@xs, $p) {
	gather {
		my @chunk;
		for @xs -> $x {
			($x ~~ $p) ?? (take @chunk; @chunk = ()) !! @chunk.push($x);
		}
		take @chunk;
	}
}

my @in = lines;
my ($os, @m) = @in.map(&{$_.comb.Array}).reverse;
my @os = $os.words;

say "silver: ", (@os ZC ([Z] @in>>.words).map(&{$_[0..^*-1]})).sum;
say "gold: ", (@os ZC snip(([Z] @m.reverse).map(&{ [~] $_.grep(none " ") }), "")).sum;
