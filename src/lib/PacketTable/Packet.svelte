<script lang="ts">
  import type { Packet as PacketType } from "src/def";
  import { toHex } from "../../utils";
  import Cell from "./Cell.svelte";
  export let packet: PacketType;
  import * as replayParser from "../../wasm/packet_analyzer";
  import {
    selection,
    selectionStatus,
    selectionRange,
    unpickleOnClick,
    unPickleResult,
    goToIndex
  } from "../../store";
  let packetType = toHex(packet.packet_type);
  let length = `${packet.data.length} bytes`;

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
    if (
      $selection != null &&
      $selection.packet.index == packet.index &&
      $selection.cell_offset == event.detail.offset
    ) {
      selection.set(null);
    } else {
      const newSelect = { packet, cell_offset: event.detail.offset };
      selectionStatus.set(false);
      selection.set(newSelect);
    }

    if ($unpickleOnClick) {
      const range = indexInPickle(event.detail.offset, packet);
      if (range != null) {
        const pickleStream = packet.data.slice(range[0], range[1] + 1);
        const array = new Uint8Array(pickleStream);
        try {
          const result = replayParser.parse_pickle_stream(array);
          $unPickleResult = result;
        } catch (err) {
          $unPickleResult = err;
        }
      }
    }
  }
  function handleCellMouseMove(event) {
    const selection = {
      packet,
      cell_offset: event.detail.offset,
    };
    if ($selectionRange == null) {
      $selectionRange = {
        anchor: selection,
        start: selection,
        end: selection,
      };
    } else if ($selectionRange.anchor.packet.index != selection.packet.index) {
      $selectionRange = null;
    } else {
      if ($selectionRange.anchor.cell_offset > selection.cell_offset) {
        $selectionRange.end = $selectionRange.anchor;
        $selectionRange.start = selection;
      } else {
        $selectionRange.start = $selectionRange.anchor;
        $selectionRange.end = selection;
      }
    }
  }

</script>

<div class="w-full h-full">
  <div class="w-full h-full  overflow-auto relative">
    <div class="px-4 py-1 sticky top-0 left-0 rounded flex justify-between bg-gray-500" class:selectedPacket={$goToIndex == packet.index}>
      <h1 class="text-xl text-black font-bold font-mono">
        {`${packet.adjusted_time} | 0x${packetType} | ${"_Dacite"} | ${length}`}
      </h1>
      <p class="text-black">{packet.index}</p>
    </div>
    <div class="py-2 flex w-full flex-wrap">
      {#each packet.data as byte, idx}
        <Cell
          isSelected={$selection != null
            ? $selection.packet.index == packet.index &&
              $selection.cell_offset == idx
            : false}
          packetIndex={packet.index}
          data={byte}
          offset={idx}
          cellType={cellType(idx, packet)}
          on:cellClick={handleCellClick}
          on:cellMouseMove={handleCellMouseMove}
        />
      {/each}
    </div>
  </div>
</div>

<style lang="postcss">
  .selectedPacket {
    background-color: aliceblue;
  }
</style>