import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import ErrorDisplay from '../../src/lib/components/ErrorDisplay.svelte';

describe('ErrorDisplay', () => {
	it('renders the error message', () => {
		render(ErrorDisplay, { props: { message: 'Something went wrong' } });
		expect(screen.getByText('Something went wrong')).toBeTruthy();
	});

	it('renders retry button when onRetry is provided', () => {
		const onRetry = vi.fn();
		render(ErrorDisplay, { props: { message: 'Error', onRetry } });
		expect(screen.getByText('Retry')).toBeTruthy();
	});

	it('does not render retry button when onRetry is not provided', () => {
		render(ErrorDisplay, { props: { message: 'Error' } });
		expect(screen.queryByText('Retry')).toBeNull();
	});

	it('calls onRetry when retry button is clicked', async () => {
		const onRetry = vi.fn();
		render(ErrorDisplay, { props: { message: 'Error', onRetry } });
		await fireEvent.click(screen.getByText('Retry'));
		expect(onRetry).toHaveBeenCalledOnce();
	});
});
