export const idlFactory = ({ IDL }) => {
	const Network = IDL.Variant({
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
		amount: IDL.Nat64,
		fromNetwork: Network,
		toNetwork: Network,
		status: ConversionStatus
	});

	const Result = IDL.Variant({
		Ok: IDL.Tuple(IDL.Nat64, IDL.Nat64),
		Err: IDL.Text
	});

	return IDL.Service({
		get_preferred_network: IDL.Func([], [Network], ['query']),
		set_preferred_network: IDL.Func([Network], [], []),
		get_balances: IDL.Func([], [Result], ['query']),
		get_conversion_history: IDL.Func([], [IDL.Vec(ConversionRecord)], ['query'])
	});
};
