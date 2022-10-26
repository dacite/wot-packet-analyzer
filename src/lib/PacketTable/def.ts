import { createColumnHelper } from "@tanstack/svelte-table";
    import type { Packet } from "../../def"


const columnHelper = createColumnHelper<Packet>()

export const packetOverviewColumns = [
    columnHelper.accessor('data', {
        cell: info => info.row.original
    }),
]

export const packetOverviewData = [
    {
        packetType: 1,
        count: 43543,
    },
    {
        packetType: 2,
        count: 43543,
    }
]