// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck
/**
 * Ampli - A strong typed wrapper for your Analytics
 *
 * This file is generated by Amplitude.
 * To update run 'ampli pull web'
 *
 * Required dependencies: @amplitude/analytics-browser@^1.3.0
 * Tracking Plan Version: 3
 * Build: 1.0.0
 * Runtime: browser:typescript-ampli-v2
 *
 * [View Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest)
 *
 * [Full Setup Instructions](https://data.amplitude.com/mystenlabs/Sui%20Wallet/implementation/web)
 */

import * as amplitude from '@amplitude/analytics-browser';

export type Environment = 'production' | 'development';

export const ApiKey: Record<Environment, string> = {
	production: '05c63856469c4e1d4aa9a6c1b2298d3b',
	development: '938968357fcb3bf031d27b5049f62449',
};

/**
 * Default Amplitude configuration options. Contains tracking plan information.
 */
export const DefaultConfiguration: BrowserOptions = {
	plan: {
		version: '3',
		branch: 'main',
		source: 'web',
		versionId: 'e417ca76-bc93-4e57-b900-d5cd7c4b033c',
	},
	...{
		ingestionMetadata: {
			sourceName: 'browser-typescript-ampli',
			sourceVersion: '2.0.0',
		},
	},
};

export interface LoadOptionsBase {
	disabled?: boolean;
}

export type LoadOptionsWithEnvironment = LoadOptionsBase & {
	environment: Environment;
	client?: { configuration?: BrowserOptions };
};
export type LoadOptionsWithApiKey = LoadOptionsBase & {
	client: { apiKey: string; configuration?: BrowserOptions };
};
export type LoadOptionsWithClientInstance = LoadOptionsBase & {
	client: { instance: BrowserClient };
};

export type LoadOptions =
	| LoadOptionsWithEnvironment
	| LoadOptionsWithApiKey
	| LoadOptionsWithClientInstance;

export interface IdentifyProperties {
	/**
	 * The type of account currently being used by the user.
	 */
	activeAccountType?: string;
	/**
	 * The Sui Network that the user is currently interacting with.
	 */
	activeNetwork: string;
	/**
	 * The active origin website that the extension is being used on.
	 */
	activeOrigin?: string;
	/**
	 * The path (e.g., /apps) of a given screen in the wallet.
	 */
	pagePath: string;
	/**
	 * The fully qualified path fragment (e.g., /apps?is_connected=true) of a given screen.
	 */
	pagePathFragment: string;
	/**
	 * The mode that the wallet is currently in.
	 *
	 * | Rule | Value |
	 * |---|---|
	 * | Enum Values | Fullscreen, Pop-up |
	 */
	walletAppMode?: 'Fullscreen' | 'Pop-up';
	/**
	 * The version of the wallet the user has installed.
	 */
	walletVersion: string;
}

export interface AddedAccountsProperties {
	/**
	 * The type of wallet account (e.g., Ledger, Qredo, etc.)
	 *
	 * | Rule | Value |
	 * |---|---|
	 * | Enum Values | Ledger, Qredo, Derived, Imported, Zklogin |
	 */
	accountType: 'Ledger' | 'Qredo' | 'Derived' | 'Imported' | 'Zklogin';
	/**
	 * The number of accounts imported.
	 *
	 * | Rule | Value |
	 * |---|---|
	 * | Type | number |
	 */
	numberOfAccounts: number;
}

export interface ClickedBullsharkQuestsCtaProperties {
	/**
	 * The flow the user came from.
	 */
	sourceFlow: string;
}

export interface ClickedCollectibleCardProperties {
	/**
	 * The object type of a collectible.
	 */
	collectibleType: string;
	/**
	 * The ID of an object on Sui.
	 */
	objectId: string;
	sourceScreen?: string;
}

export interface ClickedCreateNewAccountProperties {
	/**
	 * The flow the user came from.
	 */
	sourceFlow: string;
}

export interface ClickedHideAssetProperties {
	/**
	 * The object type of a collectible.
	 */
	collectibleType: string;
	/**
	 * The ID of an object on Sui.
	 */
	objectId: string;
}

export interface ClickedImportPassphraseProperties {
	/**
	 * The flow the user came from.
	 */
	sourceFlow: string;
}

export interface ClickedImportPrivateKeyProperties {
	/**
	 * The flow the user came from.
	 */
	sourceFlow: string;
}

