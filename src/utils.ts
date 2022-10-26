export function toHex(number: number, withTag?: boolean) {
    if (number == null) return "XX";
    const hex = number.toString(16).toUpperCase().padStart(2, "0");
    if (withTag) {
       return "0x" + hex;
    } else {
       return hex;
    }
}