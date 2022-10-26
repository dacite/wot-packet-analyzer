import { createColumnHelper } from "@tanstack/svelte-table";
import type { PacketOverview } from "src/def";
import { toHex } from "../../utils";

const columnHelper = createColumnHelper<PacketOverview>()

export const packetOverviewColumns = [
    columnHelper.accessor('packet_type', {
        header: "Type",
        cell: info => `0x${toHex(info.getValue())}`
    }),
    columnHelper.accessor('count', {
        header: "Count",
        cell: info => info.getValue()
    })
]

export const packetOverviewData: PacketOverview[] = [
    {
        packet_type: 1,
        count: 43543,
    },
    {
        packet_type: 2,
        count: 43543,
    }
]