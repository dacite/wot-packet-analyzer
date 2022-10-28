<script lang="ts">
    import PacketOverview from "./PacketOverview/PacketOverview.svelte";
    import * as replayParser from "../wasm/packet_analyzer";
    import { replay } from "../store";
    import Packet from "./PacketTable/Packet.svelte";
    let files;

    $: if (files) {
        for (const file of files) {
            file.arrayBuffer().then((item) => {
                const array = new Uint8Array(item);
                let startTime = performance.now();
                const result = replayParser.parse_packets(array);
                console.log(result)
                let endTime = performance.now();
                console.log(
                    `Call to parsePackets took ${
                        endTime - startTime
                    } milliseconds`
                );

                replay.set(result);
            });
        }
    }
</script>

<div class="pl-3 basis-1/4">
    <input bind:files id="replaySelect" type="file" hidden />
    <input
        class="btn gap-2"
        type="button"
        value="Open Replay"
        on:click={() => document.getElementById("replaySelect").click()}
    />
    <div class="divider" />
    <div class="heading">Replay Overview</div>
    <div class="p-4 flex flex-col justify-start">
        {#if $replay == null}
            <p>Load a replay</p>
        {:else}
            <div><span class="font-bold">Tank: </span>{$replay.tank}</div>
            <div><span class="font-bold">Map: </span>{$replay.map}</div>
        {/if}
    </div>
    <div class="mt-8 heading">Packet Overview</div>
    {#if $replay != null}
        <PacketOverview packetSummary={$replay.packet_summary}/>
    {/if}
</div>
 