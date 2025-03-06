# UTF8Reader 

This is a simple library which aids in reading UTF-8 encoded byte streams. It
takes in any implementer of the `Read` trait and returns an iterator of `char`s.

It also incorparates a lookahead struct which allows look ahead of many
characters. 

Suggestions and PRs are welcome.
