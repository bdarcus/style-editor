<script lang="ts">
    import { page } from '$app/stores';
    import { intent } from '$lib/stores/intent';
    import { goto } from '$app/navigation';
    import { bookmarks, toggleBookmark } from '$lib/stores/bookmarks';

    // Mock data for the demonstration
    // In a real app, this would be fetched from the backend or a style provider
    const stylesInfo: Record<string, any> = {
        'apa': {
            name: 'APA 7th Edition',
            description: 'The American Psychological Association style is most commonly used to cite sources within the social sciences.',
            version: 'v7.0.0',
            author: 'American Psychological Association',
            license: 'CC-BY-SA 3.0',
            id: 'http://www.zotero.org/styles/apa',
            tags: ['Social Sciences', 'Author-Date'],
            color: 'from-blue-700 to-blue-500'
        },
        'nature': {
            name: 'Nature',
            description: 'The citation style for the journal Nature. It is a numeric style used widely in the sciences.',
            version: 'v2.1',
            author: 'Nature Publishing Group',
            license: 'CC-BY-SA 3.0',
            id: 'http://www.zotero.org/styles/nature',
            tags: ['Science', 'Numeric'],
            color: 'from-emerald-700 to-emerald-500'
        },
        'chicago': {
            name: 'Chicago 17th Ed (Notes)',
            description: 'The Chicago Manual of Style (Notes and Bibliography) is used primarily in history and the humanities.',
            version: 'v17.0',
            author: 'University of Chicago Press',
            license: 'CC-BY-SA 3.0',
            id: 'http://www.zotero.org/styles/chicago-note-bibliography',
            tags: ['Humanities', 'Notes'],
            color: 'from-orange-700 to-orange-500'
        }
    };

    let styleId = $derived($page.params.id);
    let style = $derived(stylesInfo[styleId] || stylesInfo['apa']);

    function handleFork() {
        // Pre-fill the intent and go to wizard
        intent.update(i => ({ ...i, base_archetype: styleId }));
        goto('/create-wizard');
    }
</script>

