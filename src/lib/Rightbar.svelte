<script lang="ts">
    import type { Packet } from 'src/def';
    import { goToIndex } from './../store'
    export let packets
    let value

    function resolveTimeToPacket(packets: Packet[], time: string) {
        for(let i = 0; i < packets.length; i++) {
            if (packets[i].adjusted_time === time) {
                return i
            }
        }

        return undefined
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
                <input bind:value type="text" placeholder="Enter replay time" class="input input-bordered w-full max-w-xs" />
                <button class="btn" on:click={() => {
                    goToIndex.set(resolveTimeToPacket(packets, value))
                }}>GO</button>
            </div>
        </div>
    </div>
    
</div>