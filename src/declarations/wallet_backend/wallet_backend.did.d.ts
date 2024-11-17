import type { IDL } from '@dfinity/candid';

export type PreferredNetwork = { Bitcoin: null } | { CkBTC: null };
export type ConversionStatus = { Pending: null } | { Complete: null } | { Failed: string };
export interface ConversionRecord {
	timestamp: bigint;
	from_network: PreferredNetwork;
	to_network: PreferredNetwork;
	amount: bigint;
	status: ConversionStatus;
}

export interface _SERVICE {
	set_network_preference: (arg_0: PreferredNetwork) => Promise<{ Ok: null } | { Err: string }>;
	get_network_preference: () => Promise<PreferredNetwork>;
	get_balances: () => Promise<{ Ok: { btc: bigint; ckbtc: bigint } } | { Err: string }>;
	check_and_convert: () => Promise<{ Ok: null } | { Err: string }>;
	get_conversion_history: () => Promise<Array<ConversionRecord>>;
}

export declare const idlFactory: IDL.InterfaceFactory;
