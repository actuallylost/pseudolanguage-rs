# Pseudolanguage Documentation

Pseudolanguage is a simple programming language designed for educational purposes. It is a statically typed language and is used in Greek high schools.

## Basics

Pseudolanguage supports the following basic types:

- `ΑΚΕΡΑΙΑ` - 64-bit signed integer
- `ΠΡΑΓΜΑΤΙΚΗ` - 64-bit floating point number
- `ΧΑΡΑΚΤΗΡΑΣ` - 8-bit string
- `ΛΟΓΙΚΗ` - boolean

TODO - variable types

## Control Flow

Pseudolang supports `if` statements. Their syntax is as follows:

```
ΑΝ condition ΤΟΤΕ
    ! If condition is true, run some code
ΤΕΛΟΣ_ΑΝ
```

It also supports `else-if` and `else` clauses:

```
ΑΝ condition ΤΟΤΕ
    ! If condition is true, run some code
ΑΛΛΙΩΣ_ΑΝ otherCondition ΤΟΤΕ
    ! else if otherCondition is true, run some other code
ΑΛΛΙΩΣ
    ! else run this code
ΤΕΛΟΣ_ΑΝ
```

It also supports `switch` statements:

```
ΕΠΙΛΕΞΕ variable
    ΠΕΡΙΠΤΩΣΗ value
        ! If variable is equal to value, run some code
    ΠΕΡΙΠΤΩΣΗ condition
        ! If variable is equal to condition, run some other code
    ΠΕΡΙΠΤΩΣΗ ΑΛΛΙΩΣ
        ! If variable does not match any of the above, run this code
ΤΕΛΟΣ_ΕΠΙΛΟΓΩΝ
```

## Variables

TODO - variable declaration, assignment, scope

## Loops

TODO - do, while, for

## Functions

TODO - function declaration, calling, return

## Procedures

TODO - procedure declaration, calling

## Keywords

| Keyword | Uses | Description |
|---------|-------------|-------------|
| `ΑΛΗΘΗΣ` | Value | TRUE |
| `ΨΕΥΔΗΣ` | Value | FALSE |
| `ΜΕΤΑΒΛΗΤΕΣ` | Program, Function, Procedure | VARIABLES |
| `ΣΤΑΘΕΡΕΣ` | Program, Function, Procedure | CONSTANTS |
| `ΑΡΧΗ` | Program, Function, Procedure | BEGIN |
| `ΕΜΦΑΝΙΣΕ`, `ΓΡΑΨΕ`, `ΕΚΤΥΠΩΣΕ` | Program, Function, Procedure | PRINT |
| `ΔΙΑΒΑΣΕ` | Algorithm, Program, Function, Procedure | READ |
| `ΚΑΛΕΣΕ` | Procedure | CALL |
| `ΑΝ` | Algorithm, Program, Function, Procedure | IF |
| `ΤΟΤΕ` | After if condition | THEN |
| `ΑΛΛΙΩΣ_ΑΝ` | TODO | ELSE_IF |
| `ΑΛΛΙΩΣ` | TODO | ELSE |
| `ΤΕΛΟΣ_ΑΝ` | Mandatory to end an if expression | END_IF |
| `ΕΠΙΛΕΞΕ` | Algorithm, Program, Function, Procedure | SWITCH |
| `ΠΕΡΙΠΤΩΣΗ` | TODO | CASE |
| `ΠΕΡΙΠΤΩΣΗ ΑΛΛΙΩΣ` | Default value if no cases match | DEFAULT |
| `ΤΕΛΟΣ_ΕΠΙΛΟΓΩΝ` | Algorithm, Program, Function, Procedure | END_SWITCH |
| `ΟΣΟ` | Algorithm, Program, Function, Procedure | WHILE |
| `ΕΠΑΝΑΛΑΒΕ` | Algorithm, Program, Function, Procedure | REPEAT |
| `ΓΙΑ` | | FOR |
| `ΑΠΟ` | | FROM |
| `ΜΕΧΡΙ` | | TO |
| `ΜΕ_ΒΗΜΑ` | | STEP |
| `ΤΕΛΟΣ_ΕΠΑΝΑΛΗΨΗΣ` | While, For | END_REPEAT |
| `ΣΥΝΑΡΤΗΣΗ` | | FUNCTION |
| `ΑΚΕΡΑΙΑ` | | INTEGER |
| `ΠΡΑΓΜΑΤΙΚΗ` | | REAL |
| `ΧΑΡΑΚΤΗΡΑΣ` | | STRING |
| `ΛΟΓΙΚΗ` | | BOOLEAN |
| `ΤΕΛΟΣ_ΣΥΝΑΡΤΗΣΗΣ` | | END_FUNCTION |
| `ΔΙΑΔΙΚΑΣΙΑ` | | PROCEDURE |
| `ΤΕΛΟΣ_ΔΙΑΔΙΚΑΣΙΑΣ` | | END_PROCEDURE |
