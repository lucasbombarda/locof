# LOCOF - Lines Of Code Of
This is a CLI tool to count the lines of code (LOC) in a given directory written in Rust.
Inspired in the [cloc](https://github.com/AlDanial/cloc) tool.
This is not meant to be a replacement for cloc.

Note that this is an early-stage prototype, so functionality may be limited, and breaking changes may occur as development progresses.

# Usage
```bash
locof <path>
```

# Roadmap
- [ ] Count the lines of code, comments and blank lines in a given directory
- [ ] Add `--exclude-dir` and `--exclude-ext` flags
- [ ] Add support to .gitignore files
- [ ] Output in JSON format
- [ ] Multi-threading

# License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
