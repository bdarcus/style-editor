<script lang="ts">
    /**
     * LivePreview component
     * Displays raw HTML content from the backend in a styled container.
     */
    interface Props {
        html: string;
    }

    let { html }: Props = $props();
</script>

<div class="live-preview-content">
    {#if html}
        {@html html}
    {:else}
        <div class="empty-state">
            <p>Select options to see how your citations will look.</p>
        </div>
    {/if}
</div>

<style>
    .live-preview-content {
        line-height: 1.6;
    }

    .empty-state {
        color: var(--color-text-secondary);
        font-style: italic;
        font-size: 0.9rem;
    }
</style>
