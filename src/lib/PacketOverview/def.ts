import { createColumnHelper } from "@tanstack/svelte-table";

export interface PacketOverview {
    packetType: number
    count: number
}

const columnHelper = createColumnHelper<PacketOverview>()

export const packetOverviewColumns = [
    columnHelper.accessor('packetType', {
        header: "Type",
        cell: info => info.getValue()
    }),
    columnHelper.accessor('count', {
        header: "Count",
        cell: info => info.getValue()
    })
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