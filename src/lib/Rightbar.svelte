<script lang="ts">
    import type { Packet } from "../def";
    import { f32, f64, i16, i32, i64, u16, u32, u64 } from "../utils";
    import { goToIndex } from "./../store";
    import { selection } from "./../store";
    export let packets;
    let value;

    function resolveTimeToPacket(packets: Packet[], time: string) {
        for (let i = 0; i < packets.length; i++) {
            if (packets[i].adjusted_time === time) {
                return i;
            }
        }

        return undefined;
    }

    function display(item: number | null | bigint) {
        if (item == null) {
            return "";
        } else {
            return item;
        }
    }
</script>

<div class="basis-1/4 px-3 py-5">
    <div class="heading">Find Packet</div>
    <div class="flex w-full">
        <div class="pt-4 form-control w-full max-w-xs">
            <label class="label">
                <span class="label-text">Go to Replay Time:</span>
            </label>
            <div class="flex w-full gap-2">
                <input
                    bind:value
                    type="text"
                    placeholder="Enter replay time"
                    class="input input-bordered w-full max-w-xs"
                />
                <button
                    class="btn"
                    on:click={() => {
                        goToIndex.set(resolveTimeToPacket(packets, value));
                    }}>GO</button
                >
            </div>
        </div>
    </div>
    <div class="my-5 heading">Selection Details</div>
    {#if $selection != null}
        <div class="font-mono"><span class="font-bold">I16: </span>{display(i16($selection.packet, $selection.cell_offset))}</div>
        <div class="font-mono"><span class="font-bold">U16: </span>{display(u16($selection.packet, $selection.cell_offset))}</div>
        <div class="font-mono"><span class="font-bold">I32: </span>{display(i32($selection.packet, $selection.cell_offset))}</div>
        <div class="font-mono"><span class="font-bold">F32: </span>{display(f32($selection.packet, $selection.cell_offset))}</div>
        <div class="font-mono"><span class="font-bold">U32: </span>{display(u32($selection.packet, $selection.cell_offset))}</div>
        <div class="font-mono"><span class="font-bold">I64: </span>{display(i64($selection.packet, $selection.cell_offset))}</div>
        <div class="font-mono"><span class="font-bold">F64: </span>{display(f64($selection.packet, $selection.cell_offset))}</div>
        <div class="font-mono"><span class="font-bold">U64: </span>{display(u64($selection.packet, $selection.cell_offset))}</div>
    {/if}
</div>
