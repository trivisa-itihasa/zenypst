export default {
  // Common
  common: {
    cancel: "Cancel",
    create: "Create",
    save: "Save",
    delete: "Delete",
    close: "Close",
    thisActionCannotBeUndone: "This action cannot be undone.",
  },

  // Toolbar
  toolbar: {
    aboutZenypst: "About Zenypst",
    file: "File",
    newFile: "New File",
    openFile: "Open File\u2026",
    openFolder: "Open Folder\u2026",
    saveFile: "Save",
    saveAs: "Save As\u2026",
    exportPdf: "Export PDF\u2026",
    manageTemplates: "Manage Templates",
    minimize: "Minimize",
    restore: "Restore",
    maximize: "Maximize",
    closeWindow: "Close",
    aboutDescription: "A desktop Typst editor with live PDF preview.",
    aboutVersion: "Version 0.1.0",
    aboutBuiltWith: "Built with Tauri, Vue 3, Quasar, and CodeMirror 6.",
    pdfExportedSuccessfully: "PDF exported successfully.",
    exportFailed: "Export failed: {msg}",
  },

  // Activity bar
  activityBar: {
    toggleExplorer: "Toggle Explorer",
    togglePdfViewer: "Toggle PDF Viewer",
    settings: "Settings",
  },

  // Status bar
  statusBar: {
    compiling: "Compiling\u2026",
    compiled: "\u2713 Compiled",
    errorCount: "\u2717 {count} error(s)",
    idle: "Idle",
    realtime: "Real-time",
    onSave: "On Save",
    manual: "Manual",
    noFileOpen: "No file open",
  },

  // App shell
  appShell: {
    typstNotFound: "Typst CLI not found.",
    typstInstallFrom: "Install from",
    typstInstallSite: "typst.app",
    typstInstallSuffix: "and add to PATH.",
    templateManager: "Template Manager",
  },

  // File tree
  fileTree: {
    explorer: "Explorer",
    refresh: "Refresh",
    closeFolder: "Close Folder",
    openFolder: "Open Folder",
    openFolderToBrowse: "Open a folder to browse files",
    newFile: "New File",
    fileName: "File name",
    newFolder: "New Folder",
    folderName: "Folder name",
  },

  // File tree item
  fileTreeItem: {
    newFile: "New File",
    fileName: "File name",
    fromTemplate: "From Template",
    newFolder: "New Folder",
    folderName: "Folder name",
    deleteConfirm: 'Delete "{name}"?',
    deleteItem: "Delete",
    revealInFileManager: "Reveal in File Manager",
  },

  // Editor panel
  editorPanel: {
    openFileToStart: "Open a file to start editing",
    newFile: "New File",
    openFile: "Open File",
  },

  // Tab bar
  tabBar: {
    noFilesOpen: "No files open",
    compile: "Compile",
  },

  // PDF viewer
  pdfViewer: {
    emptyState: "PDF preview will appear here",
  },

  // Compile status
  compileStatus: {
    compiling: "Compiling\u2026",
    compiledSuccessfully: "Compiled successfully",
    errorCount: "{count} error(s)",
    idle: "Idle",
    compilationErrors: "Compilation Errors",
    hint: "hint: {hint}",
  },

  // Settings dialog
  settings: {
    title: "Settings",
    editor: "Editor",
    theme: "Theme",
    preview: "Preview",
    appearance: "Appearance",
    typst: "Typst",
    darkMode: "Dark Mode",
  },

  // Font settings
  fontSettings: {
    title: "Font Settings",
    fontFamily: "Font Family",
    fallbackFont: "Fallback Font",
    fontSize: "Font Size (px)",
    showLineNumbers: "Show Line Numbers",
    wordWrap: "Word Wrap",
  },

  // Theme editor
  themeEditor: {
    syntaxTheme: "Syntax Theme",
    newTheme: "New Theme",
    builtIn: "Built-in",
    editTheme: "Edit Theme",
    themeName: "Theme Name",
    deleteTheme: "Delete Theme?",
    // Color field labels
    background: "Background",
    text: "Text",
    cursor: "Cursor",
    selection: "Selection",
    lineHighlight: "Line Highlight",
    gutterBackground: "Gutter Background",
    gutterText: "Gutter Text",
    heading: "Heading",
    italic: "Italic",
    bold: "Bold",
    keyword: "Keyword",
    function: "Function",
    string: "String",
    number: "Number",
    comment: "Comment",
    math: "Math",
    labelReference: "Label / Reference",
    rawCode: "Raw / Code",
    operator: "Operator",
    bracket: "Bracket",
  },

  // Preview settings
  previewSettings: {
    title: "Preview Settings",
    updateMode: "Update Mode",
    realtime: "Real-time",
    realtimeDescription: "Compile automatically as you type",
    onSave: "On Save",
    onSaveDescription: "Compile when you save the file (Ctrl+S)",
    manual: "Manual",
    manualDescription: "Compile only when you click the compile button",
    debounce: "Debounce (ms)",
  },

  // Typst settings
  typstSettings: {
    title: "Typst",
    bundledMessage: "Typst is bundled directly into Zenypst \u2014 no external CLI is required.",
  },

  // Template picker
  templatePicker: {
    title: "New File \u2014 Choose Template",
    searchPlaceholder: "Search templates\u2026",
    noTemplatesFound: "No templates found",
  },

  // Template manager
  templateManager: {
    title: "Template Manager",
    newTemplate: "New Template",
    duplicate: "Duplicate",
    edit: "Edit",
    deleteTemplate: "Delete Template?",
  },

  // Template editor
  templateEditor: {
    duplicateTemplate: "Duplicate Template",
    editTemplate: "Edit Template",
    newTemplate: "New Template",
    templateName: "Template Name",
    description: "Description",
    content: "Content",
    saveTemplate: "Save Template",
  },
};
