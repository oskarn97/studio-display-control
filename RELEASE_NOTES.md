# Release v2.1 - Studio Display XDR Support

## ✨ New Features

### 🖥️ Apple Studio Display XDR Support
- **Full compatibility** with the new Apple Studio Display XDR (2026, Product ID `0x1116`)
- **Automatic model detection** - GUI shows correct model name (Studio Display vs Studio Display XDR)
- **Mixed setups** - Control both Studio Display and Studio Display XDR in the same multi-monitor setup

### 🔧 Improved HID Device Detection
- **Usage Page filtering** - More precise HID device matching using Usage Page `0x0080` (Monitor Control)
- **Fixes duplicate display entries** on devices with multiple HID collections (e.g. Studio Display XDR)

## 📊 System Requirements

- **OS:** Windows 10 or Windows 11
- **Display:** Apple Studio Display (27-inch, 5K) or Apple Studio Display XDR
- **Connection:** USB-C / Thunderbolt
- **Permissions:** Standard user (no admin required)

---

# Release v2.0 - Modern Multi-Display UI

## 🎉 Major Update: Complete UI Overhaul

This release brings a completely redesigned user interface with professional multi-display support and modern aesthetics.

## ✨ New Features

### 🖥️ Multi-Display Management
- **Master Control** - Adjust all connected displays simultaneously
- **Individual Controls** - Fine-tune each display independently
- **Up to 4 displays** - Full support for complex setups
- **Real-time status** - See all connected displays and their serial numbers

### 🎨 Modern User Interface
- **Professional design** - Clean, intuitive layout inspired by modern Windows apps
- **Custom vector icons** - Display, lightning, and brightness icons
- **HiDPI support** - Perfect scaling on 5K and high-resolution displays
- **Optimized spacing** - Compact layout fits all controls without scrolling
- **Status indicators** - Green dot shows connection status
- **Percentage display** - Large, easy-to-read brightness values

### 🚀 Enhanced Usability
- **Smart header** - Shows display count at a glance
- **Contextual UI** - Interface adapts based on number of connected displays
- **Info tooltips** - Helpful hints in the footer
- **Instant feedback** - Changes apply immediately

### 🖼️ Application Icon
- **Custom icon** - Professional monitor icon with gradient background
- **Multi-resolution** - Includes sizes from 16x16 to 256x256
- **Windows integration** - Shows in Explorer, Taskbar, and Task Manager

## 🔧 Technical Improvements

- **Better HID handling** - More reliable display detection
- **Error resilience** - Graceful handling of connection issues
- **Optimized rendering** - Smooth UI performance
- **Reduced dependencies** - Smaller binary size
- **Build automation** - Icon generation integrated into build process

## 📦 What's Included

- **studi.exe** (6.5 MB) - Standalone executable
  - No installation required
  - No external dependencies
  - Works on Windows 10/11

## 🎯 Use Cases

### Office/Productivity
- Quick brightness adjustment via Master Control
- Individual tuning for different display types
- Consistent brightness across workstations

### Docking Stations
- Set all displays to preferred brightness instantly
- No need to adjust each display manually
- Smooth undocking/redocking experience

### Multi-Monitor Setups
- Independent control for different lighting conditions
- Synchronized adjustment when needed
- Display information for easy identification

## 📊 System Requirements

- **OS:** Windows 10 or Windows 11
- **Display:** Apple Studio Display (27-inch, 5K)
- **Connection:** USB-C with DisplayPort Alt Mode
- **Permissions:** Standard user (no admin required)

## 🐛 Known Issues

None currently reported.

## 🔮 Future Considerations

Features that were explored but not included in this release:
- System tray integration (complex event loop integration)
- Auto-brightness on connect (requires persistent background service)
- Windows autostart (requires registry modifications)

These features can be added in future releases if there's demand.

## 📝 Upgrading from v1.0

Simply replace your old `studi.exe` with the new version. No configuration migration needed.

## 🙏 Credits

- Original CLI tool: [juliuszint/asdbctl](https://github.com/juliuszint/asdbctl)
- UI foundation: [himbeles/studi](https://github.com/himbeles/studi)
- Enhanced by: oskarn97

---

**Download:** [studi.exe](https://github.com/oskarn97/studio-display-control/releases/download/v2.0/studi.exe)

**Enjoy the new interface!** 🚀

