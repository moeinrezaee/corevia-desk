import { chmodSync, cpSync, copyFileSync, mkdirSync, rmSync, writeFileSync } from 'node:fs'
import { join } from 'node:path'
import { spawnSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'
import { assertReleaseTarget, macBundleIdentity, releaseMetadata, UI_RUNTIME } from './profile-contract.mjs'
import { releaseOutputDir, releaseTargetRoot } from './target-path.mjs'

const root = fileURLToPath(new URL('..', import.meta.url))

const required = (name) => {
  const value = process.env[name]
  if (!value) throw new Error(`${name} is required.`)
  return value
}

const profile = required('COREVIA_DESK_PROFILE')
const platform = required('COREVIA_DESK_PLATFORM')
const arch = required('COREVIA_DESK_ARCH')
const version = process.env.COREVIA_DESK_VERSION
  || (process.env.CI_COMMIT_TAG || '').replace(/^v/u, '')
  || '0.1.0-dev'

assertReleaseTarget({ profile, platform, architecture: arch })
const outputDir = releaseOutputDir({
  root,
  profile,
  platform,
  architecture: arch,
  configuredOutputDir: process.env.COREVIA_RELEASE_OUTPUT_DIR
})
if (!/^[A-Za-z0-9][A-Za-z0-9._-]*$/u.test(version)) {
  throw new Error('COREVIA_DESK_VERSION must contain only letters, numbers, dots, underscores, and hyphens.')
}

const run = (command, args, extraEnv = {}) => {
  const executable = process.platform === 'win32' && command === 'npm' ? 'npm.cmd' : command
  const result = spawnSync(executable, args, {
    cwd: root,
    env: { ...process.env, ...extraEnv },
    stdio: 'inherit',
    shell: process.platform === 'win32' && executable.endsWith('.cmd')
  })
  if (result.error) throw result.error
  if (result.status !== 0) throw new Error(`${command} failed with exit code ${result.status}.`)
}

const expectedPlatform = process.platform === 'darwin'
  ? 'macos'
  : process.platform === 'win32' ? 'windows' : 'linux'
const expectedArchitecture = process.arch === 'arm64' ? 'arm64' : 'x86_64'
if (!process.env.COREVIA_CARGO_TARGET && (expectedPlatform !== platform || expectedArchitecture !== arch)) {
  throw new Error(`Release target ${platform}/${arch} must be built by a matching runner or COREVIA_CARGO_TARGET.`)
}

if (process.env.COREVIA_DESK_UI && process.env.COREVIA_DESK_UI !== UI_RUNTIME.host) {
  throw new Error(`COREVIA_DESK_UI must be ${UI_RUNTIME.host}.`)
}

const webDir = join(root, 'web')
if (process.env.COREVIA_SKIP_NPM_INSTALL !== '1') {
  run('npm', ['--prefix', webDir, 'ci'])
}
run('npm', ['--prefix', webDir, 'run', 'generate'], { COREVIA_DESK_PROFILE: profile })

const targetTriple = process.env.COREVIA_CARGO_TARGET
const targetRoot = releaseTargetRoot({
  root,
  profile,
  platform,
  architecture: arch,
  configuredTargetDir: process.env.CARGO_TARGET_DIR
})
const cargoArgs = ['build', '--locked', '--release']
if (targetTriple) cargoArgs.push('--target', targetTriple)
const cargoFeatures = []
if (profile === 'customer-support') cargoFeatures.push('corevia-customer-support')
if (platform === 'linux') cargoFeatures.push('linux-pkg-config')
if (cargoFeatures.length > 0) cargoArgs.push('--features', cargoFeatures.join(','))
if (process.env.COREVIA_SKIP_CARGO_BUILD === '1') {
  throw new Error('COREVIA_SKIP_CARGO_BUILD is not allowed for release packaging; build the requested native profile first.')
}
run('cargo', cargoArgs, {
  CARGO_TARGET_DIR: targetRoot,
  COREVIA_DESK_VERSION: version
})

const releaseDir = targetTriple ? join(targetRoot, targetTriple, 'release') : join(targetRoot, 'release')
const compiledBinaryName = platform === 'windows' ? 'rustdesk.exe' : 'rustdesk'
const packagedBinaryName = platform === 'windows' ? 'Corevia Desk.exe' : 'rustdesk'
const binaryPath = join(releaseDir, compiledBinaryName)

const bundleDir = join(outputDir, 'bundle')
rmSync(bundleDir, { recursive: true, force: true })
mkdirSync(bundleDir, { recursive: true })

const releaseMetadataForTarget = releaseMetadata({ profile, platform, architecture: arch, version })
const webOutput = join(webDir, '.output', 'public')
const macIdentity = macBundleIdentity(profile)

const profileFile = (entrypoint) => `${JSON.stringify({
  ...releaseMetadata({ profile, platform, architecture: arch, version }),
  entrypoint,
  bridgeContract: 'window.__COREVIA_DESK_BRIDGE__'
}, null, 2)}\n`

if (platform === 'macos') {
  const appDir = join(bundleDir, 'Corevia Desk.app')
  const contentsDir = join(appDir, 'Contents')
  const macosDir = join(contentsDir, 'MacOS')
  const resourcesDir = join(contentsDir, 'Resources')
  mkdirSync(macosDir, { recursive: true })
  mkdirSync(resourcesDir, { recursive: true })

  const executablePath = join(macosDir, 'CoreviaDesk')
  copyFileSync(binaryPath, executablePath)
  chmodSync(executablePath, 0o755)
  cpSync(webOutput, join(resourcesDir, 'web', 'public'), { recursive: true })
  writeFileSync(join(resourcesDir, 'profile.json'), profileFile('Contents/Resources/web/public/index.html'))

  const iconsetDir = join(resourcesDir, 'CoreviaDesk.iconset')
  const iconPath = join(root, 'res', 'mac-icon.png')
  mkdirSync(iconsetDir, { recursive: true })
  for (const [size, scale] of [[16, 1], [16, 2], [32, 1], [32, 2], [128, 1], [128, 2], [256, 1], [256, 2], [512, 1], [512, 2]]) {
    const suffix = scale === 2 ? '@2x' : ''
    run('sips', [
      '-z', String(size * scale), String(size * scale), iconPath,
      '--out', join(iconsetDir, `icon_${size}x${size}${suffix}.png`)
    ])
  }
  run('iconutil', ['-c', 'icns', iconsetDir, '-o', join(resourcesDir, 'CoreviaDesk.icns')])
  rmSync(iconsetDir, { recursive: true, force: true })

  writeFileSync(join(contentsDir, 'Info.plist'), `<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleDisplayName</key>
  <string>${macIdentity.displayName}</string>
  <key>CFBundleExecutable</key>
  <string>CoreviaDesk</string>
  <key>CFBundleIconFile</key>
  <string>CoreviaDesk.icns</string>
  <key>CFBundleIdentifier</key>
  <string>${macIdentity.bundleIdentifier}</string>
  <key>CFBundleName</key>
  <string>${macIdentity.displayName}</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleShortVersionString</key>
  <string>${version}</string>
  <key>CFBundleVersion</key>
  <string>${version}</string>
  <key>LSMinimumSystemVersion</key>
  <string>10.14</string>
  <key>NSHighResolutionCapable</key>
  <true/>
</dict>
</plist>
`)

  // The Rust linker leaves an ad-hoc signature on the executable, but that is
  // not a sealed macOS app bundle and its identifier does not match the
  // profile's CFBundleIdentifier. Sign the completed bundle so macOS privacy
  // permissions can identify this exact profile. A stable Developer ID can
  // be supplied for repeatable permissions across builds; '-' is local-only.
  const signingIdentity = process.env.COREVIA_MACOS_SIGNING_IDENTITY || '-'
  run('codesign', ['--force', '--sign', signingIdentity, appDir])
} else {
  cpSync(binaryPath, join(bundleDir, packagedBinaryName))
  cpSync(webOutput, join(bundleDir, 'web', 'public'), { recursive: true })
  if (platform === 'windows') {
    const nativeAssetsDir = join(bundleDir, 'data', 'corevia_assets')
    mkdirSync(nativeAssetsDir, { recursive: true })
    copyFileSync(join(root, 'res', 'icon.ico'), join(nativeAssetsDir, 'icon.ico'))
  }
  writeFileSync(join(bundleDir, 'profile.json'), profileFile('web/public/index.html'))
}

console.log(`Corevia Desk ${releaseMetadataForTarget.platform} bundle prepared: ${bundleDir}`)