export interface ClickedSocialSignInButtonProperties {
	/**
	 * The name (e.g., Microsoft) of a social sign-in provider.
	 *
	 * | Rule | Value |
	 * |---|---|
	 * | Enum Values | Microsoft, Facebook, Google, Twitch, Kakao |
	 */
	signInProvider: 'Microsoft' | 'Facebook' | 'Google' | 'Twitch' | 'Kakao';
	/**
	 * The flow the user came from.
	 */
	sourceFlow: string;
}

export interface ClickedStakeSuiProperties {
	/**
	 * Whether or not the user is already staking some SUI.
	 */
	isCurrentlyStaking: boolean;
	/**
	 * The flow the user came from.
	 */
	sourceFlow: string;
}

export interface ClickedSwapCoinProperties {
	/**
	 * The type of a coin.
	 */
	coinType: string;
	/**
	 * The flow the user came from.
	 */
	sourceFlow: string;
	/**
	 * The total balance in SUI of the selected coin that the user has.
	 *
	 * | Rule | Value |
	 * |---|---|
	 * | Type | number |
	 */
	totalBalance: number;
}

export interface ClickedUnstakeSuiProperties {
	/**
	 * The amount of SUI staked.
	 *
	 * | Rule | Value |
	 * |---|---|
	 * | Type | number |
	 */
	stakedAmount: number;
	/**
	 * The address of the selected validator.
	 */
	validatorAddress: string;
}

export interface ConnectedHardwareWalletProperties {
	/**
	 * The type of hardware wallet that was connected to.
	 *
	 * | Rule | Value |
	 * |---|---|
	 * | Enum Values | Ledger |
	 */
	hardwareWalletType: 'Ledger';
}

export interface DisconnectedApplicationProperties {
	/**
	 * The name of the application being visited.
	 */
	applicationName?: string;
	/**
	 * The URL of the application that was disconnected.
	 */
	applicationUrl: string;
	/**
	 * The number of accounts that were disconnected.
	 *
	 * | Rule | Value |
	 * |---|---|
	 * | Type | integer |
	 */
	disconnectedAccounts: number;
	/**
	 * The flow the user disconnected the application from.
	 */
	sourceFlow: string;
}

export interface ImportedExistingAccountProperties {
	/**
	 * The flow the user came from.
	 */
	sourceFlow: string;
}

export interface OpenedApplicationProperties {
	/**
	 * The name of the application being visited.
	 */
	applicationName: string;
}

export interface OpenedConnectLedgerFlowProperties {
	/**
	 * The flow the user came from.
	 */
	sourceFlow: string;
}

export interface PinnedCoinProperties {
	/**
	 * The type of a coin.
	 */
	coinType: string;
}

export interface RespondedToConnectionRequestProperties {
	/**
	 * The name of the application that initiated the connection request.
	 */
	applicationName?: string;
	/**
	 * The URL of the application that initiated the transaction.
	 */
	applicationUrl: string;
	/**
	 * Whether or not the user approved an application connection request.
	 */
	approvedConnection: boolean;
}

export interface RespondedToTransactionRequestProperties {
	/**
	 * The URL of the application that initiated the transaction.
	 */
	applicationUrl: string;
	/**
	 * Whether or not a transaction request was approved.
	 */
	approvedTransaction: boolean;
	/**
	 * Whether or not users received a failure warning when signing a transaction.
	 */
	receivedFailureWarning: boolean;
}

export interface SelectedCoinProperties {
	/**
	 * The type of a coin.
	 */
	coinType: string;
	/**
	 * The total balance in SUI of the selected coin that the user has.
	 *
	 * | Rule | Value |
	 * |---|---|
	 * | Type | number |
	 */
	totalBalance: number;
}

export interface SelectedValidatorProperties {
	/**
	 * The address of the selected validator.
	 */
	validatorAddress: string;
	/**
	 * The current APY % of the selected validator.
	 *
	 * | Rule | Value |
	 * |---|---|
	 * | Type | number |
	 */
	validatorAPY: number;
	/**
	 * The name of the selected validator.
	 */
	validatorName: string;
}

export interface SentCoinsProperties {
	coinType: string;
}

export interface SentCollectibleProperties {
	/**
	 * The ID of an object on Sui.
	 */
	objectId: string;
}

export interface StakedSuiProperties {
	/**
	 * The amount of SUI staked.
	 *
	 * | Rule | Value |
	 * |---|---|
	 * | Type | number |
	 */
	stakedAmount: number;
	/**
	 * The address of the selected validator.
	 */
	validatorAddress: string;
}

