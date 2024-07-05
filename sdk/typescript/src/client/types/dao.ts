// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0
export type ProposalRecord = {
	pid: number;
	proposal_uid: string;
	proposer: string;
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
	description: string;
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

export type BfcDao = {
	id: {
		id: string;
	};
	admin: string;
	config: {
		voting_delay: number;
		voting_period: number;
		voting_quorum_rate: number;
		min_action_delay: number;
	};
	info: {
		id: {
			id: string;
		};
		next_proposal_id: number;
		next_action_id: number;
		proposal_create_event: {
			proposal_id: number;
			proposer: string;
		};
		vote_changed_event: {
			proposal_id: number;
			voter: string;
			proposer: string;
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
			id: string;
		};
		bfc_balance: number;
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

export type VotingBfc = {
	id: {
		id: string;
	};
	pool_id: string;
	principal: string;
};

export type Vote = {
	agree: true;
	id: {
		id: string;
	};
	proposer: string;
	vid: string;
	vote: {
		type: string;
		fields: {
			id: {
				id: string;
			};
			pool_id: string;
			principal: string;
		};
	};
};
