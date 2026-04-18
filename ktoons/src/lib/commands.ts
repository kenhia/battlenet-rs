import { invoke } from '@tauri-apps/api/core';
import type { RealmEntry, FullCharacter, AccountCharacterEntry } from './types';

/** Fetch the list of available realms for the realm dropdown. */
export async function getRealms(): Promise<RealmEntry[]> {
	return invoke<RealmEntry[]>('get_realms');
}

/** Look up a character by name and realm (client token only, no OAuth). */
export async function lookupCharacter(realmSlug: string, characterName: string): Promise<FullCharacter> {
	return invoke<FullCharacter>('lookup_character', { realmSlug, characterName });
}

/** Initiate OAuth login and return the account character list. */
export async function login(): Promise<AccountCharacterEntry[]> {
	return invoke<AccountCharacterEntry[]>('login');
}

/** Fetch a character using user token if available, otherwise client token. */
export async function getCharacter(realmSlug: string, characterName: string): Promise<FullCharacter> {
	return invoke<FullCharacter>('get_character', { realmSlug, characterName });
}

/** Force re-fetch character data from API, bypassing cache. */
export async function refreshCharacter(realmSlug: string, characterName: string): Promise<FullCharacter> {
	return invoke<FullCharacter>('refresh_character', { realmSlug, characterName });
}