export interface SwappedCoinProperties {
	/**
	 * | Rule | Value |
	 * |---|---|
	 * | Type | number |
	 */
	estimatedReturnBalance: number;
	fromCoinType: string;
	toCoinType: string;
	/**
	 * The total balance in SUI of the selected coin that the user has.
	 *
	 * | Rule | Value |
	 * |---|---|
	 * | Type | number |
	 */
	totalBalance: number;
}

export interface SwitchedAccountProperties {
	/**
	 * The type of account that is being switched to.
	 */
	toAccountType: string;
}

export interface SwitchedNetworkProperties {
	toNetwork: string;
}

export interface UnpinnedCoinProperties {
	/**
	 * The type of a coin.
	 */
	coinType: string;
}

export interface UnstakedSuiProperties {
	/**
	 * The address of the selected validator.
	 */
	validatorAddress: string;
}

export interface VisitedFiatOnRampProperties {
	/**
	 * The name of the fiat on-ramp provider.
	 */
	providerName: string;
}

export class Identify implements BaseEvent {
	event_type = amplitude.Types.SpecialEventType.IDENTIFY;

	constructor(public event_properties: IdentifyProperties) {
		this.event_properties = event_properties;
	}
}

export class AddedAccounts implements BaseEvent {
	event_type = 'added accounts';

	constructor(public event_properties: AddedAccountsProperties) {
		this.event_properties = event_properties;
	}
}

export class ClickedBullsharkQuestsCta implements BaseEvent {
	event_type = 'clicked bullshark quests cta';

	constructor(public event_properties: ClickedBullsharkQuestsCtaProperties) {
		this.event_properties = event_properties;
	}
}

export class ClickedCollectibleCard implements BaseEvent {
	event_type = 'clicked collectible card';

	constructor(public event_properties: ClickedCollectibleCardProperties) {
		this.event_properties = event_properties;
	}
}

export class ClickedCreateNewAccount implements BaseEvent {
	event_type = 'clicked create new account';

	constructor(public event_properties: ClickedCreateNewAccountProperties) {
		this.event_properties = event_properties;
	}
}

export class ClickedCreateNewWallet implements BaseEvent {
	event_type = 'clicked create new wallet';
}

export class ClickedGetStarted implements BaseEvent {
	event_type = 'clicked get started';
}

export class ClickedHideAsset implements BaseEvent {
	event_type = 'clicked hide asset';

	constructor(public event_properties: ClickedHideAssetProperties) {
		this.event_properties = event_properties;
	}
}

export class ClickedImportExistingWallet implements BaseEvent {
	event_type = 'clicked import existing wallet';
}

export class ClickedImportPassphrase implements BaseEvent {
	event_type = 'clicked import passphrase';

	constructor(public event_properties: ClickedImportPassphraseProperties) {
		this.event_properties = event_properties;
	}
}

export class ClickedImportPrivateKey implements BaseEvent {
	event_type = 'clicked import private key';

	constructor(public event_properties: ClickedImportPrivateKeyProperties) {
		this.event_properties = event_properties;
	}
}

export class ClickedSocialSignInButton implements BaseEvent {
	event_type = 'clicked social sign in button';

	constructor(public event_properties: ClickedSocialSignInButtonProperties) {
		this.event_properties = event_properties;
	}
}

export class ClickedStakeSui implements BaseEvent {
	event_type = 'clicked stake SUI';

	constructor(public event_properties: ClickedStakeSuiProperties) {
		this.event_properties = event_properties;
	}
}

export class ClickedSwapCoin implements BaseEvent {
	event_type = 'clicked swap coin';

	constructor(public event_properties: ClickedSwapCoinProperties) {
		this.event_properties = event_properties;
	}
}

export class ClickedUnstakeSui implements BaseEvent {
	event_type = 'clicked unstake SUI';

	constructor(public event_properties: ClickedUnstakeSuiProperties) {
		this.event_properties = event_properties;
	}
}

export class ConnectedHardwareWallet implements BaseEvent {
	event_type = 'connected hardware wallet';

	constructor(public event_properties: ConnectedHardwareWalletProperties) {
		this.event_properties = event_properties;
	}
}

export class CreatedNewWallet implements BaseEvent {
	event_type = 'created new wallet';
}

export class DisconnectedApplication implements BaseEvent {
	event_type = 'disconnected application';

	constructor(public event_properties: DisconnectedApplicationProperties) {
		this.event_properties = event_properties;
	}
}

export class ImportedExistingAccount implements BaseEvent {
	event_type = 'imported existing account';

	constructor(public event_properties: ImportedExistingAccountProperties) {
		this.event_properties = event_properties;
	}
}

