# fbmark_v2

This is TUI bookmark manager backed with [ratatui](https://crates.io/crates/ratatui).
This is TUI bookmark manager written with ratatui.  It could be useful in case you have an account in Tagpacker. It provides you with a fuzzy search in bookmark's names and URLs + search via tags and their combination. 

![2023-10-07_19-35-10](https://github.com/kstepanovdev/fbmark_v2/assets/13778974/a991cbcf-e82e-48c5-ad51-f6a1761e4906)

##### Known problems:
- [ ] The app isn't fully async.
- [ ] The app allows multiple tags selection (even on the same ones)
- [ ] The tags cannot be reset/deselect.
- [ ] Selected tags aren't scrollable.
- [ ] The app crashes if you provide "Url" field in creation mode with invalid Url.
- [ ] Back synchronization from the app to the Tagpacker isn't implemented.
- [ ] Sqlite request aren't optimal.
- [ ] Keymap and colors are hardcoded

##### To be done in the next release
- Bugfixing the mentioned problems
- Elm architecture for the main loop
