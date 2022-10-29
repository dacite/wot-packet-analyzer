import type { Packet } from "./def";

export function toHex(number: number, withTag?: boolean) {
    if (number == null) return "XX";
    const hex = number.toString(16).toUpperCase().padStart(2, "0");
    if (withTag) {
       return "0x" + hex;
    } else {
       return hex;
    }
}

export function u8(packet: Packet, offset: number) {
   const slice = packet.data.slice(offset, offset + 1)

   if (slice.length == 1) {
      const view = new DataView(new Uint8Array(slice).buffer)
      
      return view.getUint8(0)
   } else {
      return null
   }
}

export function i16(packet: Packet, offset: number) {
   const slice = packet.data.slice(offset, offset + 2)

   if (slice.length == 2) {
      const view = new DataView(new Uint8Array(slice).buffer)
      
      return view.getInt16(0, true)
   } else {
      return null
   }
}

export function u16(packet: Packet, offset: number) {
   const slice = packet.data.slice(offset, offset + 2)

   if (slice.length == 2) {
      const view = new DataView(new Uint8Array(slice).buffer)
      
      return view.getUint16(0, true)
   } else {
      return null
   }
}

export function i32(packet: Packet, offset: number) {
   const slice = packet.data.slice(offset, offset + 4)

   if (slice.length == 4) {
      const view = new DataView(new Uint8Array(slice).buffer)
      
      return view.getInt32(0, true)
   } else {
      return null
   }
}

export function u32(packet: Packet, offset: number) {
   const slice = packet.data.slice(offset, offset + 4)

   if (slice.length == 4) {
      const view = new DataView(new Uint8Array(slice).buffer)
      
      return view.getUint32(0, true)
   } else {
      return null
   }
}

export function f32(packet: Packet, offset: number) {
   const slice = packet.data.slice(offset, offset + 4)

   if (slice.length == 4) {
      const view = new DataView(new Uint8Array(slice).buffer)
      
      return view.getFloat32(0, true)
   } else {
      return null
   }
}


export function i64(packet: Packet, offset: number) {
   const slice = packet.data.slice(offset, offset + 8)

   if (slice.length == 8) {
      const view = new DataView(new Uint8Array(slice).buffer)
      
      return view.getBigInt64(0, true)
   } else {
      return null
   }
}

export function u64(packet: Packet, offset: number) {
   const slice = packet.data.slice(offset, offset + 8)

   if (slice.length == 8) {
      const view = new DataView(new Uint8Array(slice).buffer)
      
      return view.getBigUint64(0, true)
   } else {
      return null
   }
}

export function f64(packet: Packet, offset: number) {
   const slice = packet.data.slice(offset, offset + 8)

   if (slice.length == 8) {
      const view = new DataView(new Uint8Array(slice).buffer)
      
      return view.getFloat64(0, true)
   } else {
      return null
   }
}
