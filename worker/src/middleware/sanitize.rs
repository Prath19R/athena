pub fn sanitize_user_input(input: &str) -> String {
    let trimmed = input.trim();
    let capped = if trimmed.len() > 1500 { &trimmed[..1500] } else { trimmed };

    let lower = capped.to_lowercase();
    let injection_signatures = [
        "ignore all previous",
        "ignore prior instructions",
        "system prompt",
        "you are now",
        "reveal your prompt",
        "developer message",
    ];

    for sig in injection_signatures {
        if lower.contains(sig) {
            return "Could you please rephrase that question? I am Athena, the AI assistant here to answer questions about Prathmesh's software engineering background, projects, and skills.".into();
        }
    }

    capped.to_string()
}
