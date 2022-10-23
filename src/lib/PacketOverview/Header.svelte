<script lang="ts">
    import { createSvelteTable, flexRender, getCoreRowModel } from "@tanstack/svelte-table";
    export let table;
</script>

<thead>
    {#each $table.getHeaderGroups() as headerGroup}
        <tr>
            {#each headerGroup.headers as header}
                <th>
                    {#if !header.isPlaceholder}
                        <svelte:component
                            this={flexRender(
                                header.column.columnDef.header,
                                header.getContext()
                            )}
                        />
                    {/if}
                </th>
            {/each}
        </tr>
    {/each}
</thead>
