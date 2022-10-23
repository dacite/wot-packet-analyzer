import { getCoreRowModel, type TableOptions } from "@tanstack/svelte-table";
import { writable } from "svelte/store";
import { packetOverviewColumns, packetOverviewData, type PacketOverview } from "./lib/PacketOverview/def";

export const packetOverviewTable = writable<TableOptions<PacketOverview>>({
    data: packetOverviewData,
    columns: packetOverviewColumns,
    getCoreRowModel: getCoreRowModel(),
});