<main class="mx-auto max-w-[1400px] px-6 py-12">
        <!-- Breadcrumbs -->
        <nav class="flex items-center gap-2 mb-8 text-sm text-slate-400 font-medium">
            <a href="/" class="hover:text-primary transition-colors">Library</a>
            <span class="material-symbols-outlined text-sm">chevron_right</span>
            <span class="text-slate-900">{style.name}</span>
        </nav>

        <!-- Header -->
        <div class="flex flex-col lg:flex-row lg:items-end justify-between gap-8 pb-10 border-b border-slate-200">
            <div class="max-w-3xl">
                <div class="flex items-center gap-3 mb-4">
                    <span class="px-2.5 py-0.5 rounded-full bg-blue-50 text-blue-600 text-[10px] font-black uppercase tracking-wider border border-blue-100">Verified</span>
                    {#each style.tags as tag}
                        <span class="px-2.5 py-0.5 rounded-full bg-slate-100 text-slate-500 text-[10px] font-black uppercase tracking-wider">{tag}</span>
                    {/each}
                </div>
                <h1 class="text-5xl font-black text-slate-950 tracking-tight leading-none mb-4">
                    {style.name}
                </h1>
                <p class="text-xl text-slate-500 leading-relaxed font-medium">
                    {style.description}
                </p>
            </div>
            <div class="flex gap-3">
                <button 
                    onclick={() => toggleBookmark(styleId)}
                    class="flex items-center gap-2 h-12 px-6 rounded-xl border border-slate-200 {$bookmarks.includes(styleId) ? 'text-primary bg-blue-50 border-primary/20' : 'bg-white text-slate-950'} font-bold hover:bg-slate-50 transition-all"
                >
                    <span class="material-symbols-outlined {$bookmarks.includes(styleId) ? 'fill-1' : ''}">
                        {$bookmarks.includes(styleId) ? 'bookmark' : 'star'}
                    </span>
                    {$bookmarks.includes(styleId) ? 'Saved' : 'Star'}
                </button>
                <button 
                    onclick={handleFork}
                    class="flex items-center gap-2 h-12 px-6 rounded-xl bg-slate-950 text-white font-bold hover:bg-slate-800 transition-all shadow-xl shadow-slate-200"
                >
                    <span class="material-symbols-outlined">fork_right</span>
                    Fork & Edit
                </button>
            </div>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-12 gap-12 mt-12">
            <!-- Sidebar -->
            <aside class="lg:col-span-4 flex flex-col gap-8">
                <div class="rounded-3xl border border-slate-200 bg-white p-8 shadow-sm">
                    <div class="flex flex-col gap-6">
                        <div>
                            <h3 class="text-[10px] font-black uppercase tracking-widest text-slate-400 mb-4">Current Version</h3>
                            <div class="flex items-baseline gap-2">
                                <span class="text-4xl font-black text-slate-950">{style.version}</span>
                                <span class="px-2 py-0.5 rounded-md bg-emerald-50 text-emerald-600 text-[10px] font-black uppercase tracking-wider border border-emerald-100">Stable</span>
                            </div>
                            <p class="text-sm text-slate-400 font-medium mt-1 text-slate-500">Updated Feb 03, 2024</p>
                        </div>
                        
                        <div class="space-y-3">
                            <button class="w-full flex items-center justify-center gap-2 h-11 bg-primary text-white font-bold rounded-xl hover:bg-blue-600 transition-all shadow-lg shadow-blue-100">
                                <span class="material-symbols-outlined text-[20px]">download</span>
                                Download CSL
                            </button>
                            <button class="w-full flex items-center justify-center gap-2 h-11 bg-white border border-slate-200 text-slate-950 font-bold rounded-xl hover:bg-slate-50 transition-all">
                                <span class="material-symbols-outlined text-[20px]">code</span>
                                Copy JSON
                            </button>
                        </div>

                        <div class="pt-6 border-t border-slate-100 space-y-4">
                            <div class="flex items-start gap-3">
                                <span class="material-symbols-outlined text-slate-300">verified_user</span>
                                <div>
                                    <p class="text-xs font-black uppercase tracking-widest text-slate-400">Maintained by</p>
                                    <p class="font-bold text-slate-900">{style.author}</p>
                                </div>
                            </div>
                            <div class="flex items-start gap-3">
                                <span class="material-symbols-outlined text-slate-300">policy</span>
                                <div>
                                    <p class="text-xs font-black uppercase tracking-widest text-slate-400">License</p>
                                    <p class="font-bold text-slate-900">{style.license}</p>
                                </div>
                            </div>
                            <div class="flex items-start gap-3">
                                <span class="material-symbols-outlined text-slate-300">link</span>
                                <div class="overflow-hidden">
                                    <p class="text-xs font-black uppercase tracking-widest text-slate-400">Canonical ID</p>
                                    <p class="font-mono text-[10px] text-slate-500 truncate mt-1">{style.id}</p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="rounded-3xl border border-slate-200 bg-white p-8 shadow-sm">
                    <h3 class="text-[10px] font-black uppercase tracking-widest text-slate-400 mb-4">Compatibility</h3>
                    <div class="flex flex-wrap gap-2">
                        {#each ['Zotero', 'Mendeley', 'Papers', 'RefWorks'] as tool}
                            <span class="px-3 py-1.5 rounded-lg bg-slate-50 border border-slate-100 text-slate-900 text-xs font-bold">{tool}</span>
                        {/each}
                    </div>
                </div>
            </aside>

            <!-- Previews / Stress Tests -->
            <section class="lg:col-span-8 space-y-8">
                <div class="flex items-center justify-between mb-4">
                    <h2 class="text-2xl font-black text-slate-950 tracking-tight">Citation stress tests</h2>
                    <div class="flex gap-2">
                        <button class="px-4 py-2 rounded-lg bg-slate-200 text-slate-900 text-xs font-bold">All Tests</button>
                        <button class="px-4 py-2 rounded-lg bg-white border border-slate-100 text-slate-500 text-xs font-bold hover:bg-slate-50">Books</button>
                    </div>
                </div>

                <!-- Test Card 1 -->
                <article class="rounded-3xl border border-slate-200 bg-white overflow-hidden shadow-sm hover:shadow-md transition-shadow">
                    <div class="px-8 py-5 bg-slate-50/50 border-b border-slate-100 flex items-center justify-between">
                        <div class="flex items-center gap-3">
                            <div class="w-2 h-2 rounded-full bg-emerald-500"></div>
                            <h3 class="font-bold text-slate-900">Standard Journal Article</h3>
                        </div>
                        <span class="text-[10px] font-mono text-slate-400">article-journal</span>
                    </div>
                    <div class="p-8 space-y-8">
                        <div>
                            <p class="text-[10px] font-black uppercase tracking-widest text-slate-400 mb-4">In-Text Citation</p>
                            <div class="p-6 rounded-2xl bg-slate-50 font-serif text-lg leading-relaxed text-slate-800 border border-slate-100">
                                <span class="text-slate-300">...as demonstrated in </span>
                                <span class="bg-blue-600/10 text-primary px-1 rounded mx-0.5 border-b border-primary/20">
                                    {#if styleId === 'apa'}
                                        (Doe et al. 2024)
                                    {:else if styleId === 'nature'}
                                        <sup>1</sup>
                                    {:else}
                                        Doe (2024)
                                    {/if}
                                </span>
                                <span class="text-slate-300">, the discovery confirmed...</span>
                            </div>
                        </div>
                        <div>
                            <div class="flex items-center justify-between mb-4">
                                <p class="text-[10px] font-black uppercase tracking-widest text-slate-400">Bibliography Entry</p>
                                <button class="text-xs font-bold text-primary hover:text-blue-700 flex items-center gap-1">
                                    <span class="material-symbols-outlined text-sm">content_copy</span> Copy
                                </button>
                            </div>
                            <div class="p-6 rounded-2xl bg-slate-50 font-serif text-lg leading-relaxed text-slate-800 border border-slate-100">
                                {#if styleId === 'apa'}
                                    Doe, J., Smith, R., & Jones, A. (2024). <i>The Future of Citation.</i> Academic Press. https://doi.org/10.1000/182
                                {:else if styleId === 'nature'}
                                    1. Doe, J., Smith, R. & Jones, A. The Future of Citation. <i>Academic Press</i> (2024).
                                {:else}
                                    Doe, J., Smith, R., and Jones, A. 2024. <i>The Future of Citation.</i> New York: Academic Press.
                                {/if}
                            </div>
                        </div>
                    </div>
                </article>

                <!-- Test Card 2: Book -->
                <article class="rounded-3xl border border-slate-200 bg-white overflow-hidden shadow-sm hover:shadow-md transition-shadow">
                    <div class="px-8 py-5 bg-slate-50/50 border-b border-slate-100 flex items-center justify-between">
                        <div class="flex items-center gap-3">
                            <div class="w-2 h-2 rounded-full bg-orange-500"></div>
                            <h3 class="font-bold text-slate-900">Standard Book</h3>
                        </div>
                        <span class="text-[10px] font-mono text-slate-400">book</span>
                    </div>
                    <div class="p-8 space-y-8">
                        <div>
                            <p class="text-[10px] font-black uppercase tracking-widest text-slate-400 mb-4">In-Text Citation</p>
                            <div class="p-6 rounded-2xl bg-slate-50 font-serif text-lg leading-relaxed text-slate-800 border border-slate-100">
                                <span class="bg-blue-600/10 text-primary px-1 rounded mx-0.5 border-b border-primary/20">
                                    {#if styleId === 'apa'}
                                        (Foucault, 1977)
                                    {:else if styleId === 'nature'}
                                        <sup>2</sup>
                                    {:else}
                                        Foucault (1977)
                                    {/if}
                                </span>
                            </div>
                        </div>
                        <div>
                            <p class="text-[10px] font-black uppercase tracking-widest text-slate-400 mb-4">Bibliography Entry</p>
                            <div class="p-6 rounded-2xl bg-slate-50 font-serif text-lg leading-relaxed text-slate-800 border border-slate-100">
                                {#if styleId === 'apa'}
                                    Foucault, M. (1977). <i>Discipline and Punish: The Birth of the Prison</i>. Pantheon Books.
                                {:else if styleId === 'nature'}
                                    2. Foucault, M. <i>Discipline and Punish: The Birth of the Prison</i>. (Pantheon Books, 1977).
                                {:else}
                                    Foucault, Michel. 1977. <i>Discipline and Punish: The Birth of the Prison</i>. New York: Pantheon Books.
                                {/if}
                            </div>
                        </div>
                    </div>
                </article>

                <!-- Test Card 3: Chapter -->
                <article class="rounded-3xl border border-slate-200 bg-white overflow-hidden shadow-sm hover:shadow-md transition-shadow">
                    <div class="px-8 py-5 bg-slate-50/50 border-b border-slate-100 flex items-center justify-between">
                        <div class="flex items-center gap-3">
                            <div class="w-2 h-2 rounded-full bg-blue-500"></div>
                            <h3 class="font-bold text-slate-900">Chapter in Edited Book</h3>
                        </div>
                        <span class="text-[10px] font-mono text-slate-400">chapter</span>
                    </div>
                    <div class="p-8 space-y-8">
                        <div>
                            <p class="text-[10px] font-black uppercase tracking-widest text-slate-400 mb-4">In-Text Citation</p>
                            <div class="p-6 rounded-2xl bg-slate-50 font-serif text-lg leading-relaxed text-slate-800 border border-slate-100">
                                <span class="bg-blue-600/10 text-primary px-1 rounded mx-0.5 border-b border-primary/20">
                                    {#if styleId === 'apa'}
                                        (Gould, 1991)
                                    {:else if styleId === 'nature'}
                                        <sup>3</sup>
                                    {:else}
                                        Gould (1991)
                                    {/if}
                                </span>
                            </div>
                        </div>
                        <div>
                            <p class="text-[10px] font-black uppercase tracking-widest text-slate-400 mb-4">Bibliography Entry</p>
                            <div class="p-6 rounded-2xl bg-slate-50 font-serif text-lg leading-relaxed text-slate-800 border border-slate-100">
                                {#if styleId === 'apa'}
                                    Gould, G. (1991). Streisand as Schwarzkopf. In T. Page (Ed.), <i>The Glenn Gould Reader</i> (pp. 308–311). Vintage.
                                {:else if styleId === 'nature'}
                                    3. Gould, G. in <i>The Glenn Gould Reader</i> (ed. Page, T.) 308–311 (Vintage, 1991).
                                {:else}
                                    Gould, Glenn. 1991. “Streisand as Schwarzkopf.” In <i>The Glenn Gould Reader</i>, edited by Tim Page, 308–11. New York: Vintage.
                                {/if}
                            </div>
                        </div>
                    </div>
                </article>
            </section>
        </div>
</main>
