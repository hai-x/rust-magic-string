module.exports = {
  projects: [
    {
      displayName: 'unit tests',
      testMatch: [
        '**/__tests__/**/*.[jt]s?(x)',
        '**/__tests__/**/*.m[jt]s?(x)',
        '**/__tests__/**/?(*.)+(spec|test).[jt]s?(x)'
      ],
      testPathIgnorePatterns: ['<rootDir>/build/', '<rootDir>/node_modules/']
    }
  ]
}
