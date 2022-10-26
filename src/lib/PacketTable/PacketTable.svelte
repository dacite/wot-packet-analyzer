<script lang="ts">
  import VirtualList from "svelte-tiny-virtual-list";
  let virtualList;
  import Packet from "./Packet.svelte";
  import { goToIndex } from "../../store";
  export let packets;

  const LIST_WIDTH = 900;
  const HEADER_LEN = 52;
  const CELL_HEIGHT = 20;
  const CELL_WIDTH = 26;
  function calcItemSize(listWidth, cellCount) {
    const cellsPerRow = Math.floor(listWidth / CELL_WIDTH);
    const cellsPerColumn = Math.ceil(cellCount / cellsPerRow);

    return cellsPerColumn * CELL_HEIGHT + HEADER_LEN;
  }
</script>

<VirtualList
  bind:this={virtualList}
  height={1000}
  width={`${LIST_WIDTH}px`}
  itemCount={packets.length}
  itemSize={(index) => calcItemSize(LIST_WIDTH, packets[index].data.length)}
  scrollToIndex={$goToIndex}
  scrollToAlignment="start"
  scrollToBehaviour="smooth"
>
  <div slot="item" let:index let:style {style}>
    <Packet packet={packets[index]} />
  </div>
</VirtualList>
