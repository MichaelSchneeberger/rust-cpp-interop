# Von C++ nach Rust - Strategien für eine schrittweise Migration

Die Migration von C++ nach Rust ist - wie in jeder andere Sprache auch - mit vielen Unbekannten verbunden: 
Kann der C++-Code mit einem Übersetzer-Tool automatisch korrekt umgeschrieben werden? 
Werden im C++-Code Rust-fremde Instruktionen gebraucht, die nicht direkt übersetzt werden können?
Oder verhalten sich ähnliche Objekte in der jeweiligen Sprache während der Ausführung wirklich gleich?
Diese Unbekannten machen eine realistische Zeitabschätzung für die Migration in einem Zug schwierig.
Eine Alternative dazu stellt die schrittweise Migration dar, bei welcher nur ein Teil der Projektes in Rust umgeschrieben wird und mit dem legacy Code durch C-Schnittstellen kommuniziert.
Besteht das Programm aus gut abgrenzbaren Komponenten, bringt dieser Ansatz klare Vorteile.
Zum Beispiel können komplexe, aber gut funktionierde Komponenten erst später migriert werden, wenn genügend Zeit und Resourcen zur Verfügung stehen.
Zudem kann durch ein scrhittweises Vorgehen das Risiko für ein Misslingen einer Migration gesenkt und die Planbarkeit nach erfolgreicher Teilmigration deutlich gesteigert werden.
Trotzdem sollte dieser Ansatz mit Bedacht gewählt werden.
Zum einen limitieren die C-Schnittstellen den Datenaustausch zwischen den Sprachen, welche mit aufwändigen Neustrukturierung der Daten einher geht.
Zum anderen reduzieren sie die Dynamik der Softwareentwicklung, weil die C-Schnittstelle als Sprachenübergreifende Vertrag der beiden Sprachen sich nur mit grösseren Aufwand anpassen lässt.
Dieser Beitrag soll die Vor- und Nachteile einer schrittweisen Migration aufzeigen.

Im ersten Teil der Beitrags gibt eine Einführung in das Problem der Interoperabilität von C++ und Rust.
Im zweiten Teil wird die `cxx` vorgestellt, welche die Erstellung der C-Schnittstelle nicht nur automatisiert sondern auch dessen korrekte Anwendung überprüft.

## Interoperabilität von C++ und Rust


## Automatisierung durch `cxx`


## Schlusswort


