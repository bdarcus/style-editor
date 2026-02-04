<script lang="ts">
    import LivePreview from '$lib/components/LivePreview.svelte';
    import ComprehensivePreview from '$lib/components/ComprehensivePreview.svelte';
    import DecisionWizard from '$lib/components/DecisionWizard.svelte';
    import { onMount } from 'svelte';
    import { intent } from '$lib/stores/intent';
    import type { DecisionPackage, Preview } from '$lib/types/bindings';

    let currentDecision: DecisionPackage | null = $state(null);

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

        <!-- RIGHT: Comprehensive Preview (Inspired by Stitch Screen #4) -->
        <main class="flex-1 bg-white rounded-3xl border border-border-light shadow-sm p-4 lg:p-8 relative overflow-hidden flex flex-col items-center">
            <div class="w-full max-w-[800px] flex flex-col gap-12">
                <ComprehensivePreview 
                    previewSet={currentDecision ? {
                        in_text: currentDecision.in_text_preview,
                        note: currentDecision.note_preview,
                        bibliography: currentDecision.bibliography_preview
                    } : null} 
                />

                <!-- About logic -->
                <div class="mt-8 p-6 bg-blue-50/50 rounded-2xl border border-blue-100/50">
                    <h4 class="text-xs font-black text-blue-900 uppercase tracking-widest mb-2">Technical Insight</h4>
                    <p class="text-[11px] text-blue-800/70 leading-relaxed">
                        These previews are generated in real-time using the <strong>CSLN Processor</strong>. 
                        The formatting matches exactly what will appear in your document editor based on the current logic.
                    </p>
                </div>
            </div>
        </main>
    </div>
</div>
