# Von C++ nach Rust - Strategien für eine schrittweise Migration

Die Migration von C++ nach Rust ist - wie jede Sprachmigration - mit vielen Unbekannten verbunden: 
Lässt sich der C++-Code mit einem Tool zuverlässig auf Rust übersetzen? 
Gibt es gebrauchte C++-Konstrukte, die keine direkte Entsprechung in Rust haben?
Verhalten sich scheinbar äquivalente Objekte zur Laufzeit tatsächlich identisch?
Solche Unbekannten erschweren eine realistische Aufwandsschätzung für eine vollständige Migration in einem einzigen Schritt.

Eine praxistaugliche Alternative ist deshalb die schrittweise Migration, bei welcher nur isolierte Teile nach Rust portiert werden und über C-Schnittstellen mit dem Legacy-Code kommunizieren.
Voraussetzung hierfür ist, dass das System aus klar abgrenzbaren Komponenten besteht.
Dann bietet der Ansatz mehere Vorteile:
Komplexe, aber stabile oder sicherheitskritische Komponenten können bewusst später angegangen werden, wenn Zeit und Ressourcen verfügbar sind.
Gleichzeitig sinkt das Risiko eines Fehlschlags, und nach jeder erfolgreichen Teilmigration steigt die Planbarkeit weiterer Schritte.

Allerdings ist dieser Ansatz nicht kostenlos:
Die C-Schnittstelle limitiert den Datenaustausch zwischen den Sprachen, was oft eine nicht triviale Umstrukturierung der Datenmodelle erfordert.
Zudem reduziert sie die Entwicklungsgeschwindigkeit, da die Schnittstelle als sprachübergreifender Vertrag gilt, dessen Änderung mit erheblichem Abstimmungs- und Anpassungsaufwand verbunden ist.

Dieser Beitrag stellt die wesentlichen Vor- und Nachteile einer schrittweisen Migration dar.
Im ersten Teil folgt eine Einführung in das Problem der Interoperabilität von C++ und Rust.
Im zweiten Teil wird mit `cxx` ein Werkzeug gezeigt, das die Erstellung solcher C-Schnittstellen nicht nur automatisiert, sondern auch deren korrekte Verwendung überprüft.

## Interoperabilität von C++ und Rust


## Automatisierung durch `cxx`


## Schlusswort


