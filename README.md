# Github Upload Release Assets
This project was created to be used during build-time on my github actions, where
i wanted to upload the build asset to the release that triggered the action.
This is a simple Rust Script that uses the Github API to upload assets to the release.

# How to use

### 1. Add the fallowing Step to your Action

```
- name: Update Release - Add Asset
  run: |
    curl -L https://github.com/pedrosoares/github-upload-release-assets/releases/download/1.0.0/linux-gnu-github-upload-release-assets-1.0.0.zip > gura.zip
    mkdir gura
    unzip gura.zip -d gura
    ./gura/github-upload-release-assets
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
    INPUT_CREATED_TAG: ${{ github.ref_name }}
    GITHUB_REPOSITORY: "pedrosoares/github-upload-release-assets"
    ASSET_FILE: ${{ matrix.asset }}
    # Optional field (would use the ASSET_FILE if not provided)
    ASSET_NAME: "TheNameOfTheAsset"
```

### 2. Give the action runner permission to execute changes

In your project, go to `Settings`, in the left menu go to `Actions - General` and in the end of the page
on the `Workflow permissions` section change to `Read and write permissions`.


### 3. Enjoy
