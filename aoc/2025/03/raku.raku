#!/usr/bin/env raku

multi f(Mu, 0 --> "") {}
multi f(@xs, $n) of Str {
	my ($a, $b) = @xs[0..*-$n, *-$n^..*];
	my ($i, $x) = $a.first($a.max, :kv);
	my @p = (|$a[$i^..*], |$b);
	$x ~ f(@p, $n-1);
}

my $js = lines>>.comb.cache;
say "silver: ", $js.map(&f.assuming(*, 2)).sum;
say "gold: ", $js.map(&f.assuming(*, 12)).sum;
