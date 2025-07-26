import * as reliq from "reliq";
import {KeyPair} from "./key_pair";
import * as protocol from "./protocol";
import * as source from "./source";

export type Account = {
    address: string,
    key_pair: KeyPair,
    name: reliq.Option<string>,
    protocol: reliq.Option<protocol.Protocol>,
    source: reliq.Option<source.Source>
}

export namespace Account {
    
}