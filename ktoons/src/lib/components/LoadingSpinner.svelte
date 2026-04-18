<script lang="ts">
	let { message = 'Loading...' }: { message?: string } = $props();
</script>

<div class="spinner-container">
	<div class="spinner"></div>
	<p>{message}</p>
</div>

<style>
	.spinner-container {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.75rem;
		padding: 2rem;
	}

	.spinner {
		width: 2rem;
		height: 2rem;
		border: 3px solid #e0e0e0;
		border-top-color: #3b82f6;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	p {
		color: #666;
		font-size: 0.875rem;
	}
</style>
