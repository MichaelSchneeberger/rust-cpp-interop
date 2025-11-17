# Teil 2 - Grenzen der C++- und Rust-Interoperabililtät

## Abstract

Dieser Artikel stellt den zweiten Beitrag zu einer vierteiligen Serie über die Migration und Integration von C++ in Rust dar.

## Vorwort

Wie im letzten Teil unserer Serie gezeigt, ist die Integration von C++ in Rust technisch möglich, geht jedoch mit Einschränkungen einher.
Diese beruhen darauf, dass die in Rust und C++ kompilierten Komponenten über eine C-kompatible Schnittstelle - eine sogenannt Foreign Function Interface (FFI) - kommunizieren müssen.
Die Schnittstelle limitiert einerseits den Datenaustausch zwischen den Sprachen; andererseits erhöht sie den Entwicklungsaufwand - sowohl durch die notwendige Übersetzung von komplexer Datenstrukturen als auch durch die fortlaufende Pflege dieser Schnittstelle.
Daher ist es wichtig, die Grenzen der Schnittstelle zu kennen und zu verstehen: Nur so lassen sich der Einführungsaufwand realistisch abschätzen, die technischen Risiken bewerten und eine fundierte Entscheidung treffen, ob und in welchem Umfang eine Integration von C++ in Rust überhaupt sinnvoll ist.

Dies ist der zweite Teil unserer Artikel-Serie über die Integration und Migration von Rust in C++; er beleuchtet die Grenzen der Interoperabilität.

## Performance

Die Einführung einer FFI-Schnittstelle führt im Idealfall zu geringen oder gar keinen Leistungseinbußen. 
Auch reine C++-Programme kommunizieren intern über binäre Schnittstellen zwischen Objektdateien, die der Linker zu einem gemeinsamen Programm zusammenführt.
Würde eine solche interne binäre Schnittstelle von der jeweils anderen Sprache perfekt nachgebildet, ließen sich potenzielle Leistungsverluste vollständig vermeiden.
In der Praxis ist dieses Szenario jedoch nur schwer zu erreichen: Objekte, die über die Sprachgrenze hinweg verwendet werden, müssen bitgenau übereinstimmen.
Dies ist nicht nur fehleranfällig, sondern erfordert bei jeder Änderung der Schnittstelle eine sorgfältige, manuelle Anpassung auf beiden Seiten.
Damit wächst das Risiko, dass selbst kleine Änderungen zu undefiniertem Verhalten oder subtilen Fehlern führen, wenn sie nicht in beiden Sprachen korrekt reflektiert werden.
Eine verbreitete Alternative ist die Verwendung opaker Datentypen, die diese Probleme reduzieren, indem sie die interne Struktur eines Objekts verbergen und nur einen klar definierten Zugriff über Funktionen erlauben.

## Opaque Types

Eine zentrales Konzept zum Verständnis der Interoperabilitätsgrenzen sind opake Datentypen.
Um dieses Konzept zu veranschaulichen, kann man sich eine Analogie zu einem Verkaufstand für Eismaschinen vorstellen (siehe Abbildung unten):

* der Verkäufer repräsentiert eine Programmiersprache
* der Kunde die andere Sprache
* der Verkaufsstand entspricht der FFI-Schnittstelle

Der Verkäufer übergibt dem Kunden das Produkt ausschließlich über den Verkaufsstand – genau wie eine Sprache ein Datenobjekt über die FFI-Schnittstelle der anderen Sprache übergibt.






<!-- So wird die interne Struktur verborgen, der Zugriff kontrolliert und die Interaktion sicherer gestaltet.

stellen wir eine Analogie zu einem Verkaufsstand für Eismaschinen auf (siehe unteres Bild).
Der Verkäufer und der Kunde repräsentieren die beiden Programmiersprachen.
Der Verkäufer übergibt dem Kunden das Produkt über den Verkaufsstand so wie eine Sprache eine Datenobjekt über die FFI-Schnittstelle der anderen Sprache übergibt.

die vom Compiler/Linker optimiert werden können.
Über die Sprachgrenze hinweg sind solche Optimierungen jedoch in der Regel nicht möglich: FFI-Aufrufe werden nicht inline gesetzt, und aggressive Optimierungen über das Interface hinweg entfallen.

In der Praxis kommen jedoch noch zusätzliche Arbeitsaufwände hinzu.

Der eigentliche Overhead entsteht meist durch Datenkonvertierungen, zusätzliche Kopier- oder Allokationsvorgänge sowie durch konservative Fehlerbehändlung am Schnittstellenrand. -->



## Async Programming

## Debugging

## Fazit
