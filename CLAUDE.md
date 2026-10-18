# Ekran przejścia — aplikacje biura Filar

Jeden ekran na Macu taty: kafelki z aplikacjami biura, klik = otwiera aplikację.
Plan i postęp: `PLAN.md`.

## Dla kogo

- Tata, jeden komputer (Mac). Nie techniczny — ma być oczywiste bez tłumaczenia.
- Bez logowania: aplikacje online mają własne logowanie.

## Zakres

- Aplikacja Tauri 2 (macOS). Wygląd i logika: `ui/index.html` (jeden plik, bez frameworka). Rust: tylko komenda `otworz` w `src-tauri/src/main.rs`.
- `otworz(cel)`: adres `https://…` → domyślna przeglądarka taty (`open`); id aplikacji lokalnej → `open ~/…` wg listy `LOKALNE` w `main.rs`. Inne cele odrzucane.
- Statystyki: każde udane otwarcie zapisywane w `localStorage` okna (`otwarcia`: id + czas). Liczymy tylko kliknięcia z tego ekranu, nie czas w aplikacjach.
- Strona działa też w zwykłej przeglądarce jako aplikacja webowa (PWA na Vercelu) — patrz „Wersja webowa”. Linki online działają; aplikacji lokalnych strona nie uruchomi (na Macu kafelek podpowiada ikonę z pulpitu, na telefonie/tablecie/Windowsie jest ukryty).
- Programu dokumentów NIE wbudowujemy w ekran (Python, osobny projekt, osobne aktualizacje) — ekran tylko uruchamia jego ikonę z pulpitu.
- Nie dokładać funkcji spoza tej listy (wyszukiwarka, statusy, ustawienia, konta itp.).

## Aplikacje

| Nazwa | Opis na kafelku | Typ | Adres / ścieżka |
|---|---|---|---|
| Fakturomat | Faktura na papierze firmowym | online | https://faktury-filar.web.app |
| Finanse | Finanse osobiste i firmowe | online | https://finanse-biuro-filar.vercel.app |
| Samochody | Koszty utrzymania aut | online | https://samochody-ten.vercel.app |
| Uzupełnianie dokumentów | Wypełnia dokumenty z pliku | lokalna (Python/Flask, osobny projekt `~/Desktop/WYPELNIANIE-DOKUMENTOW`) | `~/Desktop/Dokumenty FILAR.app` (tworzy ją `instaluj_mac.sh` tamtego projektu) |
| Czas pracy | Godziny pracy i rozliczenia | desktop, **w budowie** → kafelek wyszarzony „Wkrótce” | — |

Kolejność kafelków: od najczęściej używanej (do potwierdzenia z tatą).
Ikony: folder `ui/ikony/`, nazwy jak aplikacja, SVG lub PNG ≥512 px, kwadrat bez przezroczystego marginesu (inaczej przyciąć). Logo biura: `ui/ikony/biuro.svg`, w trybie ciemnym `biuro-ciemne.svg` (napis jasny, żółty bez zmian; filtr CSS psuł żółty). Czas pracy na razie litera.

## Układ (wzór: App Store → sekcja „Great New Productivity Apps”)

- Pasek boczny jak w App Store: pływający panel (odsunięty 8px od krawędzi, zaokrąglenie 18px, cień), jasnoszary gradient jak pasek App Store, logo biura, pozycje „Aplikacje” i „Statystyki”. Wybrana pozycja: żółte tło `#f7c303` z logo, ciemny tekst.
- Główna część: delikatna żółta poświata w lewym górnym rogu, duży tytuł wyrównany do lewej (jak „Work” w App Store).
- Statystyki: dla Kacpra i do case study. Liczymy tylko kliknięcia z tego ekranu (która aplikacja, kiedy), czasu w aplikacjach nie. Na razie pusty ekran z opisem.
- Żółty tylko jako tło/poświata, nigdy jako tekst na jasnym tle (nieczytelny).
- Lista wierszy w 2 kolumnach. Wiersz: ikona po lewej, nazwa + jedno zdanie opisu, po prawej pigułka „Otwórz”.
- Cienka linia między wierszami, zaczyna się od tekstu (nie pod ikoną).
- Klikalny cały wiersz, nie tylko pigułka. Hover: lekkie tło wiersza.
- „Czas pracy”: wyszarzony, pigułka „Wkrótce”, nieklikalny.
- Bez znaczków online/lokalnie — użytkownika to nie obchodzi.
- Wzory: zrzuty App Store i Ustawień systemowych z Maca Kacpra (październik 2026).

