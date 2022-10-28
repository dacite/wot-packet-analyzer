<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { toHex } from "../../utils";

    export let data;
    export let offset;
    export let cellType: string;

    function cellBackground(offset: number, cellType: string) {
        if (offset < 4) return "bg-slate-500 text-gray-200";
        if (offset < 8) return "bg-slate-600 text-gray-200";
        if (offset < 12) return "bg-slate-500 text-gray-200";

        if (cellType === "NORMAL") return "";
        if (cellType === "PICKLE") return "bg-slate-300 text-gray-800";
        if (cellType === "ZLIB") return "bg-green-300 text-gray-800";
    }

    const dispatch = createEventDispatcher();

    function cellClick() {
        dispatch("cellClick", {
            offset: offset,
        });
    }
    const color = cellBackground(offset, cellType);
</script>

<span
    class={"flex items-center h-[20px] px-1 hover:bg-slate-700 hover:cursor-default font-mono selection:bg-slate-800 text-gray-200 " +
        color}
    on:click={cellClick}
>
    {toHex(data)}
</span>
