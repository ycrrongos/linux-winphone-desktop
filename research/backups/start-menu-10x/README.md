# Start menu backup — Win10X centered Launcher

Snapshot of the centered 10X Start (search + “My apps and websites” + 5×3 / Show all grid + Recent) before switching to a Win10-style fullscreen Live Tile Start.

| File | Contents |
|------|----------|
| `main.rs.snapshot` | Full `shell/src/main.rs` at backup time |
| `launcher_10x.rs.txt` | `draw_launcher` + `paint_app_grid` / `fit_icon_frame` / `truncate_label` |

To restore: copy helpers back into `shell/src/main.rs` and replace `draw_launcher`; set compositor `panel_card_rect` Launcher insets to `28/48/16/48` again (current tree uses fullscreen `0,0,desktop` for the tile Start).