## Design: Apple (macOS HIG)

Źródło: https://developer.apple.com/design/human-interface-guidelines (designsystems.surf to tylko spis linków do HIG — płatnych „Foundation” nie kupować).

- Font: systemowy (`-apple-system, BlinkMacSystemFont, sans-serif`) = SF Pro na Macu, nic nie ładować.
- Typografia: tytuł 34px bold, nazwa aplikacji 15px semibold, opis 13px regular w kolorze secondary.
- Kolory jasne: tło `#FFFFFF`, tekst `rgba(0,0,0,.85)`, secondary `rgba(0,0,0,.5)`, separator `rgba(0,0,0,.1)`, pigułka tło `rgba(0,0,0,.05)`, hover wiersza `rgba(0,0,0,.03)`.
- Kolory ciemne (wg systemu): tło `#1E1E1E`, tekst `rgba(255,255,255,.85)`, secondary `rgba(255,255,255,.55)`, separator `rgba(255,255,255,.1)`, pigułka tło `rgba(255,255,255,.1)`.
- Akcent (tekst pigułki): `AccentColor` (kolor akcentu z systemu taty), zapas `#007AFF`. Sprawdzić w Tauri/WebKit, czy łapie systemowy.
- Pigułka: wysokość ~28px, pełne zaokrąglenie, tekst 13px semibold w kolorze akcentu.
- Ikona aplikacji: 56–64px, zaokrąglenie ok. 22,5% boku (kształt ikon macOS).
- Odstępy: siatka co 8px, marginesy okna 24–32px.
- Ruch: krótko (150–200 ms, ease-out), bez fajerwerków; szanować `prefers-reduced-motion`.
- Wartości wyżej to standard macOS — korygować do wzorów, które wklei Kacper.

## Skille

- `impeccable` — główny do wyglądu.
- `emil-design-eng` (emilkowalski/skills) — hover, przejścia, ruch.
- Nic więcej o designie (sprzeczne zasady).

## Budowanie i instalacja

- Podgląd w oknie: `npm run dev`. Gotowa aplikacja: `npm run build` → `src-tauri/target/release/bundle/macos/Biuro Filar.app`.
- Ikona aplikacji: źródło `src-tauri/ikona.png` — MUSI być pełnym kwadratem 1024 bez przezroczystości i bez własnych zaokrągleń (macOS 26 sam przycina do kształtu; ikona z przezroczystymi rogami ląduje w szarej ramce) (znak F biura), wygenerowanie: `npm run ikona` (potem usunąć foldery android/ios i Square*).
- U taty: skopiować `Biuro Filar.app` do Aplikacji. Aplikacja nie jest podpisana — przy pierwszym uruchomieniu prawy klik → Otwórz (albo Ustawienia → Prywatność i ochrona → „Otwórz mimo to”).

## Wersja webowa (Vercel, bez instalowania narzędzi)

- Vercel: import repo z GitHuba, nic nie ustawiać — `vercel.json` podaje folder `ui` i pomija budowanie (bez tego Vercel odpaliłby `tauri build`).
- Pliki PWA w `ui/`: `manifest.webmanifest`, `sw.js` (najpierw sieć, z pamięci tylko offline), ikony `ikona-192/512.png`, `apple-touch-icon.png`, `favicon-32.png` — wygenerowane z `src-tauri/ikona.png`.
- Dodanie: Mac (Safari) → Plik → Dodaj do Docka; iPhone/iPad (Safari) → Udostępnij → Do ekranu początkowego; Windows/Android (Chrome/Edge) → ikona instalacji w pasku adresu / menu → Zainstaluj.
- Statystyki liczą się osobno na każdym urządzeniu (localStorage).
