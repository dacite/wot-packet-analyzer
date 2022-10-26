<script lang="ts">
  import { createSvelteTable, flexRender } from "@tanstack/svelte-table";
  import { packetOverviewTable as packetOverviewTableOptions } from "../../store";
  import Header from "./Header.svelte";
  export let packetSummary;
  $packetOverviewTableOptions.data = packetSummary;
  const table = createSvelteTable(packetOverviewTableOptions);
</script>

<div class="p-2">
  <table class="block">
    <Header {table} />
    <tbody class="block max-h-[500px] overflow-y-scroll">
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
