export const sleep = async (timeout: number) => {
	return new Promise<void>((resolve) => {
		setTimeout(resolve, timeout);
	});
};
