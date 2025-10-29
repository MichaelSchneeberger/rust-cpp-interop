# Teil 1 - Interoperabilität von C++ in Rust - Grundlagen

## Abstract

Dieser Artikel stellt der erste Beitrag zu einer dreiteiligen Serie über die Integration von C++ in Rust dar.


## Introduction

Viele Unternehmen stehen derzeit vor derselben Frage: Soll eine bestehende C++-Codebasis durch eine sicheren Rust-Implementierung ergänzt oder langfristig vollständig ersetzt werden.
Gerade Hersteller mit hohen Sicherheits- oder Compliance-Anforderungen (z.B. im Kontext des EU Cyber REsilience Act) geraten zunehmend unter Druck und sind bereit gezielt in Massnahmen zur Erhöhung der Softwaresicherheit zu investieren.
Der Rust-Compiler kann durch eine konservativere Programmiersprache (Ownership- und Borrowing-Modell) Sicherheitsgarantien geben, insbesondere im Umgang mit Speicherzugriffen und Racing-Conditions.
Das ist deshalb so wichtige, weil ganze Klassen sporadisch auftretender Laufzeitfehler bereits zur Compile-Zeit verhindert werden können -- Fehler, die in C++ oft erst im Feld auffallen und nur schwer reproduzierbar sind.
Solche Sicherheitgarantien steigern nachweisbar die Zuverlässigkeit und damit das Vertrauen in das Produkt.

Die Migration von C++ nach Rust ist jedoch auch mit vielen Unbekannten verbunden:
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

Die dreiteilige Serie über die Integration von C++ in Rust führt unsere allgemeine Artikel-Serie mit der letzten Ausgabe [Rust - Moderne Softwareentwicklung mit Sicherheit und Performance](https://cudos.ch/de/news-insights/rust-moderne-softwareentwicklung-mit-sicherheit-und-performance/) weiter.
Der erste Teil fasst die technischen Grundlagen der C++-Rust-Interoperabilität als Mittel zur schrittweisen Migration zusammen.
In einem zweiten Teil, werden die Grenzen dieses Ansatzes besprochen.
Der dritte Teil zeigt auf wie die Sicherheit durch Einbettung von C++ code in Rust verbessert werden kann.

## Grundlagen

Der C++- und Rust-Code kommunizieren miteinander über ihren kleinsten gemeinsamen Nenner: Ein integriertes C-interface.
Dieses C-Interface definiert ein stabiles und sprachen neutrales Application Binary Interface (ABI), durch welches die kompilierten Dateien vom C++- und Rust-Compiler mit einem Linker zu einem Programm vereint werden können.
Der Umfang dieses Interfaces ist mit Bezug auf die Zuverlässigkeit bewusst einfach gehalten (Hourglass model).
Dadurch müssen aber auch die komplexen Datentypen über diese Schnittstelle von C++ nach Rust aufwändig übersetzt werden.
Zum Beispiel muss ein std::vector in C++ als C-Array uminterpretiert werden, um schliesslich in Rust zu einem Vec zusammengesetzt zu werden.


