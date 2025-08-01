import { Option } from "reliq";
import { Some } from "reliq";
import { None } from "reliq";
import { KeyPair } from "./";
import { Protocol } from "./";
import { Source } from "./";

export type Account = {
    address: string,
    key_pair: KeyPair,
    name: Option<string>,
    protocol: Option<Protocol>,
    source: Option<Source>
}
export namespace Account {

    export function from(self: Account): Account {
        return self;
    }
} 