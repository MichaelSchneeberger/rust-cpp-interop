# Teil 2 - Grenzen der C++- und Rust-Interoperabilität

## Abstract

Dieser Artikel stellt den zweiten Beitrag zu einer vierteiligen Serie über die Migration und Integration von C++ in Rust dar.

## Vorwort

Wie im letzten Teil unserer Serie gezeigt, ist die Integration von C++ in Rust technisch möglich, geht jedoch mit Einschränkungen einher.
Diese beruhen darauf, dass die in Rust und C++ kompilierten Komponenten über eine C-kompatible Schnittstelle - ein sogenanntes Foreign Function Interface (FFI) - kommunizieren müssen.
Die Schnittstelle limitiert einerseits den Datenaustausch zwischen den Sprachen; andererseits erhöht sie den Entwicklungsaufwand - sowohl durch die notwendige Übersetzung von komplexer Datenstrukturen als auch durch die fortlaufende Pflege dieser Schnittstelle.
Daher ist es wichtig, die Grenzen der Schnittstelle zu kennen und zu verstehen: Nur so lassen sich der Einführungsaufwand realistisch abschätzen, die technischen Risiken bewerten und eine fundierte Entscheidung treffen, ob und in welchem Umfang eine Integration von C++ in Rust überhaupt sinnvoll ist.

Dies ist der zweite Teil unserer Artikel-Serie über die Integration und Migration von Rust in C++; er beleuchtet die Grenzen der Interoperabilität.

## Performance

Die Einführung einer FFI-Schnittstelle führt im Idealfall zu geringen oder gar keinen Leistungseinbussen.
Auch reine C++-Programme kommunizieren intern über binäre Schnittstellen zwischen Objektdateien, die der Linker zu einem gemeinsamen Programm zusammenführt.
Würde eine solche interne binäre Schnittstelle von der jeweils anderen Sprache perfekt nachgebildet, liessen sich potenzielle Leistungsverluste vollständig vermeiden.
In der Praxis ist dieses Szenario jedoch nur schwer zu erreichen:
Zum einen sind Optimierungen wie Inlining oder Dead-Code-Elimination des Compilers über die Sprachgrenze hinweg nicht möglich.
Zum anderen müssen Objekte, die über die Sprachgrenze hinweg verwendet werden, bitgenau übereinstimmen.
Das ist fehleranfällig und erfordert bei jeder Anpassung der binäre Speicherauslegung eine sorgfältige, manuelle Anpassung auf beiden Seiten.
Damit wächst das Risiko, dass selbst kleine Änderungen zu undefiniertem Verhalten oder subtilen Fehlern führen, welche erst spät in der Software-Release-Pipeline entdeckt werden.
Eine verbreitete Alternative ist die Verwendung opaker Datentypen wie im nächsten Kapitel beschrieben.

## Opake Datentypen

Ein zentrales Konzept zum Verständnis der Interoperabilitätsgrenzen sind opake Datentypen.
Zur Veranschaulichung kann man sich eine Analogie zu einem Verkaufsstand für Eismaschinen vorstellen (siehe Abbildung unten): Der Verkäufer und der Kunde repräsentieren die beiden Programmiersprachen, während der Verkaufsstand der FFI-Schnittstelle entspricht.
Der Verkäufer übergibt dem Kunden das Produkt ausschließlich über diesen Stand – genauso wie eine Sprache ein Datenobjekt über die FFI-Schnittstelle an die andere übergibt.
Die Produktübergabe (also der Datenaustausch) kann auf zwei Arten erfolgen:

1. Stack-Allokation -
Die Eismaschine ist modular aufgebaut und sofort verfügbar, jedoch anfällig für Störungen, wenn sie nicht korrekt gehandhabt wird.
Dies entspricht Daten, die auf dem Stack liegen: Sie lassen sich sehr effizient übertragen, müssen jedoch auf Binärebene in beiden Sprachen exakt gleich dargestellt werden.
Bei komplexen Datenstrukturen entsteht zusätzlicher Aufwand, weil sie zunächst auf einfache, C‑kompatible Grundtypen abgebildet werden müssen.

2. Heap-Allokation -
Die Eismaschine ist erst später lieferbar, dafür aber einfach zu bedienen und wenig wartungsintensiv.
Dies entspricht Daten, die auf dem Heap liegen: Die Allokation bringt zwar spürbare Verzögerungen mit sich, ermöglicht aber, den Datentyp opak zu halten.
Die interne Beschaffenheit des Produkts bleibt verborgen; der Kunde erhält lediglich einen "Griff" (Handle bzw. Zeiger) und interagiert ausschließlich über klar definierte Funktionen.
Dadurch sinken Kopplung und Fehlerrisiko: Änderungen an der internen Datenstruktur erfordern keine Anpassungen auf der Gegenseite, solange die öffentliche Schnittstelle stabil bleibt.

Ein zweiter Anwendungsfall opaker Datentypen ist die Einbindung von Datenstrukturen, deren Binärrepräsentation instabil oder vom Compiler abhängig ist.
Dazu gehören etwa Dateien- oder Socket-Handler, Closures, Mutex-Implementierungen oder andere komplexe Laufzeitobjekte.
Solche Typen können sich je nach Compiler, Plattform oder Version der Standardbibliothek unterscheiden und sind daher nicht zuverlässig direkt über die FFI abbildbar.
<!-- Durch die Verwendung eines opaken Handles wird die interne Struktur vollständig verborgen, während die Gegenseite nur über eine klar definierte API interagiert.
Dadurch bleibt die Schnittstelle stabil, auch wenn sich die interne Implementierung ändert. -->

Zusammengefasst bieten opake Datentypen Vorteile in der Benutzerfreundlichkeit, zu Lasten der Performance.

## Selbstreferenzieller Datentypen

Die bisher besprochenen Grenzen der Interoperabilität gelten allgemein für FFI zwischen zwei Sprachen. Es gibt jedoch eine spezifische Inkompatibilität zwischen C++ und Rust, die auf selbstreferenzielle Datentypen zurückzuführen ist.
Ein einfaches Beispiel ist ein Objekt, das einen Pointer auf sich selbst enthält.
In C++ treten solche Konstrukte beispielsweise bei Iteratoren, Strings oder Vektoren auf.
Rust hingegen verbietet die freie Verwendung selbstreferenzieller Datentypen und geht davon aus, dass solche Objekte – etwa Futures – nur in Verbindung mit einer Pin-Markierung existieren.

Über die FFI-Schnittstelle können jedoch selbstreferenzielle Objekte aus C++ ohne Pin-Markierung in Rust eingeführt werden.
Dies muss unbedingt vermieden werden.
Der Grund: Rust nimmt an, dass Objekte, die nicht gepinnt sind, bitweise verschiebbar sind.
Wird ein selbstreferenzielles Objekt jedoch bitweise verschoben, zeigt der interne Pointer nach der Verschiebung nicht mehr auf das eigene Objekt, sondern auf die ursprüngliche Speicheradresse.
Dies führt unweigerlich zu undefiniertem Verhalten.

## Weitere Aspekte



LTO/PGO wirken meist nicht über Sprachgrenzen; Sanitizer (ASan/TSan/UBSan) müssen auf beiden Seiten konsistent aktiviert werden. Empfehlung: Gemeinsame Build-Matrix, Fuzzing und Grenzflächentests (ABI-/Layout-Checks).

## Summary

## Fazit
