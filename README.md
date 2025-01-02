# Aufgabenbuch

> Die "To-Do" Liste für die Sophisten!

Aufgabenbuch ist eine auf SQLite basierte Anwendung für (Desktop) Linux zum Verwaltung einer oder mehrere Listen (Dateien) persönlicher Aufgaben. 

## Herunterladen

[https://console.schloosser.net/browser/aufgabenbuch](https://console.schloosser.net/browser/aufgabenbuch)

## Links

- Webseite: [https://aufgabenbuch.techne.schloosser.com](https://aufgabenbuch.techne.schloosser.com)
- Hauptquelle des Kodes: [https://git.schloosser.net/aufgabenbuch/aufgabenbuch](https://git.schloosser.net/aufgabenbuch/aufgabenbuch)
- Spiegel der Hauptquelle (Codeberg): [https://codeberg.org/techne/aufgabenbuch](https://codeberg.org/techne/aufgabenbuch)
- Dokumentationen (Quellen)
    - für NutzerInnen: [https://git.schloosser.net/aufgabenbuch/dok-gebrauch](https://git.schloosser.net/aufgabenbuch/dok-gebrauch)
    - für Entwickelung: [https://git.schloosser.net/aufgabenbuch/dok-entwicklung](https://git.schloosser.net/aufgabenbuch/dok-entwicklung)
    - für Organisation: [https://git.schloosser.net/aufgabenbuch/dok-org](https://git.schloosser.net/aufgabenbuch/dok-org)

## Entwickelung

### Durchführen auf Linux

#### für Gebrauch

```
deno install
deno task tauri build
```

#### für Bearbeitung

```
deno install
WEBKIT_DISABLE_COMPOSITING_MODE=1 deno task tauri dev
```

`WEBKIT_xyz` da es, entweder ein Bug, oder etwas anderes mit GTK und Tauri nicht gut zusammenspielt!