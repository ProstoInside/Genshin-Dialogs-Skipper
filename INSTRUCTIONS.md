# 📖 GDS — Instructions

**English** | [Русский](INSTRUCTIONS_RU.md)

This guide explains every feature of GDS: what it does, why it exists, and which settings you **must** enable.

---

## ⚡ Quick start

1. Run `gds.exe` and click **Yes** in the Windows admin prompt (UAC).
2. Make sure **Genshin only** is enabled (it is by default).
3. Open Genshin Impact and start a dialogue.
4. Press **END** (default hotkey) — you'll hear a rising chime and the status turns **green**.
5. When the dialogue is over, press **END** again — a falling chime, status turns **red**.

---

## ✅ Required settings

| Setting | Why it's required |
|---|---|
| **Run as administrator** | Genshin Impact runs with admin rights. Windows blocks input from regular programs to admin programs, so without admin rights the game simply ignores GDS. The release build requests admin rights automatically. |
| **Genshin only** — ON | Without it, GDS presses keys in **any** active window. If you Alt+Tab to Discord or a browser with the script running, it will spam `f` and spaces into your chat. Keep it on. |
| **At least one key** — F or Space | If all keys are off, GDS has nothing to press. **F + Space** is the recommended combo. |

---

## 🖥️ Interface

### Status card

Shows what GDS is doing right now:

| Status | Meaning |
|---|---|
| 🔴 **Disabled** | GDS is off and doesn't press anything. |
| 🟡 **Paused** | GDS is on, but the Genshin window is not focused, so it's waiting. Presses resume automatically when you return to the game. |
| 🟢 **Active** | GDS is pressing keys. Shows the number of presses and session time. |

The **Genshin** indicator on the right lights up green when the game window is in focus. If you're in the game and it's grey — see [Troubleshooting](#-troubleshooting).

### Start / Stop button

The big button turns GDS on and off — same as the hotkey. The current hotkey is shown on the right side of the button.

---

## ⚙️ Settings

### F key — *recommended: ON*

Presses **F**. In dialogues it selects the highlighted reply option, so GDS can get through dialogues with choices.

> Outside dialogues F picks up items and interacts with objects.

### Space — *recommended: ON*

Presses **Space**. Skips dialogue text and speeds through animations — this is the main "skip" key.

> Outside dialogues Space makes your character jump.

### Left mouse button — *optional, OFF by default*

Clicks the **left mouse button** at the current cursor position. A backup in case some dialogues don't react to F/Space. Usually not needed.

> Outside dialogues LMB is a normal attack. Also, the click lands wherever your cursor is — keep this in mind.

### Genshin only — *required: ON*

GDS presses keys **only when the Genshin Impact window is active**. If you switch to another window, GDS pauses (yellow status) and resumes when you come back.

Detection works by process name (`GenshinImpact.exe` / `YuanShen.exe`) or, as a fallback, by window title.

### Toggle key — *default: END*

The key that turns GDS on and off from anywhere, including in-game.

**How to change it:**
1. Click **Change** (or anywhere on the row).
2. Press the new key — it's applied immediately.
3. Press **Esc** or click **Cancel** to keep the old key.

**Supported:** F1–F24, Home, End, Insert, Delete, PgUp/PgDn, letters, digits, numpad, left/right Shift/Ctrl/Alt, middle mouse button, **side mouse buttons (MB4 / MB5)**.

**Not allowed:** F, Space and LMB — GDS presses them itself and would turn itself off. Also RMB, Esc and Win.

> 💡 Side mouse buttons or F6–F12 are the most convenient choices — they don't conflict with game controls.

---

## 🔊 Sound

The **speaker button** at the bottom toggles sound notifications:

- **Rising chime** — GDS started.
- **Falling chime** — GDS stopped.

Useful when the game is fullscreen and you can't see the GDS window. When you turn sound back on, you'll hear a preview.

## 🌐 Language

The **RU | EN** switch at the bottom changes the interface language. On first launch, the language is picked based on your Windows language.

---

## 💾 Where settings are stored

All settings (keys, hotkey, sound, language) are saved automatically to:

```
%APPDATA%\GDS\settings.ini
```

To reset everything to defaults — delete this file.

---

## 💡 Tips

- **Turn GDS off after the dialogue.** Otherwise your character will keep jumping (Space), picking things up (F) and attacking (LMB).
- Timing is randomized: 80–140 ms between cycles, 20–40 ms between keys, each key is held for 25–55 ms — just like a real press.
- GDS doesn't read or modify game memory and doesn't inject anything into the game. It only sends key presses, like a keyboard does.

---

## 🔧 Troubleshooting

| Problem | Solution |
|---|---|
| Status is green, but the game doesn't react | GDS is not running as administrator. Restart it and confirm the UAC prompt. |
| I'm in the game, but status is **Paused** and the Genshin indicator is grey | Click on the game window to focus it. If that doesn't help, open an [issue](https://github.com/ProstoInside/Genshin-Dialogs-Skipper/issues) and tell us your game version/region. As a workaround you can turn off **Genshin only** (carefully!). |
| The hotkey doesn't work | Another program may be using the same key. Pick a different one in **Toggle key**. |
| No sound | Check the speaker button at the bottom and your Windows volume mixer. |
| Windows says "Windows protected your PC" | The exe is not code-signed. Click **More info → Run anyway**. You can always build it yourself from source — see [README](README.md#️-developer-instructions). |
