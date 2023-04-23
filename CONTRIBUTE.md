# How To Contribute

Your contributions are always welcomed! Here are a few guidelines:

- Every PR should include unit tests for the change. Preferablly both for the happy and the unhappy paths.
- `gitopen` uses `anyhow::Error` for its errors. Please use that.
- If the PR fixes an issue, please name the branch accordingly as in `username/Issue123` and in the PR title put `Fix Issue 123`
- CI should pass
- If you're adding a new feature, please add proper documentation for it as well in the `README.md`
