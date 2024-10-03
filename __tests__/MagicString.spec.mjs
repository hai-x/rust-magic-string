import { MagicString as RustMagicString } from '..'
import MagicString from 'magic-string'

const validate = handle => {
  const res = []
  ;[RustMagicString, MagicString].map(Cons => {
    res.push(handle(Cons))
  })
  expect(res[0]).toBe(res[1])
}

describe('MagicString', () => {
  describe('append', () => {
    it('normal', () => {
      validate(Cons => {
        const s = new Cons('ABC')
        s.append('D')
        s.append('E')
        s.append('F')
        return s.toString()
      })
    })
  })

  describe('append', () => {
    it('normal', () => {
      validate(Cons => {
        const s = new Cons('DEF')
        s.prepend('C')
        s.prepend('B')
        s.prepend('A')
        return s.toString()
      })
    })
  })

  describe('appendLeft', () => {
    it('normal', () => {
      validate(Cons => {
        const s = new Cons('AF')
        s.appendLeft(1, 'B')
        s.appendLeft(1, 'C')
        s.appendLeft(1, 'D')
        s.appendLeft(1, 'E')
        return s.toString()
      })
    })
  })

  describe('appendRight', () => {
    it('normal', () => {
      validate(Cons => {
        const s = new Cons('AF')
        s.appendRight(1, 'B')
        s.appendRight(1, 'C')
        s.appendRight(1, 'D')
        s.appendRight(1, 'E')
        return s.toString()
      })
    })
    it('appendLeft + appendRight', () => {
      validate(Cons => {
        const s = new Cons('AF')
        s.appendLeft(1, 'B')
        s.appendRight(1, 'B')
        s.appendLeft(1, 'C')
        s.appendRight(1, 'C')
        s.appendLeft(1, 'D')
        s.appendRight(1, 'D')
        s.appendLeft(1, 'E')
        s.appendRight(1, 'E')
        return s.toString()
      })
    })
  })

  describe('prependLeft', () => {
    it('normal', () => {
      validate(Cons => {
        const s = new Cons('AF')
        s.prependLeft(1, 'E')
        s.prependLeft(1, 'D')
        s.prependLeft(1, 'C')
        s.prependLeft(1, 'B')

        return s.toString()
      })
    })
  })

  describe('prependRight', () => {
    it('normal', () => {
      validate(Cons => {
        const s = new Cons('AZ')
        s.prependRight(1, 'Y')
        s.prependRight(1, 'X')
        s.prependRight(1, 'I')
        s.prependRight(1, 'H')
        s.prependRight(1, 'G')
        s.prependRight(1, 'F')
        s.prependRight(1, 'E')
        return s.toString()
      })
    })
    it('prependLeft + prependRight', () => {
      validate(Cons => {
        const s = new Cons('AZ')
        s.prependLeft(1, 'G')
        s.prependLeft(1, 'F')
        s.prependLeft(1, 'E')
        s.prependRight(1, 'Y')
        s.prependRight(1, 'X')
        s.prependRight(1, 'I')
        s.prependRight(1, 'H')
        return s.toString()
      })
    })
  })

  describe('trim', () => {
    it('normal', () => {
      validate(Cons => {
        const s = new Cons('      C   ')
        s.prependLeft(4, '  B  ')
        s.appendLeft(3, 'abc')
        s.trim()
        return s.toString()
      })
    })
    it('trim after overwrite', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        s.overwrite(0, 3, '   ').overwrite(9, 12, '   ').trim()
        return s.toString()
      })
    })
  })

  describe('trimLines', () => {
    it('normal', () => {
      validate(Cons => {
        const s = new Cons('BCDEFGHIGK\r\n')
        s.prepend('\r\nA')
        s.trimLines()
        return s.toString()
      })
    })
  })

  describe('move', () => {
    it('normal', () => {
      validate(Cons => {
        const s = new Cons('ABCDEFG')
        s.prependLeft(4, '  B  ')
        s.move(4, 6, 1)
        s.move(2, 4, 5)
        return s.toString()
      })
    })
  })

  describe('isEmpty', () => {
    it('normal', () => {
      validate(Cons => {
        const s = new Cons('  ABCDEFG  ')
        return s.isEmpty()
      })
    })
  })

  describe('overwrite', () => {
    it('normal', () => {
      validate(Cons => {
        const s = new Cons('ABCDEFG')
        s.appendLeft(3, '--appendLeft--')
        s.update(2, 5, 'A')
        return s.toString()
      })
    })
  })
  describe('update', () => {
    it('normal', () => {
      validate(Cons => {
        const s = new Cons('problems = 99')
        s.update(2, 5, 'A')
        s.update(0, 8, 'answer')
        s.toString() // 'answer = 99'
        s.update(11, 13, '42') // character indices always refer to the original string
        s.toString() // 'answer = 42'
        s.prepend('var ').append(';') // most methods are chainable
        return s.toString() // 'var answer = 42;'
      })
    })
  })

  describe('hasChanged', () => {
    it('normal', () => {
      validate(Cons => {
        const s = new Cons('problems = 99')
        s.update(2, 5, 'A')
        return s.hasChanged()
      })
    })
  })

  describe('remove', () => {
    it('normal', () => {
      validate(Cons => {
        const s = new Cons('problems = 99')
        s.remove(2, 5)
        return s.toString()
      })
    })
    it('combo', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        s.remove(0, 6)
        s.appendLeft(6, 'DEF')
        s.overwrite(6, 9, 'GHI')
        return s.toString()
      })
    })
  })

  describe('insert', () => {
    it('normal', () => {
      const s = new RustMagicString('abcdefghijkl')
      expect(() => s.insert()).toThrow(
        '<rust-magic-string> Deprecated api: magicString.insert(...) is deprecated. Use prependRight(...) or appendLeft(...)'
      )
    })
  })

  describe('clone', () => {
    it('normal', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        s.overwrite(3, 9, 'XYZ')
        let cloned = s.clone()
        cloned.overwrite(1, 2, 'ABCD')
        cloned.appendLeft(3, '----appendLeft---')
        cloned.prependLeft(3, '----prependLeft---')
        return cloned.toString()
      })
    })
  })

  describe('snip', () => {
    it('should return a clone with content outside `start` and `end` removed', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl', {
          filename: 'foo.js'
        })
        s.overwrite(6, 9, 'GHI')
        const snippet = s.snip(3, 9)
        return snippet.toString()
      })
    })

    it('should snip from the start', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        const snippet = s.snip(0, 6)
        snippet.overwrite(6, 9, 'GHI')
        return snippet.toString()
      })
    })

    it('should snip from the end', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        const snippet = s.snip(6, 12)
        snippet.overwrite(6, 9, 'GHI')
        return snippet.toString()
      })
    })

    it('should respect original indices', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        const snippet = s.snip(3, 9)
        snippet.overwrite(6, 9, 'GHI')
        return snippet.toString()
      })
    })
  })

  describe('reset', () => {
    it('should reset moved characters from the original string', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        s.remove(1, 5)
        s.reset(2, 4)
        s.reset(4, 5)
        return s.toString()
      })
    })

    it('should reset from the start', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        s.remove(0, 6)
        s.reset(0, 3)
        return s.toString()
      })
    })

    it('should reset from the end', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        s.remove(6, 12)
        s.reset(10, 12)
        return s.toString()
      })
    })

    it('should treat zero-length resets as a no-op', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        s.remove(3, 5)
        s.reset(0, 0).reset(6, 6).reset(9, -3)
        return s.toString()
      })
    })

    it('should treat not modified resets as a no-op', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        s.reset(3, 5)
        return s.toString()
      })
    })

    it('should reset overlapping ranges 1', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        s.remove(0, 10)
        s.reset(1, 7).reset(5, 9)
        return s.toString()
      })
    })
    it('should reset overlapping ranges 2', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        s.remove(0, 10)
        s.reset(3, 7).reset(4, 6)
        return s.toString()
      })
    })

    it('should reset overlapping ranges, redux', () => {
      validate(Cons => {
        const s = new Cons('abccde')
        s.remove(0, 6)
        s.reset(2, 3) // c
        s.reset(1, 3) // bc
        return s.toString()
      })
    })

    it('should reset modified ranges', () => {
      validate(Cons => {
        const s = new Cons('abcdefghi')
        s.overwrite(3, 6, 'DEF')
        s.remove(1, 8) // bcDEFgh
        s.reset(2, 7) // cDEFg
        return s.toString()
      })
    })

    it('should reset modified ranges, redux', () => {
      validate(Cons => {
        const s = new Cons('abcdefghi')
        s.remove(1, 8)
        s.appendLeft(2, 'W')
        s.appendRight(2, 'X')
        s.prependLeft(3, 'Y')
        s.prependRight(5, 'Z')
        s.reset(2, 7)
        return s.toString()
      })
    })

    it('should not reset content inserted after the end of range', () => {
      validate(Cons => {
        const s = new Cons('ab.c;')
        s.prependRight(0, '(')
        s.prependRight(4, ')')
        s.remove(1, 4)
        s.reset(2, 4)
        return s.toString()
      })
    })

    // it('should provide a useful error when illegal removals are attempted', () => {
    //   validate(Cons => {
    //     const s = new Cons('abcdefghijkl');

    //     s.remove(4, 8);

    //     s.overwrite(5, 7, 'XX');

    //     // TODO:
    //     assert.throws(() => s.reset(4, 6), /Cannot split a chunk that has already been edited/);
    //   })
    // });

    it('removes across moved content', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        s.remove(5, 8)
        s.move(6, 9, 3)
        s.reset(7, 8)
        return s.toString()
      })
    })
  })

  describe('replace', () => {
    it('works with string replace', () => {
      validate(Cons => {
        const code = '1 2 1 2'
        const s = new Cons(code)
        s.replace('2', '3')
        return s.toString()
      })
    })

    // TODO:
    // it('Should not treat string as regexp', () => {
    //   validate(Cons => {
    //     const code = '1234'
    //     const s = new Cons(code)
    //     s.replace('.', '*')
    //     return s.toString()
    //   })
    // })
  })

  describe('replaceAll', () => {
    it('works with string replace', () => {
      validate(Cons => {
        const code = '1 2 1 2'
        const s = new Cons(code)
        s.replaceAll('2', '3')
        return s.toString()
      })
    })
  })
})
