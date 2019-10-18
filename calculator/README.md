# Ausdrucksberechner
Lösung von Praktikumsaufgabe 1, Lehrveranstaltung Compilerbau, HTW Dresden

## Benutzung
Der Ausdrucksberechner lässt sich aus dem Sourcetree heraus ausführen mit dem Kommando `cargo run`.
Es wird dann eine Hilfeseite angezeigt, die die weiteren Schritte erläutert.

### Beispiel
`cargo run iterative "12/3*2"` sollte die Ausgabe `8` liefern


## Struktur
Das Projekt ist unterteilt in die beiden Submodule `lexer` und `parser` sowie in eine simple CLI in `main.rs`.

Das Submodul `lexer` stellt einen einfachen Lexer für Mathematische Ausdrücke zur Verfügung.

Das Submodul `parser` stellt mehrere verschiedene Parseransätze zur Verfügung und enthält darüber hinaus
private Funktionen, die parserübergreifend geteilt werden (z.B. gemeinsame Parser-Regeln).
Die Parser berechnen jeweils einen Mathematischen Ausdruck als 64bit floating point.
Manche dieser Parser demonstrieren fehlerhaftes Verhalten, so rechnet der naive Parser z.B. von rechts nach links.


## Unit-Tests
Ein Testlauf lässt sich im Sourcetree mit dem Kommando `cargo test` anstoßen.
Es handelt sich um simple Tests, die den Lexer sowie alle Parser auf grundsätzliche
Funktionsfähigkeit testen sollen.

Die Testfunktionen befinden sich direkt in den jeweiligen Sourcecode-Modulen.