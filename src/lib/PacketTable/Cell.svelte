<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { toHex } from "../../utils";
    import { selectionRange, selectionStatus } from "../../store";

    export let data;
    export let offset;
    export let cellType: string;
    export let isSelected;
    export let packetIndex
    let isInSelectionRange = false;
    function cellBackground(offset: number, cellType: string) {
        if (offset < 4) return "bg-slate-500 text-gray-200";
        if (offset < 8) return "bg-slate-600 text-gray-200";
        if (offset < 12) return "bg-slate-500 text-gray-200";

        if (cellType === "NORMAL") return "";
        if (cellType === "PICKLE") return "bg-slate-300 text-gray-800";
        if (cellType === "ZLIB") return "bg-green-300 text-gray-800";
    }

    const dispatch = createEventDispatcher();
    function onMouseMove(event) {
        if ($selectionStatus) {
            dispatch("cellMouseMove", {
                offset: offset,
            });
        }
    }

    selectionRange.subscribe(val => {
        if (val == null) {
            isInSelectionRange = false
        } else if (val.start.packet.index == packetIndex) {
            if (
                val.start.cell_offset <= offset &&
                val.end.cell_offset >= offset
                ) {
                isInSelectionRange = true;
            } else {
                isInSelectionRange = false
            }
        } else {
            isInSelectionRange = false ;
        }
    })

    function cellClick() {
        dispatch("cellClick", {
            offset: offset,
        });
    }
    let color = cellBackground(offset, cellType);

    const selectionBackground = "bg-blue-300"
</script>

<span
    class={" select-none flex items-center h-[20px] px-1 hover:bg-slate-700 hover:cursor-default font-mono selection:bg-slate-800 text-gray-200 " +
        color}
    class:selected={isSelected}
    class:selectionRange={isInSelectionRange}
    on:click={cellClick}
    on:mousedown={() => {
        $selectionRange = null;
        selectionStatus.set(true);
    }}
    on:mouseup={() => selectionStatus.set(false)}
    on:mousemove={onMouseMove}
>
    {toHex(data)}
</span>

<style lang="postcss">
    .selected {
        -webkit-box-shadow: inset 0px 0px 0px 2px rgb(144, 160, 175);
        -moz-box-shadow: inset 0px 0px 0px 2px rgb(144, 160, 175);
        box-shadow: inset 0px 0px 0px 2px rgb(144, 160, 175);
    }
    .selectionRange {
        @apply bg-slate-600 text-gray-700;
    }
</style>
