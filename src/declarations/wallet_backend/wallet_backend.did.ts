export const idlFactory = ({ IDL }: { IDL: typeof IDL }) => {
	const PreferredNetwork = IDL.Variant({
		Bitcoin: IDL.Null,
		CkBTC: IDL.Null
	});

	const ConversionStatus = IDL.Variant({
		Pending: IDL.Null,
		Complete: IDL.Null,
		Failed: IDL.Text
	});

	const ConversionRecord = IDL.Record({
		timestamp: IDL.Nat64,
		from_network: PreferredNetwork,
		to_network: PreferredNetwork,
		amount: IDL.Nat64,
		status: ConversionStatus
	});

	const Result = IDL.Variant({
		Ok: IDL.Null,
		Err: IDL.Text
	});

	const Balances = IDL.Record({
		btc: IDL.Nat64,
		ckbtc: IDL.Nat64
	});

	const BalanceResult = IDL.Variant({
		Ok: Balances,
		Err: IDL.Text
	});

	return IDL.Service({
		set_network_preference: IDL.Func([PreferredNetwork], [Result], []),
		get_network_preference: IDL.Func([], [PreferredNetwork], ['query']),
		get_balances: IDL.Func([], [BalanceResult], []),
		check_and_convert: IDL.Func([], [Result], []),
		get_conversion_history: IDL.Func([], [IDL.Vec(ConversionRecord)], ['query'])
	});
};
