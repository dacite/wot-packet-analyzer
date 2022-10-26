export interface PacketOverview {
    packet_type: number
    count: number
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
}
