// check-contrast.mjs — WCAG 2.1 AA guard for the eight surfaces (order RELAY-10).
//
// TWO checks live in this tree, and this is the CHEAP one:
//   1. verify/contrast-probe.js  — the AUTHORITY. Run in a browser on each rendered surface;
//      it composites real backgrounds (incl. color-mix) and has its own positive control.
//      That is what proved 135 -> 0. A static script cannot composite a cascade.
//   2. THIS FILE — a TOKEN-level regression guard for CI. It reads each surface's :root and
//      asserts the text tokens clear 4.5:1 against the surface's hardest text background.
//      It cannot see a NEW element that sets a data-hue token as text — the browser probe
//      catches that. Its job is narrow: stop a known text token drifting back below AA.
//
// Law 2 applied to this tool: `selftest` builds fixtures and asserts it FAILS on a bad token
// and PASSES on the intact set. A checker only ever seen passing is a decoration.
//
//   node verify/check-contrast.mjs            # guard the surfaces
//   node verify/check-contrast.mjs selftest   # prove it bites
//
// EXIT: 0 pass · 1 a token failed · 2 REFUSED (no surfaces / no :root / no tokens) — refusal
// is not a pass.

import { readFileSync, readdirSync, mkdtempSync, writeFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

const AA = 4.5;

const lin = c => { c /= 255; return c <= 0.03928 ? c / 12.92 : Math.pow((c + 0.055) / 1.055, 2.4); };
const lum = ([r, g, b]) => 0.2126 * lin(r) + 0.7152 * lin(g) + 0.0722 * lin(b);
const ratio = (fg, bg) => { const a = lum(fg), b = lum(bg); return (Math.max(a, b) + 0.05) / (Math.min(a, b) + 0.05); };
const hex = h => { h = h.replace('#', ''); return [0, 2, 4].map(i => parseInt(h.slice(i, i + 2), 16)); };

// text tokens that are set as `color:` somewhere and must clear AA. DATA hues (--biomass,
// --o3, --o6, --sfa, --mufa, --nm, --leaf, --you, --sovereign) are DATUM used as fills or as
// large numbers and are deliberately absent — repainting them for contrast is forbidden.
const TEXT_TOKENS = ['--ink-dim', '--ink-mut', '--mut', '--dim', '--ai', '--info', '--guard', '--biomass-ink', '--sfa-ink'];

function tokens(css) {
  const root = css.match(/:root\s*\{([\s\S]*?)\}/);
  if (!root) return null;
  const map = {};
  for (const m of root[1].matchAll(/(--[a-z0-9-]+)\s*:\s*(#[0-9A-Fa-f]{6})\b/g)) map[m[1]] = m[2];
  return map;
}

// return {token, bg, ratio} failures for one surface's :root map
function checkSurface(map) {
  const fails = [];
  const dark = '--bg' in map;                      // dark surfaces declare --bg; light declare --paper
  const textBgs = dark ? ['--bg', '--card'] : ['--paper', '--card'];
  for (const t of TEXT_TOKENS) {
    if (!(t in map)) continue;
    for (const bgName of textBgs) {
      if (!(bgName in map)) continue;
      const r = ratio(hex(map[t]), hex(map[bgName]));
      if (r < AA) fails.push({ token: t, on: bgName, ratio: +r.toFixed(2) });
    }
  }
  // honey rides a dark chip on light surfaces (--ink) and a dark card on dark surfaces (--card).
  if ('--b-value' in map) {
    const honeyBg = dark ? '--card' : '--ink';
    if (honeyBg in map) {
      const r = ratio(hex(map['--b-value']), hex(map[honeyBg]));
      if (r < AA) fails.push({ token: '--b-value', on: honeyBg, ratio: +r.toFixed(2) });
    }
  }
  return fails;
}

function run(dir) {
  let files;
  try { files = readdirSync(dir).filter(f => f.endsWith('.html')); }
  catch { console.error(`REFUSE: cannot read ${dir}`); return 2; }
  if (files.length === 0) { console.error('REFUSE: 0 surfaces — a scan of nothing is not a pass.'); return 2; }
  let checked = 0, failed = 0;
  for (const f of files.sort()) {
    const map = tokens(readFileSync(join(dir, f), 'utf8'));
    if (!map) { console.error(`REFUSE: no :root in ${f}`); return 2; }
    const present = TEXT_TOKENS.filter(t => t in map);
    if (present.length === 0 && !('--b-value' in map)) { console.error(`REFUSE: no checkable text tokens in ${f}`); return 2; }
    const fails = checkSurface(map);
    checked++;
    if (fails.length) {
      failed++;
      console.log(`  FAIL ${f}`);
      for (const x of fails) console.log(`    ${x.token} on ${x.on} = ${x.ratio}:1 (need ${AA})`);
    } else {
      console.log(`  ok   ${f} (${present.length} text tokens + honey clear ${AA}:1)`);
    }
  }
  console.log(`surfaces_checked=${checked} failed=${failed}`);
  return failed ? 1 : 0;
}

function selftest() {
  console.log('selftest: assert the guard fails on a bad token and passes intact');
  const work = mkdtempSync(join(tmpdir(), 'contrast-selftest-'));
  const good = `<style>:root{--paper:#F7F6F1;--card:#FFFFFF;--ink:#1E2320;--ink-dim:#68726A;--biomass-ink:#487D34;--b-value:#E8B54B;}</style>`;
  const bad = `<style>:root{--paper:#F7F6F1;--card:#FFFFFF;--ink:#1E2320;--ink-dim:#AAAAAA;--b-value:#E8B54B;}</style>`; // #AAAAAA ~1.9:1
  let pass = 0, fail = 0;
  const c = (name, dir, want) => {
    const got = run(dir);
    if (got === want) { console.log(`  ok    ${name} exit=${got} (want ${want})`); pass++; }
    else { console.log(`  FAIL  ${name} exit=${got} (want ${want})`); fail++; }
  };
  writeFileSync(join(work, 'good.html'), good); c('intact_passes', work, 0);
  rmSync(join(work, 'good.html')); writeFileSync(join(work, 'bad.html'), bad); c('bad_token_fails', work, 1);
  rmSync(work, { recursive: true, force: true });
  console.log(`selftest: ${pass} passed, ${fail} failed`);
  return fail ? 1 : 0;
}

const mode = process.argv[2];
const surfaces = join(process.cwd(), 'surfaces');
process.exit(mode === 'selftest' ? selftest() : run(surfaces));