export class OpenedApplication implements BaseEvent {
	event_type = 'opened application';

	constructor(public event_properties: OpenedApplicationProperties) {
		this.event_properties = event_properties;
	}
}

export class OpenedConnectLedgerFlow implements BaseEvent {
	event_type = 'opened connect ledger flow';

	constructor(public event_properties: OpenedConnectLedgerFlowProperties) {
		this.event_properties = event_properties;
	}
}

export class OpenedWalletExtension implements BaseEvent {
	event_type = 'opened wallet extension';
}

export class PinnedCoin implements BaseEvent {
	event_type = 'pinned coin';

	constructor(public event_properties: PinnedCoinProperties) {
		this.event_properties = event_properties;
	}
}

export class RespondedToConnectionRequest implements BaseEvent {
	event_type = 'responded to connection request';

	constructor(public event_properties: RespondedToConnectionRequestProperties) {
		this.event_properties = event_properties;
	}
}

export class RespondedToTransactionRequest implements BaseEvent {
	event_type = 'responded to transaction request';

	constructor(public event_properties: RespondedToTransactionRequestProperties) {
		this.event_properties = event_properties;
	}
}

export class SelectedCoin implements BaseEvent {
	event_type = 'selected coin';

	constructor(public event_properties: SelectedCoinProperties) {
		this.event_properties = event_properties;
	}
}

export class SelectedValidator implements BaseEvent {
	event_type = 'selected validator';

	constructor(public event_properties: SelectedValidatorProperties) {
		this.event_properties = event_properties;
	}
}

export class SentCoins implements BaseEvent {
	event_type = 'sent coins';

	constructor(public event_properties: SentCoinsProperties) {
		this.event_properties = event_properties;
	}
}

export class SentCollectible implements BaseEvent {
	event_type = 'sent collectible';

	constructor(public event_properties: SentCollectibleProperties) {
		this.event_properties = event_properties;
	}
}

export class StakedSui implements BaseEvent {
	event_type = 'staked SUI';

	constructor(public event_properties: StakedSuiProperties) {
		this.event_properties = event_properties;
	}
}

export class SwappedCoin implements BaseEvent {
	event_type = 'swapped coin';

	constructor(public event_properties: SwappedCoinProperties) {
		this.event_properties = event_properties;
	}
}

export class SwitchedAccount implements BaseEvent {
	event_type = 'switched account';

	constructor(public event_properties: SwitchedAccountProperties) {
		this.event_properties = event_properties;
	}
}

export class SwitchedNetwork implements BaseEvent {
	event_type = 'switched network';

	constructor(public event_properties: SwitchedNetworkProperties) {
		this.event_properties = event_properties;
	}
}

export class UnpinnedCoin implements BaseEvent {
	event_type = 'unpinned coin';

	constructor(public event_properties: UnpinnedCoinProperties) {
		this.event_properties = event_properties;
	}
}

export class UnstakedSui implements BaseEvent {
	event_type = 'unstaked SUI';

	constructor(public event_properties: UnstakedSuiProperties) {
		this.event_properties = event_properties;
	}
}

export class ViewedLedgerTutorial implements BaseEvent {
	event_type = 'viewed ledger tutorial';
}

export class VisitedFiatOnRamp implements BaseEvent {
	event_type = 'visited fiat on-ramp';

	constructor(public event_properties: VisitedFiatOnRampProperties) {
		this.event_properties = event_properties;
	}
}

export type PromiseResult<T> = { promise: Promise<T | void> };

const getVoidPromiseResult = () => ({ promise: Promise.resolve() });

// prettier-ignore
export class Ampli {
  private disabled: boolean = false;
  private amplitude?: BrowserClient;

  get client(): BrowserClient {
    this.isInitializedAndEnabled();
    return this.amplitude!;
  }

  get isLoaded(): boolean {
    return this.amplitude != null;
  }

  private isInitializedAndEnabled(): boolean {
    // if (!this.amplitude) {
    //   console.error('ERROR: Ampli is not yet initialized. Have you called ampli.load() on app start?');
    //   return false;
    // }
    // return !this.disabled;
    return false;
  }

