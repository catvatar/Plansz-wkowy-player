Zamysł jest taki, małe urządzenie podłączone do wifi. Urządzenie komunikuje się z API Spotify żeby kontrolować muzykę w jak najmniej inwazyjny sposób _immersja_. Wybierasz 3 playlisty ze spoti i za pomocą fizycznych przycisków przełączasz się między nimi tak jak tego wymaga rozgrywka.
# Wymagania

## Koniecznie
- min 3 programowalne przyciski
> np. ambient, walka, story
- 2 przyciski pauza i skip
- WiFi
- ma się mieścić pod stołem
- żywotności baterii min 4h 
# Problemy
## Jak połączyć się z nowym wifi bez wyświetlacza
Może jakaś rutyna, jeśli nie udało się połączyć z wifi to hostuje własne na której jest stroną gdzie wybierasz wifi
## Pamięć trwała
Co musi posiadać:
- wifi credencials
- spotify credencials
- playlisty przypisane do przycisków
# Pomysły
- może jakiś internal skip counter
- urządzenie może hostować małą stronę na której przypisujesz do każdego przycisku jaką playlistę ma odpalać
- przy przełączaniu utworów za pomocą przycisków zrobić płynne przejście fade out fade in
- przy przełączaniu w walce skipuj kilka sekund początku żeby cały czas było intensywne tempo 
- metadata na piosenkach że chcemy tylko ich fragmenty 
- **boss fight music**
# Materiały
- [Embedded Rust](https://docs.esp-rs.org/book/introduction.html)
- myśle żeby zrobić to na esp32
- klawiatura Kuby w mojej szafie