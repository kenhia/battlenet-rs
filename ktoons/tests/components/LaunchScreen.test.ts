import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import LaunchScreen from '../../src/lib/components/LaunchScreen.svelte';
import type { RealmEntry } from '../../src/lib/types';

const mockRealms: RealmEntry[] = [
	{ name: 'Trollbane', slug: 'trollbane' },
	{ name: 'Area 52', slug: 'area-52' }
];

describe('LaunchScreen', () => {
	it('renders the lookup button', () => {
		render(LaunchScreen, { props: { realms: mockRealms, onLookup: vi.fn(), onLogin: vi.fn() } });
		expect(screen.getByText('Lookup')).toBeTruthy();
	});

	it('renders the login button', () => {
		render(LaunchScreen, { props: { realms: mockRealms, onLookup: vi.fn(), onLogin: vi.fn() } });
		expect(screen.getByText(/Login with Battle\.net/)).toBeTruthy();
	});

	it('renders realm options from props', () => {
		render(LaunchScreen, { props: { realms: mockRealms, onLookup: vi.fn(), onLogin: vi.fn() } });
		const options = document.querySelectorAll('option');
		// +1 for placeholder option
		expect(options.length).toBeGreaterThanOrEqual(2);
	});

	it('renders character name input', () => {
		render(LaunchScreen, { props: { realms: mockRealms, onLookup: vi.fn(), onLogin: vi.fn() } });
		const input = document.querySelector('input[type="text"]');
		expect(input).toBeTruthy();
	});
});
