import { getCoreRowModel, type TableOptions } from "@tanstack/svelte-table";
import { writable } from "svelte/store";
import type { ReplayData, PacketOverview } from "./def";
import { packetOverviewColumns, packetOverviewData } from "./lib/PacketOverview/def";

export const packetOverviewTable = writable<TableOptions<PacketOverview>>({
    data: packetOverviewData,
    columns: packetOverviewColumns,
    getCoreRowModel: getCoreRowModel(),
});


export const replay = writable<ReplayData | null>(null);

export const goToIndex = writable<number | undefined>(undefined)