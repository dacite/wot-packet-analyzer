<script lang="ts">
    import {
        f32,
        f64,
        fromHex,
        i16,
        i32,
        i64,
        u16,
        u32,
        u64,
        u8,
    } from "../../utils";
    import { goToIndex, selectionRange } from "../../store";
    import * as replayParser from "../../wasm/packet_analyzer";
    import { selection, unPickleResult, unpickleOnClick, replay } from "../../store";
    import type { Packet } from "../../def";
    export let packets;
    let replayTimeValue;
    let findPacketValue;
    let goToIndexValue;
    console.log($replay.players)
    function resolveTimeToPacket(packets: Packet[], time: string) {
        for (let i = 0; i < packets.length; i++) {
            if (packets[i].adjusted_time === time) {
                return i;
            }
        }

        return null;
    }

    function findPacket(index: number) {
        for (let i = 0; i < packets.length; i++) {
            if (packets[i].index == index) {
                return i;
            }
        }
        return null;
    }
    
    function findPacketForward(currentIndex: number, packetType: string) {
        let packetTypeDecimal = fromHex(packetType);

        for (let i = currentIndex + 1; i < packets.length; i++) {
            if (packets[i].packet_type == packetTypeDecimal) {
                return i;
            }
        }

        return null;
    }

    function findPacketBackward(currentIndex: number, packetType: string) {
        if (currentIndex == null) {
            currentIndex = packets.length;
        }
        let packetTypeDecimal = fromHex(packetType);

        for (let i = currentIndex - 1; i >= 0; i--) {
            if (packets[i].packet_type == packetTypeDecimal) {
                return i;
            }
        }

        return null;
    }

    function display(item: number | null | bigint) {
        if (item == null) {
            return "";
        } else {
            return item;
        }
    }

    function player(packet: Packet, offset: number) {
        const number = i32(packet, offset)

        const playerInfo = $replay.players.get(number)

        if (playerInfo == null) {
            return ""
        } else {
            return playerInfo
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
        { text: "Player", func: player}
    ];
</script>

<div class="basis-1/4 px-3 py-5">
    <div class="heading">Find Packet</div>
    <div class="flex flex-col w-full">
        <form class="pt-4 form-control w-full max-w-xs">
            <label class="label flex flex-col items-start gap-2">
                <span class="label-text font-mono">Go to Replay Time:</span>
                <div class="flex w-full gap-2">
                    <input
                        bind:value={replayTimeValue}
                        type="text"
                        placeholder="Enter replay time"
                        class="input input-bordered w-full max-w-xs"
                    />
                    <button
                        class="btn"
                        on:click|preventDefault={() => {
                            goToIndex.set(
                                resolveTimeToPacket(packets, replayTimeValue)
                            );
                        }}>GO</button
                    >
                </div>
            </label>
        </form>
        <form class="pt-4 form-control w-full max-w-xs">
            <label class="label flex flex-col items-start gap-2">
                <span class="label-text font-mono">Find Packet type:</span>
                <div class="flex w-full gap-2">
                    <input
                        bind:value={findPacketValue}
                        type="text"
                        placeholder="0x08"
                        class="input input-bordered w-full max-w-xs"
                    />
                    <button
                        class="btn"
                        on:click|preventDefault={() => {
                            goToIndex.set(
                                findPacketForward($goToIndex, findPacketValue)
                            );
                        }}>Next</button
                    >
                    <button
                        class="btn max-h-[10px]"
                        on:click|preventDefault={() => {
                            goToIndex.set(
                                findPacketBackward($goToIndex, findPacketValue)
                            );
                        }}>Before</button
                    >
                </div>
                <span class="label-text font-mono">Go to Packet:</span>
                <div class="flex w-full gap-2">
                    <input
                        bind:value={goToIndexValue}
                        type="text"
                        class="input input-bordered w-full max-w-xs"
                    />
                    <button
                        class="btn"
                        on:click|preventDefault={() => {
                            goToIndex.set(findPacket(goToIndexValue));
                        }}>Go</button
                    >
                </div>
                <div class="flex w-full gap-3">
                    <button
                        class="btn"
                        on:click|preventDefault={() => {
                            goToIndex.set(0);
                        }}>Go To Start</button
                    ><button
                        class="btn"
                        on:click|preventDefault={() => {
                            console.log("GOING TO", packets.length - 1);
                            goToIndex.set(packets.length - 1);
                        }}>Go To End</button
                    >
                </div>
            </label>
        </form>
    </div>
    
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
