{
	local $/ = "%\n";
	open my $file, '<', 'fortunes';
	my @fortunes = <$file>;
	my $fortune = $fortunes[rand @fortunes];
	chomp($fortune);
	print $fortune;
};
