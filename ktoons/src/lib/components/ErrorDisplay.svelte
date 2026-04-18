<script lang="ts">
	let { message, onRetry }: { message: string; onRetry?: () => void } = $props();
</script>

<div class="error-container">
	<p class="error-message">{message}</p>
	{#if onRetry}
		<button class="retry-button" onclick={onRetry}>Retry</button>
	{/if}
</div>

<style>
	.error-container {
		padding: 1rem;
		border: 1px solid #ef4444;
		border-radius: 0.5rem;
		background: #fef2f2;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.75rem;
	}

	.error-message {
		color: #dc2626;
		margin: 0;
		text-align: center;
		word-break: break-word;
	}

	.retry-button {
		padding: 0.5rem 1rem;
		background: #ef4444;
		color: white;
		border: none;
		border-radius: 0.25rem;
		cursor: pointer;
		font-size: 0.875rem;
	}

	.retry-button:hover {
		background: #dc2626;
	}
</style>
