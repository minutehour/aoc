#!/usr/bin/env raku

my @i = slurp.trans("-,"=>" ").words;

sub solve(:$p2) {
	my $N := @i>>.chars.max;
	sub invalids((:$key, :$value)) { (10**($key-1)...^10**$key) Xx (2...$value) }
	my %factors = (1 ... $N div 2).map(&{ $_ => $p2 ?? $N div $_ !! 2});
	my @r = %factors.flatmap(&invalids)>>.Int.unique.sort;
	my @rs = [\+] (0,|@r);
	sub bs($x, :$ff) {
		my ($l, $r) = 0, @r.elems;
		while $l < $r {
			my $m = ($l + $r) div 2;
			if $x < @r[$m] and !$ff { $r = $m }
			elsif @r[$m] < $x and $ff or !$ff { $l = ++$m }
			else { $r = $m }
		}
		return @rs[$l]
	}
	return @i.rotor(2).map(-> ($l, $r) {bs($r) - bs($l, :ff)}).sum;
}


say "silver: ", solve;
say "gold: ", solve :p2;
