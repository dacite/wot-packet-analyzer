<script lang="ts">
    import { toHex } from "../../utils";

    export let data;
    export let offset;

    function cellBackground(offset: number) {
        if (offset < 4) return "bg-slate-500 text-gray-200"
        if (offset < 8) return "bg-slate-600 text-gray-200"
        if (offset < 12) return "bg-slate-500 text-gray-200"
    }

    const color = cellBackground(offset)
</script>

<span
    class={"flex items-center h-[20px] px-1 hover:bg-slate-700 hover:cursor-default font-mono selection:bg-slate-800 text-gray-200 " + color}
>
    {toHex(data)}
</span>
