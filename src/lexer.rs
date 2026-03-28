use santiago::lexer::LexerRules;

pub fn lexer_rules() -> LexerRules {
  santiago::lexer_rules!(

        "DEFAULT" | "NUMBER" = pattern r"[0-9]+";
        "DEFAULT" | "FORWARD" = string r"forward";
        "DEFAULT" | "BACKWARD" = string r"backward";
        "DEFAULT" | "LEFT" = string r"left";
        "DEFAULT" | "RIGHT" = string r"right";
        "DEFAULT" | "WS" = pattern r"\s+" => |lexer| lexer.skip();
    )
}