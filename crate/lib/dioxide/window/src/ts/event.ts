export type EventHandler = () => void;
export type KillSwitch = () => void;

export function on_animation_frame(event_handler: EventHandler): KillSwitch {
	let polling: boolean = true;
	
	let update = () => {
		if (!polling) {
			return;
		}

		// @ts-ignore
		window.requestAnimationFrame(update);
	};

	// @ts-ignore
	window.requestAnimationFrame(update);
	
	return () => {
		if (polling) {
			polling = false;
		}
	};
}

function on_interval(ms: bigint, event_handler: EventHandler): KillSwitch {
    let timer: NodeJS.Timeout = setInterval(() => {
        event_handler();
    }, Number(ms));
    return () => clearInterval(timer);
}

function on_timeout(ms: bigint, event_handler: EventHandler) {
    setTimeout(() => {
        event_handler();
    }, Number(ms));
}