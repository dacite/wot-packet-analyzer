<script lang="ts">
    import { f32, f64, i16, i32, i64, u16, u32, u64, u8 } from "../utils";
    import { goToIndex, selectionRange } from "./../store";
    import * as replayParser from "../wasm/packet_analyzer";
    import { selection, unPickleResult, unpickleOnClick } from "./../store";
    import type { Packet } from "../def";
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

    const CONVERSIONS = [
        { text: "U08", func: u8 },
        { text: "U16", func: u16 },
        { text: "I16", func: i16 },
        { text: "U32", func: u32 },
        { text: "I32", func: i32 },
        { text: "F32", func: f32 },
        { text: "U64", func: u64 },
        { text: "I64", func: i64 },
        { text: "F64", func: f64 },
    ];
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
    <div class="my-5 heading">Data View</div>
    {#if $selection != null}
        {#each CONVERSIONS as { text, func }}
            <div class="font-mono">
                <span class="font-bold">{`${text}: `}</span>{display(
                    func($selection.packet, $selection.cell_offset)
                )}
            </div>
        {/each}
    {/if}
    <div class="my-5 heading">Selection Details</div>
    {#if $selectionRange}
        <div>
            Selection Size: <span
                >{$selectionRange.end.cell_offset +
                    1 -
                    $selectionRange.start.cell_offset}</span
            >
        </div>
    {/if}
    <div class="flex flex-col gap-3">
        <div class="flex gap-2">
            <input
                type="checkbox"
                bind:checked={$unpickleOnClick}
                class="checkbox"
            /> Unpickle on click
        </div>
        <button
            class="btn"
            class:btn-disabled={$selectionRange == null}
            on:click={() => {
                const packet = $selectionRange.start.packet;
                const pickleStream = packet.data.slice(
                    $selectionRange.start.cell_offset,
                    $selectionRange.end.cell_offset + 1
                );
                const array = new Uint8Array(pickleStream);

                try {
                    const result = replayParser.parse_pickle_stream(array);
                    $unPickleResult = result;
                } catch (err) {
                    $unPickleResult = err;
                }
            }}>Unpickle</button
        >
        <button
            class="btn"
            class:btn-disabled={$selectionRange == null}
            on:click={() => {
                const packet = $selectionRange.start.packet;
                const pickleStream = packet.data.slice(
                    $selectionRange.start.cell_offset,
                    $selectionRange.end.cell_offset + 1
                );
                const array = new Uint8Array(pickleStream);

                try {
                    const result =
                        replayParser.decompress_and_parse_pickle_stream(array);
                    $unPickleResult = result;
                } catch (err) {
                    $unPickleResult = err;
                }
            }}>Decompress & Unpickle</button
        >
    </div>
    <textarea
        bind:value={$unPickleResult}
        class="my-4 textarea textarea-bordered text-400 w-full h-[400px] font-mono text-sm"
        placeholder="Result"
    />
</div>
