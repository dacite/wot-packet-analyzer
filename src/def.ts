export interface PacketOverview {
    packet_type: number
    count: number
}

export interface PacketSegments {
    pickles: number[][]
    zlibs: number[]
}
export interface ReplayData {
    map: string
    packet_summary: PacketOverview[]
    packets: Packet[]
    tank: string
    time: string
    version: string
}

export interface Packet {
    data: number[]
    packet_type: number
    time: number
    adjusted_time: string
    packet_segments: PacketSegments
}

export interface PacketSelection {
    packet: Packet
    cell_offset: number
}