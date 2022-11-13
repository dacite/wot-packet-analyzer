<script lang="ts">
    import { f32, f64, i16, i32, i64, u16, u32, u64, u8 } from "../../utils";
    import { selection, replay } from "../../store";
    import type { Packet } from "../../def";

    function display(item: number | null | bigint | string) {
        if (item == null) {
            return "";
        } else {
            return item;
        }
    }

    function player(packet: Packet, offset: number) {
        const number = i32(packet, offset);

        const playerInfo = $replay.players.get(number);

        if (playerInfo == null) {
            return "";
        } else {
            return playerInfo;
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
        { text: "Offset", func: (packet: Packet, offset: number) => offset },
        { text: "Player", func: player },
    ];
</script>

<div class="my-5 heading">Data View</div>
{#if $selection != null}
    {#each CONVERSIONS as { text, func }}
        <div class="pl-3 font-mono">
            <span class="font-bold">{`${text}: `}</span>{display(
                func($selection.packet, $selection.cell_offset)
            )}
        </div>
    {/each}
{/if}
