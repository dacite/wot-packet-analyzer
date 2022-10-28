<script lang="ts">
  import type { Packet as PacketType } from "src/def";
  import { toHex } from "../../utils";
  import Cell from "./Cell.svelte";
  export let packet: PacketType;
  import * as replayParser from "../../wasm/packet_analyzer";
  import { selection } from "../../store";
  const packetType = toHex(packet.packet_type);
  const length = `${packet.data.length} bytes`;

  function indexInPickle(index, packet: PacketType) {
    for (let range of packet.packet_segments.pickles) {
      if (range[0] <= index && range[1] >= index) return range;
    }
    return null;
  }
  function indexInZlib(index, packet) {
    for (let range of packet.packet_segments.zlibs) {
      if (range == index || range + 1 == index) return true;
    }
    return null;
  }
  function cellType(index, packet) {
    if (indexInPickle(index, packet) != null) return "PICKLE";
    if (indexInZlib(index, packet)) return "ZLIB";

    return "NORMAL";
  }
  function handleCellClick(event) {
    selection.set({ packet, cell_offset: event.detail.offset });
    const range = indexInPickle(event.detail.offset, packet);
    if (range != null) {
      const pickleStream = packet.data.slice(range[0], range[1] + 1);
      const array = new Uint8Array(pickleStream);
      const result = replayParser.parse_pickle_stream(array);
      console.log(result);
    }
  }
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
        <Cell
          data={byte}
          offset={idx}
          cellType={cellType(idx, packet)}
          on:cellClick={handleCellClick}
        />
      {/each}
    </div>
  </div>
</div>
