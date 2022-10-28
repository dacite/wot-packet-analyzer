/* tslint:disable */
/* eslint-disable */
/**
* @param {Uint8Array} replay
* @returns {any}
*/
export function parse_packets(replay: Uint8Array): any;
/**
* @param {Uint8Array} stream
* @returns {string}
*/
export function parse_pickle_stream(stream: Uint8Array): string;
/**
*/
export class PacketSummary {
  free(): void;
}
