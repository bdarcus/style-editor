<script lang="ts">
    import DecisionWizard from '$lib/components/DecisionWizard.svelte';
    import { intent } from '$lib/stores/intent';
    import LivePreview from '$lib/components/LivePreview.svelte';
    import type { DecisionPackage } from '$lib/types/bindings';

    let currentDecision: DecisionPackage | null = $state(null);
    let currentPreviewHtml = $derived(currentDecision?.preview_html || "");

    function handleDecision(event: CustomEvent<DecisionPackage | null>) {
        currentDecision = event.detail;
    }

    const progress = $derived(currentDecision ? Math.round(((4 - currentDecision.missing_fields.length) / 4) * 100) : 0);
    const isComplete = $derived(currentDecision && !currentDecision.question);
</script>

<div class="max-w-[1440px] mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <div class="flex flex-col lg:flex-row gap-10 min-h-[calc(100vh-12rem)] relative">
        <!-- LEFT: Configuration Wizard -->
        <aside class="w-full lg:w-[450px] flex flex-col bg-white border border-border-light rounded-2xl shadow-sm overflow-hidden sticky top-24 h-fit">
            <div class="p-8">
                <!-- Breadcrumbs -->
                <nav class="flex gap-2 mb-6">
                    <a href="/" class="text-text-secondary text-xs font-medium hover:underline">All Styles</a>
                    <span class="text-text-secondary text-xs">/</span>
                    <span class="text-text-main text-xs font-bold">Create New</span>
                </nav>

                <!-- Title & Progress -->
                <div class="flex flex-col gap-2 mb-8">
                    <h1 class="text-2xl font-black text-text-main tracking-tight">Create New Style</h1>
                    <p class="text-text-secondary text-sm">
                        {isComplete ? 'Configuration Complete' : 'Refine your citation style'}
                    </p>

                    <div class="mt-4 flex flex-col gap-2">
                        <div class="flex justify-between items-end">
                            <span class="text-[10px] font-bold uppercase tracking-widest text-text-secondary">Progress</span>
                            <span class="text-primary text-xs font-bold">{progress}%</span>
                        </div>
                        <div class="h-1.5 w-full bg-slate-100 rounded-full overflow-hidden">
                            <div
                                class="h-full bg-primary transition-all duration-500 ease-out"
                                style:width="{progress}%"
                            ></div>
                        </div>
                    </div>
                </div>

                <DecisionWizard on:decision={handleDecision} />
            </div>
        </aside>

        <!-- RIGHT: Live Preview (The "Paper" style) -->
        <main class="flex-1 bg-slate-100/50 rounded-3xl border border-border-light p-8 lg:p-12 relative overflow-hidden flex flex-col items-center">
            <div class="bg-white w-full max-w-[700px] min-h-[850px] shadow-2xl p-12 lg:p-16 relative origin-top transition-transform duration-500 font-serif">
                <div class="absolute top-0 right-0 bg-slate-100 text-slate-400 px-4 py-1.5 text-[10px] font-sans font-black uppercase tracking-widest rounded-bl-xl">
                    Live Preview
                </div>

                <div class="border-b-2 border-slate-900 pb-6 mb-10">
                    <h2 class="text-3xl font-black text-slate-900 leading-tight mb-4">
                        The Evolution of Bibliographic Standards in the Digital Age
                    </h2>
                    <div class="text-slate-600 italic text-sm">By Jonathan K. Smith and Maria Rodriguez</div>
                </div>

                <div class="flex flex-col gap-8 text-lg leading-relaxed text-slate-800 text-justify mb-16">
                    <p>
                        The rapid proliferation of digital publication platforms has necessitated a re-evaluation of traditional citation metrics.
                        As transitions in scholarship occur, the role of metadata remains central.
                    </p>

                    <div class="p-6 bg-slate-50 border border-slate-200 rounded-2xl relative font-sans">
                        <span class="absolute -top-3 left-6 px-2 bg-white text-[10px] font-black uppercase tracking-widest text-primary border border-slate-200 rounded">Current Style Output</span>
                        <div class="prose prose-slate max-w-none pt-2">
                            {#if currentPreviewHtml}
                                <LivePreview html={currentPreviewHtml} />
                            {:else}
                                <p class="text-slate-400 italic">Configure your style to see a preview...</p>
                            {/if}
                        </div>
                    </div>

                    <p>
                        Furthermore, studies indicate that consistency in tagging improves discoverability. We propose a framework that adapts contextually based on the medium.
                    </p>
                </div>

                <div class="mt-12 pt-8 border-t border-slate-200 font-sans">
                    <p class="text-[10px] font-black uppercase tracking-[0.2em] mb-4 text-slate-400">Preview Metadata</p>
                    <div class="flex flex-wrap gap-2">
                        <span class="px-2 py-1 bg-slate-100 rounded text-[10px] font-bold text-slate-600">CLASS: {$intent.class || 'PENDING'}</span>
                        <span class="px-2 py-1 bg-slate-100 rounded text-[10px] font-bold text-slate-600">NAME: {$intent.author_format || 'PENDING'}</span>
                        <span class="px-2 py-1 bg-slate-100 rounded text-[10px] font-bold text-slate-600">BIB: {$intent.has_bibliography ? 'YES' : 'NO'}</span>
                    </div>
                </div>
            </div>

            <!-- Zoom Controls (Mockup styling) -->
            <div class="absolute bottom-10 flex items-center gap-2 p-1 bg-white/80 backdrop-blur shadow-xl rounded-full border border-border-light font-sans">
                <button class="size-10 rounded-full flex items-center justify-center text-text-secondary hover:bg-slate-100 transition-colors">
                    <span class="material-symbols-outlined">zoom_out</span>
                </button>
                <span class="text-xs font-bold text-text-main px-2">100%</span>
                <button class="size-10 rounded-full flex items-center justify-center text-text-secondary hover:bg-slate-100 transition-colors">
                    <span class="material-symbols-outlined">zoom_in</span>
                </button>
            </div>
        </main>
    </div>
</div>
