const b = require('benny')
const MagicString = require('magic-string')
const { MagicString: RustMagicString } = require('./')

const BANNER = `/*!
* Vue.js v2.6.14
* (c) 2014-2021 Evan You
* Released under the MIT License.
*/
`

const EXPORT_STATEMENT = `export default foo`

b.suite(
  'overwrite',
  b.add('MagicString', () => {
    const m = new MagicString(`export const foo = 'bar'`)
    m.overwrite(13, 16, 'bar')
  }),
  b.add('RustMagicString', () => {
    const m = new RustMagicString(`export const foo = 'bar'`)
    m.overwrite(13, 16, 'bar')
  }),
  b.cycle(),
  b.complete()
)

b.suite(
  'prepend|append',
  b.add('MagicString', () => {
    const m = new MagicString(`export const foo = 'bar'`)
    m.prepend(BANNER)
    m.append(EXPORT_STATEMENT)
  }),
  b.add('RustMagicString', () => {
    const m = new RustMagicString(`export const foo = 'bar'`)
    m.prepend(BANNER)
    m.append(EXPORT_STATEMENT)
  }),
  b.cycle(),
  b.complete()
)

b.suite(
  'add banner#toString',
  b.add('MagicString', () => {
    const m = new MagicString(`export const foo = 'bar'`)
    m.prepend(BANNER)
    m.toString()
  }),
  b.add('RustMagicString', () => {
    const m = new RustMagicString(`export const foo = 'bar'`)
    m.prepend(BANNER)
    m.toString()
  }),
  b.cycle(),
  b.complete()
)

b.suite(
  'add banner#generateDecodedMap',
  b.add('MagicString', () => {
    const m = new MagicString(`export const foo = 'bar'`)
    m.prepend(BANNER)
    m.generateDecodedMap()
  }),
  b.add('RustMagicString', () => {
    const m = new RustMagicString(`export const foo = 'bar'`)
    m.prepend(BANNER)
    m.generateDecodedMap()
  }),
  b.cycle(),
  b.complete()
)

b.suite(
  'add banner#generateMapHires',
  b.add('MagicString', () => {
    const m = new MagicString(`export const foo = 'bar'`)
    m.prepend(BANNER)
    m.generateMap({
      hires: true
    })
  }),
  b.add('RustMagicString', () => {
    const m = new RustMagicString(`export const foo = 'bar'`)
    m.prepend(BANNER)
    m.generateMap({
      hires: true
    })
  }),
  b.cycle(),
  b.complete()
)

b.suite(
  'add banner#generateMap',
  b.add('MagicString', () => {
    const m = new MagicString(`export const foo = 'bar'`)
    m.prepend(BANNER)
    m.generateMap()
  }),
  b.add('RustMagicString', () => {
    const m = new RustMagicString(`export const foo = 'bar'`)
    m.prepend(BANNER)
    m.generateMap()
  }),
  b.cycle(),
  b.complete()
)

b.suite(
  'add banner#generateMap.toString',
  b.add('MagicString', () => {
    const m = new MagicString(`export const foo = 'bar'`)
    m.prepend(BANNER)
    m.generateMap().toString()
  }),
  b.add('RustMagicString', () => {
    const m = new RustMagicString(`export const foo = 'bar'`)
    m.prepend(BANNER)
    m.generateMap().toString()
  }),
  b.cycle(),
  b.complete()
)

b.suite(
  'add banner#generateMapHires.toString',
  b.add('MagicString', () => {
    const m = new MagicString(`export const foo = 'bar'`)
    m.prepend(BANNER)
    m.generateMap({
      hires: true
    }).toString()
  }),
  b.add('RustMagicString', () => {
    const m = new RustMagicString(`export const foo = 'bar'`)
    m.prepend(BANNER)
    m.generateMap({
      hires: true
    }).toString()
  }),
  b.cycle(),
  b.complete()
)

b.suite(
  'add banner#generateMap.toUrl',
  b.add('MagicString', () => {
    const m = new MagicString(`export const foo = 'bar'`)
    m.prepend(BANNER)
    m.generateMap().toUrl()
  }),
  b.add('RustMagicString', () => {
    const m = new RustMagicString(`export const foo = 'bar'`)
    m.prepend(BANNER)
    m.generateMap().toUrl()
  }),
  b.cycle(),
  b.complete()
)
