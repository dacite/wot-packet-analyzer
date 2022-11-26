/* tslint:disable */
/* eslint-disable */
/**
* @param {Uint8Array} replay
* @returns {any}
*/
export function parse_packets(replay: Uint8Array): any;
/**
* @param {Uint8Array} needle
* @param {number} from
* @returns {any}
*/
export function search_value(needle: Uint8Array, from: number): any;
/**
* @returns {string}
*/
export function get_json(): string;
/**
* @param {Uint8Array} stream
* @returns {string}
*/
export function decompress_and_parse_pickle_stream(stream: Uint8Array): string;
/**
* @param {Uint8Array} stream
* @returns {string}
*/
export function parse_pickle_stream(stream: Uint8Array): string;
/**
*/
export class PacketSearchResult {
  free(): void;
/**
*/
  offset: number;
/**
*/
  packet_id: number;
}
/**
*/
export class PacketSummary {
  free(): void;
}
