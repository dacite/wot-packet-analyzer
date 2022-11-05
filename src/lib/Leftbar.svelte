<script lang="ts">
    import PacketOverview from "./PacketOverview/PacketOverview.svelte";
    import * as replayParser from "../wasm/packet_analyzer";
    import { replay, replayFile } from "../store";
    import Dataview from "./Rightbar/Dataview.svelte";
    let files;

    $: if (files) {
        for (const file of files) {
            file.arrayBuffer().then((item) => {
                const array = new Uint8Array(item);
                let startTime = performance.now();
                const result = replayParser.parse_packets(array);
                console.log(file.name)
                let endTime = performance.now();
                console.log(
                    `Call to parsePackets took ${
                        endTime - startTime
                    } milliseconds`
                );
                replayFile.set(file.name)
                replay.update(value => {
                    value = result
                    return value
                })
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
            <div class="flex"><span class="font-bold min-w-[70px] block">Tank: </span>{$replay.tank}</div>
            <div class="flex"><span class="font-bold min-w-[70px] block">Map: </span>{$replay.map}</div>
            <div class="flex"><span class="font-bold min-w-[70px] block">Version: </span>{$replay.version}</div>
            <div class="flex"><span class="font-bold min-w-[70px] block">Time: </span>{$replay.time}</div>
        {/if}
    </div>
    <Dataview />
    <div class="mt-8 heading">Packet Overview</div>
    {#if $replay != null}
        <PacketOverview replay={$replay}/>
    {/if}
</div>

