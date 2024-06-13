<script lang="ts">
    let { show_modal } = $props<{ show_modal: boolean }>();

    let dialog: HTMLDialogElement;

    $effect(() => {
        dialog && show_modal && dialog.showModal();
    });
</script>

<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-noninteractive-element-interactions -->
<dialog
    bind:this={dialog}
    on:close={() => (show_modal = false)}
    on:click|self={() => dialog.close()}
    class="rounded backdrop:bg-[#000000dc] p-2 bg-transparent"
>
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div on:click|stopPropagation class="relative max-w-xl w-full">
        <button on:click={() => dialog.close()}>
            <slot name="button" />
        </button>

        <slot><!-- optional fallback --></slot>
    </div>
</dialog>

<style lang="postcss">
    button {
        @apply absolute size-10 right-8 top-8 bg-gray-950 text-white text-base rounded;
    }

    /* Dialog Responsive Applied Here */

    dialog[open] {
        animation: zoom 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
    }

    dialog[open]::backdrop {
        animation: fade 0.2s ease-out;
    }

    @keyframes zoom {
        from {
            transform: scale(0.95);
        }

        to {
            transform: scale(1);
        }
    }

    @keyframes fade {
        from {
            opacity: 0;
        }

        to {
            opacity: 1;
        }
    }
</style>
