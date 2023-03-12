import { createRequire } from 'module'; 
import path from 'path'; 
import { fileURLToPath } from 'url';

export default function fileDirName(meta) {
  const __filename = fileURLToPath(meta.url);

  const __dirname = path.dirname(__filename);

  return { __dirname, __filename };
}

const { __dirname } = fileDirName(import.meta);

const modulesPath = path.resolve(__dirname, '../');
const require = createRequire(modulesPath + "\\");





const clippord =  require("./index.node");

const getClipboard = clippord.getClipboard;
const setClipboard = clippord.setClipboard;

export { getClipboard, setClipboard };

