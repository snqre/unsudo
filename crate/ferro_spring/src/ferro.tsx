import { type ReactNode } from "react";
import { useRef } from "react";
import { useEffect } from "react";
import { useSpring } from "react-spring";
import { animated } from "react-spring";
import { SpringRef } from "react-spring";

const SPRING_CONTROLLERS: Record<string, SpringRef<any>> = {};

(window as any).animate_with_react_spring = (key: string, to: any) => {
    let controller: SpringRef<any> = SPRING_CONTROLLERS[key];
    if (controller) {
        controller.start(to);
    } else {
        console.warn(`No spring controller found for key=${key}`);
    }
}

function Bridge(props: Bridge.Props): ReactNode {
    let spring_ref = useRef<SpringRef<any>>(null!);
    let [style, animation] = useSpring(() => ({
        ref: SpringRef,
        from: { opacity: 0 },
        config: {
            
        }
    }));

    useEffect(() => {
        SPRING_CONTROLLERS[props.key] = animation;
        return () => {
            delete SPRING_CONTROLLERS[props.key];
        };
    }, [props.key]);

    return <>
        <animated.div key={props.key} style={style}>{ props.children }</animated.div>
    </>
}

namespace Bridge {

    export type Props = {
        key: string;
        children: ReactNode;
    }
}