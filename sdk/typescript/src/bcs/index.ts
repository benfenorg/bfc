// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import {
	Address,
	AppId,
	Argument,
	BenfenObjectRef,
	CallArg,
	Command,
	CompressedSignature,
	GasData,
	Intent,
	IntentMessage,
	IntentScope,
	IntentVersion,
	MultiSig,
	MultiSigPkMap,
	MultiSigPublicKey,
	ObjectArg,
	ObjectDigest,
	PasskeyAuthenticator,
	ProgrammableMoveCall,
	ProgrammableTransaction,
	PublicKey,
	SenderSignedData,
	SenderSignedTransaction,
	SharedObjectRef,
	StructTag,
	TransactionData,
	TransactionDataV1,
	TransactionExpiration,
	TransactionKind,
	TypeTag,
} from './bcs.js';
import { TransactionEffects } from './effects.js';
import { bcs } from './src/index.js';

export type { TypeTag } from './types.js';

export { TypeTagSerializer } from './type-tag-serializer.js';
export * from './src/index.js';

const benfenBcs = {
	...bcs,
	U8: bcs.u8(),
	U16: bcs.u16(),
	U32: bcs.u32(),
	U64: bcs.u64(),
	U128: bcs.u128(),
	U256: bcs.u256(),
	ULEB128: bcs.uleb128(),
	Bool: bcs.bool(),
	String: bcs.string(),
	Address,
	AppId,
	Argument,
	CallArg,
	CompressedSignature,
	GasData,
	Intent,
	IntentMessage,
	IntentScope,
	IntentVersion,
	MultiSig,
	MultiSigPkMap,
	MultiSigPublicKey,
	ObjectArg,
	ObjectDigest,
	ProgrammableMoveCall,
	ProgrammableTransaction,
	PublicKey,
	SenderSignedData,
	SenderSignedTransaction,
	SharedObjectRef,
	StructTag,
	BenfenObjectRef,
	Command,
	TransactionData,
	TransactionDataV1,
	TransactionExpiration,
	TransactionKind,
	TypeTag,
	TransactionEffects,
	PasskeyAuthenticator,
};
export {
	pureBcsSchemaFromTypeName,
	type ShapeFromPureTypeName,
	type PureTypeName,
} from './pure.js';

export { benfenBcs as bcs };
