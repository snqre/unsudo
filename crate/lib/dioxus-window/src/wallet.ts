

export type OnAnimationFrame = () => boolean;
export type KillSwitch = () => void;

export function on_animation_frame(event_handler: OnAnimationFrame): KillSwitch {
	let polling: boolean = true;
	
	let update = () => {
		if (!event_handler() || !polling) {
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

on_animation_frame(() => {
	
	return true;
});