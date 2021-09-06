# The Nari2D Export File Format

## XML

NOTE: All "binary" data is encoded in base64.

### Available Tags:

#### Nari2D & Nari2D Configuration Tags:
- `<nari2d>`: The start of the file. Contains everything. (REQUIRED)
  - `version=`: The Nari2D Version
- `<version>`: A tag describing an version
  - `major=`: A major version number.
  - `minor=`: A minor version number.
  - `patch=`: A patch version number.
  - `postfix=`: An optional postfix for the version.
  - `commit=`: An optional GH commit SHA.
- `<preview>`: Path/Data to a preview PNG
  - `type=`: One of two values, `path` or `data`. In `path`, it is a path to the file. In `data`, the preview value is the binary data for the preview image.
- `<feature>`: A tag describing an experimental feature required to run
  - `type=`: Feature Name
- `<manifest>`: The tag that contains author & model metadata.

#### Nari2D Author & Model Metadata Tags:
- `<name>`: A tag containing a name
  - `lang=`: The language the name is in to facilitate Unicode Han Unification. 2 character language ISO string, e.g. `"ja"` (Japanese) (Optional)
  - `desc=`: A description of what this person did.
- `<authors>`: Tag containing the authors.
  - `<artist>`: A tag containing the name of the artists. Contains `<name>` tag(s)
  - `<animators>`: A tag containing the name of the animators/riggers.
- `<title>`: The name of the model itself
  - `lang=`: The language the name is in to facilitate Unicode Han Unification. 2 character language ISO string, e.g. `"ja"` (Japanese)
- `<license>`: SPDX License 