# PaintMate File Operations

## File Dialogs Implemented

### Open File Dialog
- **Trigger**: File → Open (Ctrl+O)
- **Supported formats**: PNG, JPG, JPEG, GIF, BMP, TIFF, WebP
- **Features**: 
  - Asynchronous file loading using threaded file dialogs
  - Automatic format detection
  - Proper error handling with logging

### Save File Dialog  
- **Trigger**: File → Save As (Ctrl+Shift+S)
- **Supported formats**: PNG, JPEG, GIF, BMP, TIFF
- **Features**:
  - Format-specific file filters
  - Asynchronous saving
  - Overwrites current file path when saving

### Save (Quick Save)
- **Trigger**: File → Save (Ctrl+S)
- **Behavior**:
  - If file has been saved before: overwrites existing file
  - If new file: opens Save As dialog

### Export Dialog
- **Trigger**: File → Export
- **Supported formats**: PNG, JPEG, TIFF, BMP
- **Features**:
  - Same as Save As but for export-specific workflows
  - Does not change the current file path

## Technical Implementation

### Architecture
- **Channel-based communication**: Uses `std::sync::mpsc` for thread-safe file operations
- **Async file dialogs**: File dialogs run in separate threads to avoid blocking the UI
- **Error handling**: Comprehensive error logging for failed operations

### File Operation Flow
1. User clicks menu item or uses keyboard shortcut
2. File dialog spawns in background thread
3. User selects file in native OS dialog
4. File path sent through channel to main thread
5. Main thread processes file operation on next update cycle
6. Success/error logged appropriately

### Supported Image Formats
- **Reading**: PNG, JPG, JPEG, GIF, BMP, TIFF, WebP
- **Writing**: PNG, JPEG, GIF, BMP, TIFF

### Dependencies
- `rfd`: Native file dialogs
- `image`: Image processing and format support
- `anyhow`: Error handling
- `std::sync::mpsc`: Thread communication

## Testing
A test image has been created at `test_images/test_image.png` for testing the open functionality.

## Future Enhancements
- [ ] Recent files menu
- [ ] Drag and drop file support
- [ ] Batch file operations
- [ ] Format conversion tools
- [ ] Preview thumbnails in file dialogs
