import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import CharacterHeader from '../../src/lib/components/CharacterHeader.svelte';
import type { FullCharacter } from '../../src/lib/types';

describe('CharacterHeader partial failure', () => {
	it('shows "no profile data" when profile is null', () => {
		const character: FullCharacter = {
			realm_slug: 'trollbane',
			character_name: 'Unknown',
			has_profile_data: false,
			fetched_at: '2026-04-17T00:00:00Z',
			errors: [{ endpoint: 'profile', message: '404 Not Found' }],
			profile: null,
			equipment: null,
			statistics: null,
			specializations: null,
			media: null
		};
		render(CharacterHeader, { props: { character } });
		expect(screen.getByText(/No profile data available/)).toBeTruthy();
	});
});
