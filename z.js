const MagicString = require('magic-string')
const { MagicString: RustMagicString } = require('.')

function getClassMethods(cls) {
  const prototype = cls.prototype
  const propertyNames = Object.getOwnPropertyNames(prototype)
  const methods = propertyNames.filter(
    prop =>
      typeof prototype[prop] === 'function' &&
      prop !== 'constructor' &&
      !prop.startsWith('_')
  )
  return methods
}

function z() {
  console.log(
    getClassMethods(MagicString)
      .filter(
        api =>
          // exclude private api
          ![
            'trimEndAborted',
            'trimStartAborted',
            'lastChar',
            'lastLine',
            'length',
            'getIndentString'
          ].includes(api)
      )
      .map(api => {
        if (
          Object.getOwnPropertyNames(RustMagicString.prototype).includes(api)
        ) {
          return '- [x] ' + api
        } else {
          return '- [ ] ' + api
        }
      })
      .join('\n')
  )
}

z()
