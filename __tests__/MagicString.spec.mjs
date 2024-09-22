import { MagicString as RustMagicString } from '..'
import MagicString from 'magic-string'

const validator = (handle, expect) => {
  const res = [];
  [RustMagicString, MagicString].map(Cons => {
    res.push(handle(Cons))
  })
  expect(res)
}

describe('MagicString', () => {
  describe('append', () => {
    it('normal', () => {
      validator(
        Cons => {
          const ms = new Cons('ABC')
          ms.append('D')
          ms.append('E')
          ms.append('F')
          if (ms instanceof RustMagicString) {
            console.log(ms.toString())
          }
          return ms.toString()
        },
        res => {
          expect(res[0]).toBe(res[1])
        }
      )
    })
  })

  describe('append', () => {
    it('normal', () => {
      validator(
        Cons => {
          const ms = new Cons('DEF')
          ms.prepend('C')
          ms.prepend('B')
          ms.prepend('A')
          if (ms instanceof RustMagicString) {
            console.log(ms.toString())
          }
          return ms.toString()
        },
        res => {
          expect(res[0]).toBe(res[1])
        }
      )
    })
  })

  describe('appendLeft', () => {
    it("normal",
      () => {
        validator(
          Cons => {
            const ms = new Cons('AF')
            ms.appendLeft(1, 'B')
            ms.appendLeft(1, 'C')
            ms.appendLeft(1, 'D')
            ms.appendLeft(1, 'E')
            if (ms instanceof RustMagicString) {
              console.log(ms.toString())
            }
            return ms.toString()
          },
          res => {
            expect(res[0]).toBe(res[1])
          }
        )
      }

    )
  })

  describe('appendRight', () => {
    it('normal', () => {
      validator(
        Cons => {
          const ms = new Cons('AF')
          ms.appendRight(1, 'B')
          ms.appendRight(1, 'C')
          ms.appendRight(1, 'D')
          ms.appendRight(1, 'E')
          if (ms instanceof RustMagicString) {
            console.log(ms.toString())
          }
          return ms.toString()
        },
        res => {
          expect(res[0]).toBe(res[1])
        }
      )
    })
    it('appendLeft + appendRight', () => {
      validator(
        Cons => {
          const ms = new Cons('AF')
          ms.appendLeft(1, 'B')
          ms.appendRight(1, 'B')
          ms.appendLeft(1, 'C')
          ms.appendRight(1, 'C')
          ms.appendLeft(1, 'D')
          ms.appendRight(1, 'D')
          ms.appendLeft(1, 'E')
          ms.appendRight(1, 'E')
          if (ms instanceof RustMagicString) {
            console.log(ms.toString())
          }
          return ms.toString()
        },
        res => {
          expect(res[0]).toBe(res[1])
        }
      )
    })
  })

  describe('prependLeft', () => {
    it('normal', () => {
      validator(
        Cons => {
          const ms = new Cons('AF')
          ms.prependLeft(1, 'E')
          ms.prependLeft(1, 'D')
          ms.prependLeft(1, 'C')
          ms.prependLeft(1, 'B')
          if (ms instanceof RustMagicString) {
            console.log(ms.toString())
          }
          return ms.toString()
        },
        res => {
          expect(res[0]).toBe(res[1])
        }
      )
    })
  })

  describe('prependRight', () => {
    it('normal', () => {
      validator(
        Cons => {
          const ms = new Cons('AZ')
          ms.prependRight(1, 'Y')
          ms.prependRight(1, 'X')
          ms.prependRight(1, 'I')
          ms.prependRight(1, 'H')
          ms.prependRight(1, 'G')
          ms.prependRight(1, 'F')
          ms.prependRight(1, 'E')
          if (ms instanceof RustMagicString) {
            console.log(ms.toString())
          }
          return ms.toString()
        },
        res => {
          expect(res[0]).toBe(res[1])
        }
      )
    })
    it('prependLeft + prependRight', () => {
      validator(
        Cons => {
          const ms = new Cons('AZ')
          ms.prependLeft(1, 'G')
          ms.prependLeft(1, 'F')
          ms.prependLeft(1, 'E')
          ms.prependRight(1, 'Y')
          ms.prependRight(1, 'X')
          ms.prependRight(1, 'I')
          ms.prependRight(1, 'H')
          if (ms instanceof RustMagicString) {
            console.log(ms.toString())
          }
          return ms.toString()
        },
        res => {
          expect(res[0]).toBe(res[1])
        }
      )
    })
  })

  describe('trim', () => {
    it('normal', () => {
      validator(
        Cons => {
          const ms = new Cons('      C   ')
          ms.prependLeft(4, '  B  ')
          ms.appendLeft(3, 'abc')
          ms.trim()
          if (ms instanceof RustMagicString) {
            console.log(ms.toString())
          }
          return ms.toString()
        },
        res => {
          expect(res[0]).toBe(res[1])
        }
      )
    })
  })

  describe('trimLines', () => {
    it('normal', () => {
      validator(
        Cons => {
          const ms = new Cons('BCDEFGHIGK\r\n')
          ms.prepend('\r\nA')
          ms.trimLines()
          if (ms instanceof RustMagicString) {
            console.log(ms.toString())
          }
          return ms.toString()
        },
        res => {
          expect(res[0]).toBe(res[1])
        }
      )
    })
  })

  describe('move', () => {
    it('normal', () => {
      validator(
        Cons => {
          const ms = new Cons('ABCDEFG')
          ms.prependLeft(4, '  B  ')
          ms.move(4, 6, 1)
          ms.move(2, 4, 5)
          if (ms instanceof RustMagicString) {
            console.log(ms.toString())
          }
          return ms.toString()
        },
        res => {
          expect(res[0]).toBe(res[1])
        }
      )
    })
  })
})
