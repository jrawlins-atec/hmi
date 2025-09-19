# Slint HMI Application

A modern Human-Machine Interface (HMI) application built with Rust and Slint, featuring custom widgets and dynamic theme switching.

## Features

### Custom Widgets
- **CustomButton**: Styled buttons with multiple variants (primary, secondary, danger)
- **CustomCard**: Elevated card components with shadows and themed styling
- **CustomInput**: Text input fields with labels and placeholder support
- **CustomToggle**: Switch components with smooth animations
- **TextFileCard**: File viewer card that displays text file contents with syntax highlighting

### Theme System
- **Light Theme**: Clean, bright interface with subtle gradients
- **Dark Theme**: Modern dark interface for low-light environments
- **Colorful Theme**: Vibrant theme with gradient backgrounds and colorful accents

### Theme Features
- Dynamic theme switching at runtime
- Persistent theme preferences (saved to config file)
- Smooth color transitions between themes
- Gradient support for enhanced visual appeal

## Project Structure

```
├── src/
│   └── main.rs              # Main application logic and theme persistence
├── ui/
│   ├── main.slint           # Main application window and layout
│   ├── widgets.slint        # Custom widget definitions
│   └── themes.slint         # Theme definitions and theme manager
├── Cargo.toml               # Project dependencies
├── build.rs                 # Build script for Slint compilation
└── README.md               # This file
```

## Dependencies

- **slint**: UI framework for building native applications
- **serde**: Serialization framework for config persistence
- **serde_json**: JSON support for configuration files
- **dirs**: Cross-platform directory utilities

## Building and Running

### Prerequisites
- Rust 1.70 or later
- Cargo (comes with Rust)

### Build
```bash
cargo build
```

### Run
```bash
cargo run
```

## Usage

### Theme Switching
- Use the theme buttons in the header to switch between Light, Dark, and Colorful themes
- Theme preferences are automatically saved and restored on application restart
- The "Dark mode preference" toggle provides an alternative way to switch between light and dark themes

### Custom Widgets Demo
The application showcases various custom widgets:

1. **Button Variants**: Primary, Secondary, and Danger buttons with hover effects
2. **Input Fields**: Text inputs with and without labels, featuring focus states
3. **Toggle Controls**: Interactive switches with smooth animations
4. **Status Indicators**: Visual status badges, progress bars, and connection indicators
5. **Cards**: Elevated containers with themed styling and shadows
6. **Text File Viewer**: Interactive file browser that displays text file contents with monospace font

### Configuration
Theme preferences are stored in:
- Windows: `%APPDATA%/slint-hmi-app/config.json`
- macOS: `~/Library/Application Support/slint-hmi-app/config.json`
- Linux: `~/.config/slint-hmi-app/config.json`

## Architecture

### Theme System
The theme system is built around three main components:

1. **Theme Globals**: Define color palettes for each theme
2. **ThemeManager**: Provides dynamic theme switching logic
3. **Widget Integration**: Each widget accepts a `current-theme` property

### Widget Design
Custom widgets follow these principles:
- Theme-aware styling using the ThemeManager
- Consistent API with standard properties (enabled, text, etc.)
- Smooth animations and transitions
- Responsive design with proper sizing

## Customization

### Adding New Themes
1. Define a new global theme in `ui/themes.slint`
2. Add the theme to the ThemeManager component
3. Update the theme selector in `ui/main.slint`

### Creating Custom Widgets
1. Define the widget in `ui/widgets.slint`
2. Import ThemeManager for theme-aware styling
3. Add the `current-theme` property for theme integration
4. Export the widget for use in other components

## Recent Updates

### New Features Added
- ✅ **Text File Viewer Card**: Added interactive text file display functionality
  - Displays text file contents with monospace font for code/data files
  - Multiple file selection buttons (Sample.txt, Config.json, README.md, Example.rs)
  - File information display (file path, size, line count)
  - Error handling for missing or unreadable files
  - Theme-aware styling and professional layout
  - Rust backend integration for file loading

- ✅ **Status Indicators Card**: Added comprehensive status visualization components
  - Status badges for Online, Warning, and Error states
  - Progress bar with percentage display
  - Animated connection status indicator with pulsing effect
  - Theme-aware styling for all status elements
  - Professional HMI-style visual indicators

### Screen Layout Issues Fixed
- ✅ Resolved overlapping text in Theme Information section
- ✅ Fixed missing card titles for Custom Buttons and Input Fields
- ✅ Improved layout spacing to prevent content overlap
- ✅ Enhanced card content containment and organization
- ✅ Optimized window height for better content display
- ✅ Fixed input field height consistency
- ✅ Improved vertical alignment and spacing

### UI Improvements
- Better visual hierarchy with proper spacing
- Consistent widget sizing and alignment
- Improved readability and user experience
- Enhanced theme switching visual feedback
- Added professional status indicators for HMI applications

## License

This project is open source and available under the MIT License.
