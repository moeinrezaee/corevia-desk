import { join, resolve, sep } from 'node:path'

export const releaseTargetRoot = ({
  root,
  profile,
  platform,
  architecture,
  configuredTargetDir
}) => configuredTargetDir
  ? resolve(root, configuredTargetDir)
  : join(root, 'target', 'corevia-profiles', platform, architecture, profile)

export const releaseOutputDir = ({
  root,
  profile,
  platform,
  architecture,
  configuredOutputDir
}) => {
  const repositoryRoot = resolve(root)
  const outputRoot = configuredOutputDir
    ? resolve(repositoryRoot, configuredOutputDir)
    : join(repositoryRoot, 'release-output')

  if (configuredOutputDir) {
    const workRoot = join(repositoryRoot, '.work')
    if (outputRoot === workRoot || !outputRoot.startsWith(`${workRoot}${sep}`)) {
      throw new Error('COREVIA_RELEASE_OUTPUT_DIR must be a dedicated subdirectory of the repository .work directory.')
    }
  }

  return join(outputRoot, profile, platform, architecture)
}
