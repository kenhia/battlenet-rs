/** Realm entry for the dropdown (from get_realms command). */
export interface RealmEntry {
	name: string;
	slug: string;
}

/**
 * Character list entry for the left navigation.
 * Frontend-only state — accumulated from lookups or populated from account profile.
 *
 * Note: AccountCharacterEntry is the backend response shape from `login`;
 * CharacterListEntry is the frontend nav state (superset, includes quick-lookup characters).
 */
export interface CharacterListEntry {
	name: string;
	realmName: string;
	realmSlug: string;
	level: number;
	className: string;
	raceName: string;
	faction: string;
}

/** Account character entry returned by the login command. */
export interface AccountCharacterEntry {
	name: string;
	realm_name: string;
	realm_slug: string;
	level: number;
	class_name: string;
	faction: string;
	id: number;
}

// --- FullCharacter types (MVP-relevant fields only) ---

export interface HrefLink {
	href: string;
}

export interface TypeAndName {
	type_?: string;
	name: string;
}

export interface NameAndId {
	name: string;
	id: number;
}

export interface KeyAndId {
	key: HrefLink;
	id: number;
}

export interface CharacterProfile {
	name: string;
	level: number;
	race: NameAndId;
	character_class: NameAndId;
	faction: TypeAndName;
	guild?: { name: string; realm: { name: string; slug: string } };
	realm: { name: string; slug: string; id: number };
	active_title?: { name: string; display_string?: string };
	achievement_points: number;
	equipped_item_level: number;
	average_item_level: number;
}

export interface EquippedItem {
	slot: TypeAndName;
	name: string;
	level: { value: number };
	quality: TypeAndName;
}

export interface CharacterEquipmentSummary {
	equipped_items: EquippedItem[];
}

export interface StatValue {
	base: number;
	effective: number;
}

export interface CharacterStatisticsSummary {
	health: number;
	power: number;
	power_type: NameAndId;
	strength: StatValue;
	agility: StatValue;
	intellect: StatValue;
	stamina: StatValue;
}

export interface SpecEntry {
	specialization: NameAndId;
}

export interface CharacterSpecializationsSummary {
	active_specialization: NameAndId;
	specializations: SpecEntry[];
}

export interface MediaAsset {
	key: string;
	value: string;
}

export interface CharacterMediaSummary {
	assets: MediaAsset[];
}

export interface EndpointError {
	endpoint: string;
	message: string;
}

/** Full character data returned by lookup/get/refresh commands. */
export interface FullCharacter {
	realm_slug: string;
	character_name: string;
	has_profile_data: boolean;
	fetched_at: string;
	errors: EndpointError[];

	profile: CharacterProfile | null;
	equipment: CharacterEquipmentSummary | null;
	statistics: CharacterStatisticsSummary | null;
	specializations: CharacterSpecializationsSummary | null;
	media: CharacterMediaSummary | null;
}
