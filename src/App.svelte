<script lang="ts">
    import Leftbar from './lib/Leftbar.svelte';
    import PacketTable from './lib/PacketTable.svelte';
    import Rightbar from './lib/Rightbar.svelte';
    import * as wasm from './wasm/packet_analyzer.js'
    import './app.css'
  console.log(wasm)
</script>

<main class="w-full h-full">
  <div class="w-full flex h-full">
    <Leftbar />
    <PacketTable />
    <Rightbar />
  </div>
</main>