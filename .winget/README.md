# WinGet Setup

This directory contains the initial manifest for submitting recall to the Windows Package Manager.

## One-time Setup

1. **Fork winget-pkgs**: Fork https://github.com/microsoft/winget-pkgs to your account (zippoxer)

2. **Create a PAT**: Create a classic Personal Access Token at https://github.com/settings/tokens with `public_repo` scope

3. **Add repository secret**: Add the PAT as `WINGET_TOKEN` secret at https://github.com/zippoxer/recall/settings/secrets/actions

4. **Submit initial manifest**: Use `wingetcreate` to submit the first version:
   ```powershell
   # Install wingetcreate
   winget install wingetcreate

   # Create and submit manifest (run from this directory)
   wingetcreate submit --token <YOUR_PAT> manifests/z/zippoxer/recall/0.2.0/
   ```

   Or manually create a PR to microsoft/winget-pkgs with the manifests in the `manifests/` subdirectory.

## Automatic Updates

After the initial manifest is merged, future releases will automatically submit PRs to winget-pkgs via the `publish-winget` job in `.github/workflows/release.yml`.

Pre-release versions (e.g., v0.2.1-rc1) are skipped.
