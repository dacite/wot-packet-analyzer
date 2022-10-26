<script lang="ts">
  import type { Packet as PacketType } from "src/def";
  import { toHex } from "../../utils";
  import Cell from "./Cell.svelte";
  console.log("HI")
  export let packet: PacketType;
  const packetType = toHex(packet.packet_type);
  const length = `${packet.data.length} bytes`;
</script>

<div class="w-full h-full">
  <div class="w-full h-full  overflow-auto relative">
    <div class="px-4 py-1 sticky top-0 left-0 bg-gray-500 rounded">
      <h1 class="text-xl text-black font-bold font-mono">
        {`${packet.adjusted_time} | 0x${packetType} | ${"_Dacite"} | ${length}`}
      </h1>
    </div>
    <div class="py-2 flex w-full flex-wrap">
      {#each packet.data as byte, idx}
        <Cell data={byte} offset={idx} />
      {/each}
    </div>
  </div>
</div>
