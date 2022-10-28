<script lang="ts">
    import Leftbar from './lib/Leftbar.svelte';
    import Rightbar from './lib/Rightbar.svelte';
    import * as wasm from './wasm/packet_analyzer.js'
    import './app.css'
    import PacketTable from './lib/PacketTable/PacketTable.svelte';
    import { replay } from './store';
</script>

<main class="flex w-full h-screen">
  <div class="w-full flex h-full">
    <Leftbar />
    <div class="divider"></div>
    {#if $replay != null}
    <div class="px-8 w-full h-full flex items-center justify-center">
      <PacketTable packets={$replay.packets.filter(packet => packet.packet_type != 10 && packet.packet_type != 7)}/>
    </div>
    <Rightbar packets={$replay.packets}/>
    {:else}
    <div>Load Replays</div>
    {/if}
  </div>
</main>