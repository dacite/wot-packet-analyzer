<script lang="ts">
    import { createSvelteTable, flexRender, getCoreRowModel } from "@tanstack/svelte-table";
    import { packetOverviewTable as packetOverviewTableOptions } from '../../store'
    import Header from "./Header.svelte";
    
    const table = createSvelteTable(packetOverviewTableOptions);
</script>

<div class="p-2">
    <table>
      <Header table={table} />
      <tbody>
        {#each $table.getRowModel().rows as row}
          <tr>
            {#each row.getVisibleCells() as cell}
              <td>
                <svelte:component
                  this={flexRender(cell.column.columnDef.cell, cell.getContext())}
                />
              </td>
            {/each}
          </tr>
        {/each}
      </tbody>
    </table>
    <div class="h-4" />
</div>