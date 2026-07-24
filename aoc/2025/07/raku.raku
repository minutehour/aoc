#!/usr/bin/env raku

my @m = lines.map(&{$_.comb.Array});

my %memo;
sub f(+@k ($r, $c)) {
	if $r >= @m { return 1 }
	if @m[$r;$c] !~~ "^" { return f($r+1, $c) }
	%memo{~@k} //= f($r, $c-1) + f($r, $c+1)
}

my $p = f(0, @m[0].first("S", :k));
say "silver: ", +%memo;
say "gold: ", $p;
