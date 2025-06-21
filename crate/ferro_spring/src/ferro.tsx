import type { SpringConfig } from "react-spring";
import { Controller } from "react-spring";

export const REGISTRAR: Map<number, Spring> = new Map();


/// extern

export function register(initialValue: number, config?: SpringConfig): number {
    return Spring(initialValue, config).key();
}

export function get(key: number): number | undefined {
    return REGISTRAR.get(key)?.get();
}

export function set(key: number, value: number, config?: SpringConfig) {
    return REGISTRAR.get(key)?.set(value, config);
}

export function cleanup(key: number) {
    REGISTRAR.get(key)?.cleanup();
}


type Spring = {
    key(): number;
    get(): number;
    set(value: number, config?: SpringConfig): void;
    cleanup(): void;
};

function Spring(_initialValue: number, _config?: SpringConfig): Spring {
    let _key: number = Spring.genKey();
    let _now: number = _initialValue;
    let _controller: Controller<{ value: number }> = new Controller<{ value: number }>({
        from: { value: _initialValue },
        onChange: result => {
            if (result.value?.value != null) {
                _now = result.value.value;
            }
        }
    });

    /** @constructor */ {
        let spring: Spring = { key, get, set, cleanup };
        REGISTRAR.set(_key, spring);
        return spring;
    }

    function key(): number {
        return _key;
    }

    function get(): number {
        return _now;
    }

    function set(value: number, config?: SpringConfig) {
        _controller.start({ value, config });
    }

    function cleanup() {
        _controller.stop();
        REGISTRAR.delete(key());
    }
}

namespace Spring {
    /// Possible memory leak if this gets too big needs to be handled.
    /// Should check if any smaller numbers have been freed.
    let nextKey: number = 0;

    export function genKey(): number {
        return nextKey++;
    }
}
