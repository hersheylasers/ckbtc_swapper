import type { ActorMethod } from '@dfinity/agent';

export interface Balances {
	btc: bigint;
	ckbtc: bigint;
}
export type BalanceResult = { Ok: Balances } | { Err: string };
export interface ConversionRecord {
	timestamp: bigint;
	from_network: PreferredNetwork;
	to_network: PreferredNetwork;
	amount: bigint;
	status: ConversionStatus;
}
export type ConversionStatus = { Pending: null } | { Complete: null } | { Failed: string };
export type PreferredNetwork = { Bitcoin: null } | { CkBTC: null };
export type Result = { Ok: null } | { Err: string };

export interface _SERVICE {
	set_network_preference: ActorMethod<[PreferredNetwork], Result>;
	get_network_preference: ActorMethod<[], PreferredNetwork>;
	get_balances: ActorMethod<[], BalanceResult>;
	check_and_convert: ActorMethod<[], Result>;
	get_conversion_history: ActorMethod<[], Array<ConversionRecord>>;
}
