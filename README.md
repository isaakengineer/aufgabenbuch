# Aufgabenbuch

> Die "To-Do" Liste für die Sophisten!

Aufgabenbuch ist eine auf SQLite basierte Anwendung für (Desktop) Linux zum Verwaltung einer oder mehrere Listen (Dateien) persönlicher Aufgaben.

## Download

This application is available for download and installation on Linux desktop environment with for following distribution families:

- Debian (Ubuntu, etc)
	- ARM 64
	- AMD 64
- RPM (Redhat, Fedora, etc)
	- ARM 64
	- AMD 64

[https://console.schloosser.net/browser/aufgabenbuch](https://console.schloosser.net/browser/aufgabenbuch)

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

## Status

### Maturity

- quality of source code: MVP 2
- quality of documentation: NULL
- stage of product growth: mittle (mehrartiges Textbearbeitung)

### Dedication

_to my soulamte_. **Valentine Gift 2025**.

### Links

- Webseite: [https://aufgabenbuch.techne.schloosser.com](https://aufgabenbuch.techne.schloosser.com)
- Hauptquelle des Kodes: [https://git.schloosser.net/aufgabenbuch/aufgabenbuch](https://git.schloosser.net/aufgabenbuch/aufgabenbuch)
- Spiegel der Hauptquelle (Codeberg): [https://codeberg.org/techne/aufgabenbuch](https://codeberg.org/techne/aufgabenbuch)
- Dokumentationen (Quellen)
    - für NutzerInnen: [https://git.schloosser.net/aufgabenbuch/dok-gebrauch](https://git.schloosser.net/aufgabenbuch/dok-gebrauch)
    - für Entwickelung: [https://git.schloosser.net/aufgabenbuch/dok-entwicklung](https://git.schloosser.net/aufgabenbuch/dok-entwicklung)
    - für Organisation: [https://git.schloosser.net/aufgabenbuch/dok-org](https://git.schloosser.net/aufgabenbuch/dok-org)

## Technologies

- Tauri 2
- SvelteKit 1, Svelte 4
- Vite

### License

Copyright (C) 2025 Hossein Rezaei (penname: Isaak Engineer)

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, version 3.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