  /**
   * Initialize the Ampli SDK. Call once when your application starts.
   *
   * @param options Configuration options to initialize the Ampli SDK with.
   */
  load(options: LoadOptions): PromiseResult<void> {
    this.disabled = options.disabled ?? false;

    if (this.amplitude) {
      console.warn('WARNING: Ampli is already initialized. Ampli.load() should be called once at application startup.');
      return getVoidPromiseResult();
    }

    let apiKey: string | null = null;
    if (options.client && 'apiKey' in options.client) {
      apiKey = options.client.apiKey;
    } else if ('environment' in options) {
      apiKey = ApiKey[options.environment];
    }

    if (options.client && 'instance' in options.client) {
      this.amplitude = options.client.instance;
    } else if (apiKey) {
      this.amplitude = amplitude.createInstance();
      const configuration = (options.client && 'configuration' in options.client) ? options.client.configuration : {};
      return this.amplitude.init(apiKey, undefined, { ...DefaultConfiguration, ...configuration });
    } else {
      console.error("ERROR: ampli.load() requires 'environment', 'client.apiKey', or 'client.instance'");
    }

    return getVoidPromiseResult();
  }

  /**
   * Identify a user and set user properties.
   *
   * @param userId The user's id.
   * @param properties The user properties.
   * @param options Optional event options.
   */
  identify(
    userId: string | undefined,
    properties: IdentifyProperties,
    options?: EventOptions,
  ): PromiseResult<Result> {
    if (!this.isInitializedAndEnabled()) {
      return getVoidPromiseResult();
    }

    if (userId) {
      options = {...options,  user_id: userId};
    }

    const amplitudeIdentify = new amplitude.Identify();
    const eventProperties = properties;
    if (eventProperties != null) {
      for (const [key, value] of Object.entries(eventProperties)) {
        amplitudeIdentify.set(key, value);
      }
    }
    return this.amplitude!.identify(
      amplitudeIdentify,
      options,
    );
  }

 /**
  * Flush the event.
  */
  flush() : PromiseResult<Result> {
    if (!this.isInitializedAndEnabled()) {
      return getVoidPromiseResult();
    }

    return this.amplitude!.flush();
  }

  /**
   * Track event
   *
   * @param event The event to track.
   * @param options Optional event options.
   */
  track(event: Event, options?: EventOptions): PromiseResult<Result> {
    // if (!this.isInitializedAndEnabled()) {
    //   return getVoidPromiseResult();
    // }

    // return this.amplitude!.track(event, undefined, options);
    return getVoidPromiseResult();
  }

  /**
   * added accounts
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/added%20accounts)
   *
   * When users successfully add new accounts to the wallet.
   *
   * Owner: Jon Shek
   *
   * @param properties The event's properties (e.g. accountType)
   * @param options Amplitude event options.
   */
  addedAccounts(
    properties: AddedAccountsProperties,
    options?: EventOptions,
  ) {
    return this.track(new AddedAccounts(properties), options);
  }

  /**
   * clicked bullshark quests cta
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/clicked%20bullshark%20quests%20cta)
   *
   * When users click the call-to-action for the Bullshark Quests interstitial/banner.
   *
   * @param properties The event's properties (e.g. sourceFlow)
   * @param options Amplitude event options.
   */
  clickedBullsharkQuestsCta(
    properties: ClickedBullsharkQuestsCtaProperties,
    options?: EventOptions,
  ) {
    return this.track(new ClickedBullsharkQuestsCta(properties), options);
  }

  /**
   * clicked collectible card
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/clicked%20collectible%20card)
   *
   * When users click to view a collectible in the wallet.
   *
   * Owner: William Robertson
   *
   * @param properties The event's properties (e.g. collectibleType)
   * @param options Amplitude event options.
   */
  clickedCollectibleCard(
    properties: ClickedCollectibleCardProperties,
    options?: EventOptions,
  ) {
    return this.track(new ClickedCollectibleCard(properties), options);
  }

  /**
   * clicked create new account
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/clicked%20create%20new%20account)
   *
   * When users click the button to create a new passphrase account.
   *
   * @param properties The event's properties (e.g. sourceFlow)
   * @param options Amplitude event options.
   */
  clickedCreateNewAccount(
    properties: ClickedCreateNewAccountProperties,
    options?: EventOptions,
  ) {
    return this.track(new ClickedCreateNewAccount(properties), options);
  }

  /**
   * clicked create new wallet
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/clicked%20create%20new%20wallet)
   *
   * When users click to create a new wallet during onboarding.
   *
   * Owner: Jon Shek
   *
   * @param options Amplitude event options.
   */
  clickedCreateNewWallet(
    options?: EventOptions,
  ) {
    return this.track(new ClickedCreateNewWallet(), options);
  }

  /**
   * clicked get started
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/clicked%20get%20started)
   *
   * When users click "Get Started" after installing the wallet.
   *
   * Owner: Jon Shek
   *
   * @param options Amplitude event options.
   */
  clickedGetStarted(
    options?: EventOptions,
  ) {
    return this.track(new ClickedGetStarted(), options);
  }

