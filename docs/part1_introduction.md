# Teil 1 - Interoperabilität von C++ in Rust - Grundlagen

## Abstract

Dieser Artikel stellt der erste Beitrag zu einer dreiteiligen Serie über die Integration von C++ in Rust dar.


## Introduction

Viele Unternehmen stehen derzeit vor derselben Frage: Soll eine bestehende C++-Codebasis durch eine sichere Rust-Implementierung teilweise oder langfristig vollständig ersetzt werden.
Gerade Hersteller mit hohen Sicherheits- oder Compliance-Anforderungen (z.B. im Kontext des EU Cyber REsilience Act) geraten zunehmend unter Druck und sind bereit gezielt in Massnahmen zur Erhöhung der Softwaresicherheit zu investieren.
Der Rust-Compiler kann durch ein konservatives Ownership- und Borrowing-Modell Speicherzugriff- und Racing-Conditions-Fehler verhindern.
Das ist deshalb wichtig, weil so ganze Klassen von sporadisch auftretender Laufzeitfehler bereits zur Compile-Zeit verhindert werden können -- Fehler, die in C++ oft erst im Feld auffallen und nur schwer reproduzierbar sind.
Dies steigert nachweisbar die Zuverlässigkeit und somit das Vertrauen in das Produkt.

Die Migration von C++ nach Rust ist jedoch auch mit vielen Unbekannten verbunden:
Lässt sich der C++-Code mit einem Tool zuverlässig auf Rust übersetzen? 
Gibt es gebrauchte C++-Konstrukte, die gar keine direkte Entsprechung in Rust haben?
Oder, verhalten sich scheinbar äquivalente Objekte in C++ und Rust zur Laufzeit tatsächlich identisch?
Solche Unbekannten erschweren eine realistische Aufwandsschätzung für eine Migration und können sogar zum Scheitern des Vorhabens führen.

Eine praxistaugliche Alternative ist deshalb die schrittweise Migration, bei welcher nur isolierte Komponenten nach Rust portiert werden und über C-Schnittstellen mit dem Legacy-Code kommunizieren.
Besteht das Programm aus gut abgrenzbaren Komponenten, bietet der Ansatz mehere Vorteile:
Komplexe, aber stabile oder sicherheitskritische Komponenten können bewusst später angegangen werden, wenn Zeit und Ressourcen verfügbar sind.
Gleichzeitig sinkt das Risiko für ein Misslingen einer Migration und nach jeder erfolgreichen Teilmigration steigt die Planbarkeit weiterer Schritte.
Zudem ist eine Teilmigration firmenpolistisch weitaus weniger heikel wie eine Vollmigration.

Allerdings ist dieser Ansatz nicht kostenlos.
Die C-Schnittstelle limitiert den Datenaustausch zwischen den Sprachen, was oft eine nicht triviale Umstrukturierung der Datenmodelle erfordert.
Zudem reduziert sie die Entwicklungsgeschwindigkeit, da die Schnittstelle als sprachübergreifender Vertrag gilt, dessen Änderung mit erheblichem Abstimmungs- und Anpassungsaufwand verbunden ist.

Die dreiteilige Serie über die Integration von C++ in Rust führt unsere allgemeine Artikel-Serie mit der letzten Ausgabe [Rust - Moderne Softwareentwicklung mit Sicherheit und Performance](https://cudos.ch/de/news-insights/rust-moderne-softwareentwicklung-mit-sicherheit-und-performance/) weiter.
Der erste Teil fasst die technischen Grundlagen der C++-Rust-Interoperabilität als Mittel zur schrittweisen Migration zusammen.
In einem zweiten Teil, werden die Grenzen dieses Ansatzes besprochen.
Der dritte Teil zeigt auf wie die Sicherheit durch Einbettung von C++ code in Rust verbessert werden kann.

## Das Sanduhr-Modell

Der C++- und Rust-Code kommunizieren miteinander mittels C-kompatiblen Schnittstelle (wie im Bild dargestellt).
In C++ enspricht diese Schnittstelle einer Teilmenge der Standard-C++-Sprachkonstrukte, während sie in Rust über das Modul std::ffi (Foreign Function Interface) bereitgestellt wird.
Datentype, die in einer Sprache definiert und in der anderen Sprache verwendet werden sollen, müssen zunächst auf C-kompatiblen Grundtypen reduziert und in der anderen Sprache wieder zusammengebaut werden.
Zum Beispiel muss ein std::vector in C++ als C-Array uminterpretiert werden, um schliesslich in Rust zu einem Vec zusammengesetzt zu werden.
Dieses Prinzip wird als *Sanduhr-Modell* (*hourglass model*) bezeichnet:
Die C-Schnittstelle bildet den schmalen, gemeinsamen Kern ("Hals") zwischen den beiden Sprachen, während auf beiden Seiten komplexe und typsichere Abstraktionen in c++ und Rust bestehen bleiben können.

<img src="images/hourglass_model.png" alt="hourglass_model" width="200"/>

Für ein konkretes Project wird mithilfe der C-Schnittstelle ein Application Binary Interface (ABI) definiert.
Dieses beschreibt, wie Funktionen und Datenstrukturen auf Binärebene zwischen zwei Programmiersprachen ausgetauscht werden.
Es umfasst also die Aufrufkonventionen, die Speicheranordung von Datenstrukturen sowie die Namenkonventionen (Name-Mangling).
Über dieses ABI können Funktionen, die in einer Sprache implementier wurden, von der anderen aufgerufen werden.
Im Gegensatz dazu beschreibt ein Application Programming Interface (API) die Schnittstelle auf Quellcode und nicht auf Maschinencode-Ebende.

Diese Unterscheidung wird deutlich, wenn man den Build-Prozess betrachtet:
Rust- und C++-Compiler übersetzen ihren jeweiligen Quellcode unabhängig voneinander in Objektdateien (.o).
Erst im zweiten Schritt werden diese Objektdateien durch den Linker zu einem gemeinsamen Binärprogramm zusammengefügt — dieser Vorgang erfolgt ohne direkte Kontrolle durch einen der Compiler.
Während die Verlinkung von Objektdateien innerhalb einer Sprache vom jeweiligen Compiler garantiert korrekt funktioniert, ist die Verknüpfung zwischen Rust- und C++-Objekten fehleranfälliger.
Hier muss sichergestellt werden, dass beide Seiten das gleiche ABI einhalten — sonst kann es zu undefiniertem Verhalten, Speicherfehlern oder Abstürzen kommen.

