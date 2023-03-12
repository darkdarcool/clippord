
/**
 * Gets the users current copied data from their clipboard
 * Returns as a string
 */
function getClipboard(): string;
/**
 * Copies the given text to the users clipboard
 * @param text The text to be copied to the clipboard
 */
function setClipboard(text: string): undefined;

export { getClipboard, setClipboard };