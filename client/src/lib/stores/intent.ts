import { writable } from 'svelte/store';
import type { StyleIntent } from '../types/bindings';

const initialState: StyleIntent = {
    base_archetype: null,
    field: null,
    class: null,
    citation_preset: null,
    bibliography_preset: null,
    detailed_config: null,
    author_format: null,
    has_bibliography: null
};

export const intent = writable<StyleIntent>(initialState);

export function resetIntent() {
    intent.set(initialState);
}
