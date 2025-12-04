# Teil 2 - Technische Aspekte der C++- und Rust-Interoperabilität

## Vorwort

Wie wir im ersten Teil unserer Serie gezeigt haben, bietet die Interoperabilität zwischen Rust und C++ grosses Potential bestehende Systeme schrittweise zu integrieren und sicherer zu gestalten.
Dank Bibliotheken wie [cxx](https://cxx.rs/) ist die Integration von C++-Komponenten in Rust heute gut realisierbar.
Ob eine schrittweise Migration jedoch sinnvoll ist, hängt stark von den individuellen Anforderungen eines Projekts ab.
Entscheidend ist dabei das Verständnis der technischen Grenzen.
Sie resultieren daraus, dass C++- und Rust-Komponenten über eine gemeinsame C-kompatible Schnittstelle - ein sogenanntes Foreign Function Interface (FFI) - kommunizieren müssen.
Diese Schnittstelle beeinflusst sowohl die Performance und die Handhabung komplexer Datentypen als auch die fehlenden Compiler-Garantien und die Debugging-Möglichkeiten - Faktoren, die für die Risikoanalyse und Planung einer Integration wichtig sind.

In diesem zweiten Teil unserer Serie beleuchten wir die zentralen technischen Aspekte der Interoperabilität:

* **Performance** - Eine FFI-Schnittstelle bringt zwangsläufig gewisse Performance-Kosten mit sich, welche jedoch im Vergleich zu alternativen Ansätzen oft gering ausfallen.
* **Opake Datentypen** - Manche Datenstrukturen lassen sich nicht direkt über das FFI abbilden und müssen daher als opake Typen eingekapselt werden. Dies führt zu Einschränkungen hinsichtlich der Möglichkeit, wie auf diese Daten zugegriffen werden kann.
* **"Move"-Verhalten** - C++ und Rust unterscheiden sich grundlegend darin, wie Objekte verschoben und referenziert werden. Beim Übergeben solcher Objekte über Sprachgrenzen hinweg kann es daher zu unerwartetem Verhalten kommen. Mit Tools wie cxx lassen sich diese Probleme jedoch wirksam abfangen.

Abgerundet wird der Artikel durch weitere Herausforderungen und ein Fazit.

## Performance

Die Einführung einer FFI-Schnittstelle führt im Idealfall nur zu geringen Leistungseinbussen.
Auch reine C++-Programme kommunizieren intern über binäre Schnittstellen zwischen Objektdateien, die der Linker zu einem gemeinsamen Programm zusammenführt.
Würde eine solche binäre Schnittstelle von der jeweils anderen Sprache perfekt nachgebildet, liessen sich Leistungseinbussen sogar vollständig vermeiden.
In der Praxis ist dieses Szenario jedoch kaum zu erreichen.
Zum einen sind Optimierungen des Compilers wie Inlining oder Dead-Code-Elimination über die Sprachgrenze hinweg nicht möglich, was die Performance einschränkt.
Dennoch fallen die Einbussen in der Regel deutlich kleiner aus als bei alternativen Integrationslösungen, etwa eine Interprozesskommunikation via Message-Passing.
Zum anderen müssen Objekte, die über die Sprachgrenze hinweg verwendet werden, bitgenau übereinstimmen.
Das ist fehleranfällig und erfordert bei jeder Anpassung des binären Speicherlayouts eine sorgfältige Anpassung auf beiden Seiten.
Damit wächst das Risiko, dass selbst kleine Änderungen zu undefiniertem Verhalten oder subtilen Fehlern führen, welche erst spät in der Release-Pipeline entdeckt werden.
Eine verbreitete Alternative ist die Verwendung opaker Datentypen, wie im nächsten Kapitel beschrieben.
Diese erleichtern die Handhabung mit komplexen Objekten, sie bringen jedoch zusätzliche Leistungseinbussen mit sich.

## Opake Datentypen

Ein zentrales Konzept zum Verständnis der Interoperabilitätsgrenzen sind opake Datentypen.
Zur Veranschaulichung kann man sich eine Analogie zu einem Verkaufsstand für Eismaschinen vorstellen (siehe Abbildung unten).
Der Verkäufer und der Kunde repräsentieren die beiden Programmiersprachen, während der Verkaufsstand der FFI-Schnittstelle entspricht.
Der Verkäufer übergibt dem Kunden das Produkt ausschliesslich über diesen Stand - genauso wie eine Sprache ein Datenobjekt über die FFI-Schnittstelle an die andere übergibt.
Die Produktübergabe (also der Datenaustausch) kann auf zwei Arten erfolgen:

1. **Stack-Allokation** -
Die Eismaschine ist modular aufgebaut und sofort verfügbar, jedoch kompliziert zu bedienen und anfällig für Störungen, wenn sie nicht korrekt gehandhabt wird.
Dies entspricht Daten, die auf dem Stack liegen.
Sie lassen sich sehr effizient übertragen, müssen jedoch auf Binärebene in beiden Sprachen exakt gleich dargestellt werden.
Zudem entsteht bei komplexen Datenstrukturen zusätzlicher Aufwand, weil sie zuerst auf einfache, C‑kompatible Grundtypen abgebildet und danach in der anderen Sprache wieder rekonstruiert werden müssen.

2. **Heap-Allokation** -
Die Eismaschine ist erst später lieferbar und kann nur mithilfe des Verkäufers bedient werden (z.B. über "operate()", "check()", "clean()", siehe Abbildung), dafür ist sie aber wenig wartungsintensiv.
Dies entspricht Daten, die auf dem Heap liegen.
Die Heap-Allokation verursacht zwar spürbare Verzögerungen, ermöglicht aber, den Datentyp opak zu halten.
Dies bedeutet, dass das Speicherlayout der Daten verborgen bleibt; der Empfänger erhält lediglich einen Pointer und interagiert ausschliesslich über ein klar definiertes Funktionen-Interface.

Die Verwendung von opaken Datentypen reduziert Kopplung und Fehlerrisiko.
Änderungen an der internen Datenstruktur erfordern keine Anpassungen auf der Gegenseite, solange die FFI-Schnittstelle stabil bleibt.

<img src="images/opaque_types.png" alt="opaque_type" width="800"/>

Ein weiterer Anwendungsfall opaker Datentypen ist die Einbindung von Datenstrukturen, deren Binärrepräsentation instabil oder vom Compiler abhängig ist.
Dazu gehören etwa Dateien- oder Socket-Handler, Closures oder andere komplexe Laufzeitobjekte.
Solche Typen können sich je nach Compiler unterscheiden und sind daher nicht zuverlässig direkt über die FFI abbildbar.
Opake Datentypen verbergen die Compiler-abhängige interne Struktur dieser Objekte, sodass die Gegenseite nur über eine sichere API mit ihnen interagiert.

Zusammengefasst erhöhen opake Datentypen die Benutzerfreundlichkeit und Stabilität der Schnittstelle, gehen jedoch zu Lasten der Performance.

## Move-Verhalten

Die bisher besprochenen Grenzen der Interoperabilität gelten allgemein für FFI-Schnittstellen zwischen zwei Sprachen.
Es gibt jedoch auch eine Inkompatibilität spezifisch zwischen C++ und Rust:
Den Umgang mit selbstreferenziellen Datentypen.
Ein selbstreferenzieller Datentyp ist ein Objekt, das intern einen Zeiger (Pointer) auf sich selbst enthält.
In C++ kommen solche Konstrukte häufig vor - etwa bei Iteratoren und Listen, aber auch bei Strings oder Vektoren können sie je nach Compiler nicht ausgeschlossen werden.
C++ kann diese Objekte sicher im Speicher verschieben, weil der Move-Konstruktor dafür sorgt, dass der interne Zeiger nach dem Verschieben aktualisiert wird.
Rust hingegen erlaubt selbstreferenzielle Datentypen nur in streng kontrollierten Situationen.
Objekte, die nicht im Speicher bewegt werden dürfen, werden als "gepinnt" markiert.
Beispiele dafür sind Futures oder bestimmte Generatoren.

Über die FFI-Schnittstelle können jedoch selbstreferenzielle Objekte aus C++ ohne Pin-Markierung nach Rust gelangen - und genau hier entsteht ein gefährliches Missverständnis.
Rust geht davon aus, dass alle nicht gepinnten Objekte frei und bitweise verschiebbar sind.
Wird ein selbstreferenzielles Objekt jedoch bitweise verschoben, bleibt der interne Zeiger unverändert und zeigt nach der Verschiebung nicht mehr auf das verschobene Objekt, sondern auf die alte Speicheradresse.
Das Ergebnis ist undefiniertes Verhalten, das sich schwer debuggen lässt und potenziell sicherheitskritische Fehler verursacht.

Zusammengefasst können durch die FFI-Schnittstelle unvorhergesehene Fehlerfälle auftreten. Diese können aber durch Tools wie cxx automatisch erkannt werden.

## Weitere Herausforderungen

* **Debugging** - Das Debuggen über Sprachgrenzen hinweg erfordert besondere Aufmerksamkeit. Unterschiedliche Calling Conventions, optimierende Compiler und eingeschränkt interpretierbare Stacktraces können die Analyse erschweren. Mit passenden Tools und klaren Debugging-Richtlinien lässt sich diese Herausforderung jedoch effizient meistern.
* **Async** - Asynchrones Programmieren über FFI erfordert eine sorgfältige Planung, da Rusts async-/await-Mechanismus und C++-Futures oder Coroutinen nicht direkt kompatibel sind. Durch Oneshot-Kanäle, Callbacks oder gut strukturierte Event-Loops lässt sich die Interoperabilität zuverlässig herstellen.
* **Sanitizer** - Laufzeit-Analysewerkzeuge wie AddressSanitizer, ThreadSanitizer oder UndefinedBehaviorSanitizer liefern über Sprachgrenzen hinweg nicht immer vollständige Informationen. Mit angepassten Test- und Analyseprozessen können mögliche Probleme frühzeitig erkannt werden.
* **Threading-Modelle** - Rust kann keine Thread-Sicherheitsgarantien für über FFI ausgetauschte Objekte übernehmen. Eine klare Dokumentation der Schnittstelle sowie definierte Regeln für den Zugriff auf gemeinsame Objekte sorgen jedoch für sichere Multi-Threading-Szenarien.

Wer diese Herausforderungen frühzeitig berücksichtigt und geeignete Strategien entwickelt, kann die Vorteile beider Sprachen optimal nutzen und die Interoperabilität stabil und effizient gestalten.

## Fazit

Die Interoperabilität zwischen C++ und Rust eröffnet in vielen Projekten die Möglichkeit, bestehende Systeme schrittweise zu modernisieren oder die Stärken beider Sprachen gezielt zu kombinieren.
Damit dieser Ansatz maximalen Nutzen bringt, ist es jedoch wichtig, die technischen Rahmenbedingungen gut zu verstehen.
So verursacht eine FFI-Schnittstelle zwar gewisse Leistungseinbußen, da Daten über die Sprachgrenze hinweg übertragen werden müssen, diese fallen jedoch deutlich kleiner aus als bei alternativen Integrationsansätzen.
Zudem erfordert die Schnittstelle sorgfältige Planung, da bestimmte Datentypen nicht direkt zwischen den Sprachen ausgetauscht werden können.
Eine klare, logische Trennung der Komponenten und einfach gehaltene Schnittstellen sind daher zentrale Erfolgsfaktoren: Sie reduzieren den Entwicklungsaufwand, minimieren Fehlerquellen und erhöhen die Stabilität der Gesamtlösung.
Moderne Werkzeuge wie cxx unterstützen diesen Prozess zusätzlich, indem viele potenzielle Fehler bereits zur Compile-Zeit erkannt werden.
Wer diese Aspekte im Blick behält, schafft eine robuste Grundlage für eine erfolgreiche und nachhaltige Integration beider Sprachen.