  /**
   * clicked hide asset
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/clicked%20hide%20asset)
   *
   * Event has no description in tracking plan.
   *
   * @param properties The event's properties (e.g. collectibleType)
   * @param options Amplitude event options.
   */
  clickedHideAsset(
    properties: ClickedHideAssetProperties,
    options?: EventOptions,
  ) {
    return this.track(new ClickedHideAsset(properties), options);
  }

  /**
   * clicked import existing wallet
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/clicked%20import%20existing%20wallet)
   *
   * When users click to import an existing wallet during onboarding.
   *
   * Owner: Jon Shek
   *
   * @param options Amplitude event options.
   */
  clickedImportExistingWallet(
    options?: EventOptions,
  ) {
    return this.track(new ClickedImportExistingWallet(), options);
  }

  /**
   * clicked import passphrase
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/clicked%20import%20passphrase)
   *
   * When users click to import an account via passphrase.
   *
   * @param properties The event's properties (e.g. sourceFlow)
   * @param options Amplitude event options.
   */
  clickedImportPassphrase(
    properties: ClickedImportPassphraseProperties,
    options?: EventOptions,
  ) {
    return this.track(new ClickedImportPassphrase(properties), options);
  }

  /**
   * clicked import private key
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/clicked%20import%20private%20key)
   *
   * When users click the button to import an account via private key.
   *
   * @param properties The event's properties (e.g. sourceFlow)
   * @param options Amplitude event options.
   */
  clickedImportPrivateKey(
    properties: ClickedImportPrivateKeyProperties,
    options?: EventOptions,
  ) {
    return this.track(new ClickedImportPrivateKey(properties), options);
  }

  /**
   * clicked social sign in button
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/clicked%20social%20sign%20in%20button)
   *
   * When users click a social sign-in button to create an account.
   *
   * @param properties The event's properties (e.g. signInProvider)
   * @param options Amplitude event options.
   */
  clickedSocialSignInButton(
    properties: ClickedSocialSignInButtonProperties,
    options?: EventOptions,
  ) {
    return this.track(new ClickedSocialSignInButton(properties), options);
  }

  /**
   * clicked stake SUI
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/clicked%20stake%20SUI)
   *
   * When users click to stake SUI in the wallet.
   *
   * Owner: Jon Shek
   *
   * @param properties The event's properties (e.g. isCurrentlyStaking)
   * @param options Amplitude event options.
   */
  clickedStakeSui(
    properties: ClickedStakeSuiProperties,
    options?: EventOptions,
  ) {
    return this.track(new ClickedStakeSui(properties), options);
  }

  /**
   * clicked swap coin
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/clicked%20swap%20coin)
   *
   * When users click to swap a coin in the wallet
   *
   * @param properties The event's properties (e.g. coinType)
   * @param options Amplitude event options.
   */
  clickedSwapCoin(
    properties: ClickedSwapCoinProperties,
    options?: EventOptions,
  ) {
    return this.track(new ClickedSwapCoin(properties), options);
  }

  /**
   * clicked unstake SUI
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/clicked%20unstake%20SUI)
   *
   * When users click to unstake SUI.
   *
   * Owner: Jon Shek
   *
   * @param properties The event's properties (e.g. stakedAmount)
   * @param options Amplitude event options.
   */
  clickedUnstakeSui(
    properties: ClickedUnstakeSuiProperties,
    options?: EventOptions,
  ) {
    return this.track(new ClickedUnstakeSui(properties), options);
  }

  /**
   * connected hardware wallet
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/connected%20hardware%20wallet)
   *
   * When users successfully connect their hardware wallet.
   *
   * Owner: Jon Shek
   *
   * @param properties The event's properties (e.g. hardwareWalletType)
   * @param options Amplitude event options.
   */
  connectedHardwareWallet(
    properties: ConnectedHardwareWalletProperties,
    options?: EventOptions,
  ) {
    return this.track(new ConnectedHardwareWallet(properties), options);
  }

  /**
   * created new wallet
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/created%20new%20wallet)
   *
   * When users successfully create a new wallet during onboarding.
   *
   * Owner: Jon Shek
   *
   * @param options Amplitude event options.
   */
  createdNewWallet(
    options?: EventOptions,
  ) {
    return this.track(new CreatedNewWallet(), options);
  }

