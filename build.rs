fn main() {
    #[cfg(windows)]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/open-claude.ico")
            .set("ProductName", "Open Claude")
            .set("FileDescription", "Open Claude")
            .set("CompanyName", "Scalia Studio")
            .set("LegalCopyright", "Copyright (c) 2026 Scalia Studio contributors. MIT License.")
            .set("OriginalFilename", "OpenClaude.exe")
            .set("InternalName", "OpenClaude")
            .set("Comments", "It opens Claude. Not affiliated with Anthropic.");
        res.compile().expect("failed to embed Windows resources");
    }
}
