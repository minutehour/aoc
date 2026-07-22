#!/usr/bin/env raku


my @m = lines.map(&{$_.comb.Array});

sub get(($x, $y)) { $x < 0 || $y < 0 ?? Nil !! @m[$x;$y] }
sub rem(($x, $y)) { @m[$x;$y] = "." }
sub roll($xy) { get($xy) ~~ "@" }
sub ns($xy) { [X<<+>>] ([X] (-1..1) xx 2).grep(none (<0 0>,)), ($xy,) }
sub access($xy) {
	my $valid = ns($xy).map(&get).Bag{"@"} < 4;
	return $valid;
}

my @idxs = ((0...@m.elems) X (0...@m.first.elems)).grep(&roll).grep(&access);
say "silver: ", @idxs.elems;

my $gold = 0;
while @idxs {
	my $i = @idxs.pop();
	if roll($i) and access($i) {
		rem($i);
		@idxs.push(|ns($i));
		$gold++;
	}
}
say "gold: ", $gold;