  /**
   * disconnected application
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/disconnected%20application)
   *
   * When users disconnect from an application in the wallet.
   *
   * Owner: Jon Shek
   *
   * @param properties The event's properties (e.g. applicationName)
   * @param options Amplitude event options.
   */
  disconnectedApplication(
    properties: DisconnectedApplicationProperties,
    options?: EventOptions,
  ) {
    return this.track(new DisconnectedApplication(properties), options);
  }

  /**
   * imported existing account
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/imported%20existing%20account)
   *
   * When users successfully import an existing account during onboarding.
   *
   * Owner: Jon Shek
   *
   * @param properties The event's properties (e.g. sourceFlow)
   * @param options Amplitude event options.
   */
  importedExistingAccount(
    properties: ImportedExistingAccountProperties,
    options?: EventOptions,
  ) {
    return this.track(new ImportedExistingAccount(properties), options);
  }

  /**
   * opened application
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/opened%20application)
   *
   * When users click to open an application from the wallet.
   *
   * Owner: Jon Shek
   *
   * @param properties The event's properties (e.g. applicationName)
   * @param options Amplitude event options.
   */
  openedApplication(
    properties: OpenedApplicationProperties,
    options?: EventOptions,
  ) {
    return this.track(new OpenedApplication(properties), options);
  }

  /**
   * opened connect ledger flow
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/opened%20connect%20ledger%20flow)
   *
   * When users open the "Connect Ledger Wallet" flow.
   *
   * Owner: Jon Shek
   *
   * @param properties The event's properties (e.g. sourceFlow)
   * @param options Amplitude event options.
   */
  openedConnectLedgerFlow(
    properties: OpenedConnectLedgerFlowProperties,
    options?: EventOptions,
  ) {
    return this.track(new OpenedConnectLedgerFlow(properties), options);
  }

  /**
   * opened wallet extension
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/opened%20wallet%20extension)
   *
   * When users first open the wallet extension.
   *
   * Owner: Jon Shek
   *
   * @param options Amplitude event options.
   */
  openedWalletExtension(
    options?: EventOptions,
  ) {
    return this.track(new OpenedWalletExtension(), options);
  }

  /**
   * pinned coin
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/pinned%20coin)
   *
   * When users pin an unrecognized coin on the home page.
   *
   * Owner: William Robertson
   *
   * @param properties The event's properties (e.g. coinType)
   * @param options Amplitude event options.
   */
  pinnedCoin(
    properties: PinnedCoinProperties,
    options?: EventOptions,
  ) {
    return this.track(new PinnedCoin(properties), options);
  }

  /**
   * responded to connection request
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/responded%20to%20connection%20request)
   *
   * When users respond to a connection request in the wallet.
   *
   * Owner: William Robertson
   *
   * @param properties The event's properties (e.g. applicationName)
   * @param options Amplitude event options.
   */
  respondedToConnectionRequest(
    properties: RespondedToConnectionRequestProperties,
    options?: EventOptions,
  ) {
    return this.track(new RespondedToConnectionRequest(properties), options);
  }

  /**
   * responded to transaction request
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/responded%20to%20transaction%20request)
   *
   * When users respond to a transaction request from an application.
   *
   * Owner: Jon Shek
   *
   * @param properties The event's properties (e.g. applicationUrl)
   * @param options Amplitude event options.
   */
  respondedToTransactionRequest(
    properties: RespondedToTransactionRequestProperties,
    options?: EventOptions,
  ) {
    return this.track(new RespondedToTransactionRequest(properties), options);
  }

  /**
   * selected coin
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/selected%20coin)
   *
   * When users select a specific coin from the home screen.
   *
   * Owner: Jon Shek
   *
   * @param properties The event's properties (e.g. coinType)
   * @param options Amplitude event options.
   */
  selectedCoin(
    properties: SelectedCoinProperties,
    options?: EventOptions,
  ) {
    return this.track(new SelectedCoin(properties), options);
  }

  /**
   * selected validator
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/selected%20validator)
   *
   * When users select a validator in the staking flow.
   *
   * Owner: Jon Shek
   *
   * @param properties The event's properties (e.g. validatorAddress)
   * @param options Amplitude event options.
   */
  selectedValidator(
    properties: SelectedValidatorProperties,
    options?: EventOptions,
  ) {
    return this.track(new SelectedValidator(properties), options);
  }

  /**
   * sent coins
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/sent%20coins)
   *
   * When users successfully send coins to someone.
   *
   * Owner: Jon Shek
   *
   * @param properties The event's properties (e.g. coinType)
   * @param options Amplitude event options.
   */
  sentCoins(
    properties: SentCoinsProperties,
    options?: EventOptions,
  ) {
    return this.track(new SentCoins(properties), options);
  }

