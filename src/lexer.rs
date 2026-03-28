use santiago::lexer::LexerRules;

pub fn lexer_rules() -> LexerRules {
  santiago::lexer_rules!(

        "DEFAULT" | "NUMBER" = pattern r"[0-9]+";
        "DEFAULT" | "FORWARD" = string r"forward";
        "DEFAULT" | "BACKWARD" = string r"backward";
        "DEFAULT" | "LEFT" = string r"left";
        "DEFAULT" | "RIGHT" = string r"right";
        // états
        "DEFAULT" | "PENUP" = string r"penup";
        "DEFAULT" | "PENDOWN" = string r"pendown";
        // boucle
        "DEFAULT" | "REPEAT" = string r"repeat";
        // blocs
        "DEFAULT" | "LBRACKET" = string r"[";
        "DEFAULT" | "RBRACKET" = string r"]";
        "DEFAULT" | "WS" = pattern r"\s+" => |lexer| lexer.skip();
    )
}