const Benchmark = require('benchmark')
const MagicString = require('magic-string')
const { MagicString: RustMagicString } = require('./')

const BANNER = `/*!
* Vue.js v2.6.14
* (c) 2014-2021 Evan You
* Released under the MIT License.
*/
`

const EXPORT_STATEMENT = `export default foo`

let suites = {
  overwrite: Cons => {
    const m = new Cons(`export const foo = 'bar'`)
    m.overwrite(13, 16, 'bar')
  },
  'prepend|append': Cons => {
    const m = new Cons(`export const foo = 'bar'`)
    m.prepend(BANNER)
    m.append(EXPORT_STATEMENT)
  },
  generateDecodedMap: Cons => {
    const m = new Cons(`export const foo = 'bar'`)
    m.prepend(BANNER)
    m.generateDecodedMap()
  },
  clone: Cons => {
    const m = new Cons(`export const foo = 'bar'`)
    m.prepend(BANNER)
    m.clone()
  },
  move: Cons => {
    const m = new Cons(`export const foo = 'bar'`)
    m.move(3, 6, 9)
  },
  slice: Cons => {
    const m = new Cons(`export const foo = 'bar'`)
    m.slice(6, 9)
  },
  toString: Cons => {
    const m = new Cons(`export const foo = 'bar'`)
    m.prepend(BANNER)
    m.toString()
  }
}

const run = () => {
  const s = Object.keys(suites)
  let count = s.length
  s.forEach(k => {
    const fn = suites[k]
    const suite = new Benchmark.Suite()
    suite
      .add(`MagicString # ${k}`, function () {
        fn(MagicString)
      })
      .add(`RustMagicString # ${k}`, function () {
        fn(RustMagicString)
      })
      .on('cycle', function (event) {
        console.log(String(event.target))
      })
      .on('complete', function () {
        console.log('Fastest is ' + this.filter('fastest').map('name'))
        count--
        if (!count) process.exit()
      })
      .on('error', e => {
        console.error(e.target.error)
      })
      .run({ async: false })
  })
}

run()
