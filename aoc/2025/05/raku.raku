#!/usr/bin/env raku

my @rs = (lines() ...^ "").map(&{ Range.new(|$_.split("-")>>.Int) });
my @ids = lines;

sub infix:<U> ($r, $s) { $r.min .. max($r.max,$s.max) }
sub infix:<N> ($r, $s) { $r.max >= $s.min }

multi merge() { () }
multi merge($r is copy, +@rs) {
	gather {
		for merge(|@rs) -> $s {
			($r N $s) ?? ($r U= $s) !! take $s
		}
		take $r;
	}
}

say "silver: ", @ids.grep(any @rs).elems;
say "gold: ", merge(|@rs.sort)>>.elems.sum;
