use santiago::grammar::Grammar;

pub fn grammar() -> Grammar<()> {
    santiago::grammar!(
        "program" => rules "command" "program";
        "program" => empty;

        "command" => rules "order" "number" ;

        "number" => lexemes "NUMBER";

        "order" => lexemes "FORWARD";
        "order" => lexemes "BACKWARD";
        "order" => lexemes "LEFT";
        "order" => lexemes "RIGHT";
        

    )
}