  /**
   * sent collectible
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/sent%20collectible)
   *
   * Owner: William Robertson
   *
   * @param properties The event's properties (e.g. objectId)
   * @param options Amplitude event options.
   */
  sentCollectible(
    properties: SentCollectibleProperties,
    options?: EventOptions,
  ) {
    return this.track(new SentCollectible(properties), options);
  }

  /**
   * staked SUI
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/staked%20SUI)
   *
   * When users successfully stake SUI with a validator.
   *
   * Owner: Jon Shek
   *
   * @param properties The event's properties (e.g. stakedAmount)
   * @param options Amplitude event options.
   */
  stakedSui(
    properties: StakedSuiProperties,
    options?: EventOptions,
  ) {
    return this.track(new StakedSui(properties), options);
  }

  /**
   * swapped coin
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/swapped%20coin)
   *
   * When users complete swapping 1 coin to another
   *
   * @param properties The event's properties (e.g. estimatedReturnBalance)
   * @param options Amplitude event options.
   */
  swappedCoin(
    properties: SwappedCoinProperties,
    options?: EventOptions,
  ) {
    return this.track(new SwappedCoin(properties), options);
  }

  /**
   * switched account
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/switched%20account)
   *
   * When users switch their active account in the wallet.
   *
   * Owner: Jon Shek
   *
   * @param properties The event's properties (e.g. toAccountType)
   * @param options Amplitude event options.
   */
  switchedAccount(
    properties: SwitchedAccountProperties,
    options?: EventOptions,
  ) {
    return this.track(new SwitchedAccount(properties), options);
  }

  /**
   * switched network
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/switched%20network)
   *
   * When users switch between different network connections.
   *
   * Owner: William Robertson
   *
   * @param properties The event's properties (e.g. toNetwork)
   * @param options Amplitude event options.
   */
  switchedNetwork(
    properties: SwitchedNetworkProperties,
    options?: EventOptions,
  ) {
    return this.track(new SwitchedNetwork(properties), options);
  }

  /**
   * unpinned coin
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/unpinned%20coin)
   *
   * When users un-pin a recognized coin on the home page.
   *
   * Owner: William Robertson
   *
   * @param properties The event's properties (e.g. coinType)
   * @param options Amplitude event options.
   */
  unpinnedCoin(
    properties: UnpinnedCoinProperties,
    options?: EventOptions,
  ) {
    return this.track(new UnpinnedCoin(properties), options);
  }

  /**
   * unstaked SUI
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/unstaked%20SUI)
   *
   * When users successfully un-stake SUI.
   *
   * Owner: Jon Shek
   *
   * @param properties The event's properties (e.g. validatorAddress)
   * @param options Amplitude event options.
   */
  unstakedSui(
    properties: UnstakedSuiProperties,
    options?: EventOptions,
  ) {
    return this.track(new UnstakedSui(properties), options);
  }

  /**
   * viewed ledger tutorial
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/viewed%20ledger%20tutorial)
   *
   * When users click the link to get help with connecting their Ledger wallet.
   *
   * Owner: Jon Shek
   *
   * @param options Amplitude event options.
   */
  viewedLedgerTutorial(
    options?: EventOptions,
  ) {
    return this.track(new ViewedLedgerTutorial(), options);
  }

  /**
   * visited fiat on-ramp
   *
   * [View in Tracking Plan](https://data.amplitude.com/mystenlabs/Sui%20Wallet/events/main/latest/visited%20fiat%20on-ramp)
   *
   * When users visit a fiat on-ramp from the wallet.
   *
   * Owner: Jon Shek
   *
   * @param properties The event's properties (e.g. providerName)
   * @param options Amplitude event options.
   */
  visitedFiatOnRamp(
    properties: VisitedFiatOnRampProperties,
    options?: EventOptions,
  ) {
    return this.track(new VisitedFiatOnRamp(properties), options);
  }
}

export const ampli = new Ampli();

// BASE TYPES
type BrowserOptions = amplitude.Types.BrowserOptions;

export type BrowserClient = amplitude.Types.BrowserClient;
export type BaseEvent = amplitude.Types.BaseEvent;
export type IdentifyEvent = amplitude.Types.IdentifyEvent;
export type GroupEvent = amplitude.Types.GroupIdentifyEvent;
export type Event = amplitude.Types.Event;
export type EventOptions = amplitude.Types.EventOptions;
export type Result = amplitude.Types.Result;
