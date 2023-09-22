// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { SuiAddress } from '../../../types/common.js';
import type { ObjectOwner } from './objects.js';

// event types mirror those in "sui-json-rpc-types/src/sui_event.rs"
export type SuiEvent = {
	id: EventId;
	// Move package where this event was emitted.
	packageId: string;
	// Move module where this event was emitted.
	transactionModule: string;
	// Sender's Sui address.
	sender: string;
	// Move event type.
	type: string;
	// Parsed json value of the event
	parsedJson?: Record<string, any>;
	// Base 58 encoded bcs bytes of the move event
	bcs?: string;
	timestampMs?: string;
};

export type EventId = {
	txDigest: string;
	eventSeq: string;
};

export type PaginatedEvents = {
	data: SuiEvent[];
	nextCursor: EventId | null;
	hasNextPage: boolean;
};

export type BalanceChange = {
	owner: ObjectOwner;
	coinType: string;
	/* Coin balance change(positive means receive, negative means send) */
	amount: string;
};

export type SuiObjectChangePublished = {
	type: 'published';
	packageId: string;
	version: string;
	digest: string;
	modules: string[];
};

export type SuiObjectChangeTransferred = {
	type: 'transferred';
	sender: string;
	recipient: ObjectOwner;
	objectType: string;
	objectId: string;
	version: string;
	digest: string;
};

export type SuiObjectChangeMutated = {
	type: 'mutated';
	sender: string;
	owner: ObjectOwner;
	objectType: string;
	objectId: string;
	version: string;
	previousVersion: string;
	digest: string;
};

export type SuiObjectChangeDeleted = {
	type: 'deleted';
	sender: string;
	objectType: string;
	objectId: string;
	version: string;
};

export type SuiObjectChangeWrapped = {
	type: 'wrapped';
	sender: string;
	objectType: string;
	objectId: string;
	version: string;
};

export type SuiObjectChangeCreated = {
	type: 'created';
	sender: string;
	owner: ObjectOwner;
	objectType: string;
	objectId: string;
	version: string;
	digest: string;
};

export type SuiObjectChange =
	| SuiObjectChangePublished
	| SuiObjectChangeTransferred
	| SuiObjectChangeMutated
	| SuiObjectChangeDeleted
	| SuiObjectChangeWrapped
	| SuiObjectChangeCreated;

export type ProposalRecord = {
	pid: number;
	proposal_uid: SuiAddress;
	proposer: SuiAddress;
	start_time: number;
	end_time: number;
	for_votes: number;
	against_votes: number;
	eta: number;
	action_delay: number;
	quorum_votes: number;
	action: {
		action_id: number;
		name: string;
	};
	version_id: number;
};

export enum ProposalStatus {
	Pending = 1,
	Active = 2,
	Defeat = 3,
	Agree = 4,
	Queued = 5,
	Executable = 6,
	Extracted = 7,
}

export type ProposalRecordWithStatus = ProposalRecord & {
	status: ProposalStatus;
};

export type ObcDao = {
	id: {
		id: SuiAddress;
	};
	admin: SuiAddress;
	config: {
		voting_delay: number;
		voting_period: number;
		voting_quorum_rate: number;
		min_action_delay: number;
	};
	info: {
		id: {
			id: SuiAddress;
		};
		next_proposal_id: number;
		next_action_id: number;
		proposal_create_event: {
			proposal_id: number;
			proposer: SuiAddress;
		};
		vote_changed_event: {
			proposal_id: number;
			voter: SuiAddress;
			proposer: SuiAddress;
			agree: boolean;
			vote: number;
		};
	};
	proposal_record: ProposalRecord[];
	action_record: Record<
		string,
		{
			action_id: number;
			name: string;
		}
	>;
	votes_record: Record<string, string>;
	voting_pool: {
		id: {
			id: SuiAddress;
		};
		obc_balance: number;
		pool_token_balance: number;
	};
	current_proposal_status: Record<
		string,
		{
			version_id: number;
			status: ProposalStatus;
		}
	>;
};

export type VotingObc = {
	id: {
		id: SuiAddress;
	};
	pool_id: SuiAddress;
	principal: string;
};
