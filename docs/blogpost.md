# Von C++ nach Rust - Strategien für eine schrittweise Migration

Dieser Beitrag stellt die wesentlichen Vor- und Nachteile einer schrittweisen Migration dar.
Im ersten Teil folgt eine Einführung in die Interoperabilität von C++ und Rust.
Im zweiten Teil wird mit `cxx` ein Werkzeug vorgestellt, das die Erstellung von C-Schnittstellen nicht nur automatisiert, sondern auch deren korrekte Verwendung überprüft.

## Introduction

Die Migration von C++ nach Rust ist - wie jede Sprachmigration - mit vielen Unbekannten verbunden: 
Lässt sich der C++-Code mit einem Tool zuverlässig auf Rust übersetzen? 
Gibt es gebrauchte C++-Konstrukte, die gar keine direkte Entsprechung in Rust haben?
Oder, verhalten sich scheinbar äquivalente Objekte in C++ und Rust zur Laufzeit tatsächlich identisch?
Solche Unbekannten erschweren eine realistische Aufwandsschätzung für eine Migration und können sogar zum Scheitern des Vorhabens führen.

Eine praxistaugliche Alternative ist deshalb die schrittweise Migration, bei welcher nur isolierte Komponenten nach Rust portiert werden und über C-Schnittstellen mit dem Legacy-Code kommunizieren.
Besteht das Programm aus gut abgrenzbaren Komponenten, bietet der Ansatz mehere Vorteile:
Komplexe, aber stabile oder sicherheitskritische Komponenten können bewusst später angegangen werden, wenn Zeit und Ressourcen verfügbar sind.
Gleichzeitig sinkt das Risiko für ein Misslingen einer Migratio und nach jeder erfolgreichen Teilmigration steigt die Planbarkeit weiterer Schritte.

Allerdings ist dieser Ansatz nicht kostenlos.
Die C-Schnittstelle limitiert den Datenaustausch zwischen den Sprachen, was oft eine nicht triviale Umstrukturierung der Datenmodelle erfordert.
Zudem reduziert sie die Entwicklungsgeschwindigkeit, da die Schnittstelle als sprachübergreifender Vertrag gilt, dessen Änderung mit erheblichem Abstimmungs- und Anpassungsaufwand verbunden ist.


## Interoperabilität von C++ und Rust


## Automatisierung durch `cxx`


## Schlusswort


