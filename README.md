# ayysee

`ayysee` is a language for writing programs that will run on ICs in [Stationeers](https://store.steampowered.com/app/544550/Stationeers/).
The ICs use an assembly language based on MIPS to allow for programmable logic in game.
Writing and refactoring programs can be a tedious and error-prone process.
Additionally there is a limitation on the number of lines of code available so having a higher level representation that can be full of comments and quality of life language features is beneficial.

Making a compiler for Stationeers MIPS has been done before, but none of the existing projects seem to work properly to the level that I can use them.
Having tried to use a few I have yet to succeed in replacing any of my IC programs due to bugs in the output and restrictions on syntax.
As a result `ayysee` was created to be a playground for practicing making a programming language and to write more robust programs for the game.

`ayysee` is currently in development, so there is little to no functionality that can be used in game.
Development will focus first on building an AST that can support the operations needed before emitting any assembly.
