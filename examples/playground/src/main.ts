import init, { Config, transpileRomanDocument } from '../wasm/druti_wasm.js';
import { DrutiEditor } from './editor';

const byId = <T extends HTMLElement>(id: string): T => {
  const element = document.getElementById(id);
  if (!element) throw new Error(`Playground markup is missing #${id}`);
  return element as T;
};

const main = async () => {
  const status = byId<HTMLParagraphElement>('status');
  try {
    await init();
  } catch (error) {
    status.textContent = 'The engine failed to load. This page needs WebAssembly.';
    throw error;
  }
  status.textContent = 'Engine: druti-core (Rust → WebAssembly).';

  const digits = byId<HTMLInputElement>('opt-digits');
  const dari = byId<HTMLInputElement>('opt-dari');
  const quotes = byId<HTMLInputElement>('opt-quotes');
  const autocorrect = byId<HTMLInputElement>('opt-autocorrect');
  const readConfig = (): Config => {
    const config = new Config();
    config.bengaliDigits = digits.checked;
    config.dariForPeriod = dari.checked;
    config.smartQuotes = quotes.checked;
    config.autocorrect = autocorrect.checked;
    return config;
  };

  let config = readConfig();
  const editor = new DrutiEditor(byId('editor'), config);

  // Live typing.
  const toggleEnglish = byId<HTMLButtonElement>('toggle-english');
  const modeLabel = byId<HTMLSpanElement>('mode-label');
  const refreshMode = () => {
    toggleEnglish.setAttribute('aria-pressed', String(editor.isEnglishMode));
    modeLabel.textContent = editor.isEnglishMode ? 'Typing English' : 'Typing Bengali';
  };
  toggleEnglish.addEventListener('click', () => {
    editor.setEnglishMode(!editor.isEnglishMode);
    refreshMode();
    editor.focus();
  });
  refreshMode();

  // Bulk conversion.
  const romanInput = byId<HTMLTextAreaElement>('roman-input');
  const bengaliOutput = byId<HTMLTextAreaElement>('bengali-output');
  const preserveBreaks = byId<HTMLInputElement>('preserve-breaks');
  const convert = () => {
    bengaliOutput.value = transpileRomanDocument(romanInput.value, preserveBreaks.checked, config);
  };
  byId<HTMLButtonElement>('convert').addEventListener('click', convert);
  preserveBreaks.addEventListener('change', convert);
  convert();

  // Settings apply to the next key and to the bulk conversion.
  for (const input of [digits, dari, quotes, autocorrect]) {
    input.addEventListener('change', () => {
      const previous = config;
      config = readConfig();
      editor.setConfig(config);
      previous.free();
      convert();
    });
  }
};

void main();
