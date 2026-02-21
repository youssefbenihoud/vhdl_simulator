# VHDL-Simulator in Rust
### Projekt: Analyse von Variablen vs. Signalen und Kontrolllogik

Dieses Projekt demonstriert die Simulation von Hardware-Logik (VHDL) unter Verwendung der Programmiersprache Rust. Im Fokus steht der funktionale Unterschied zwischen **Variablen** und **Signalen** sowie die Implementierung von **Enable**-Signalen und **asynchronen Resets**.

---

## 1. Architekturidee
Die Architektur simuliert eine einfache Zähleinheit innerhalb eines FPGAs. 

* **Variable (`variable_counter`):** Repräsentiert einen rein sequentiellen Rechenschritt innerhalb eines Prozesses. Der Wert ist sofort nach der Zuweisung für nachfolgende Operationen verfügbar.
* **Signal (`signal_counter`):** Repräsentiert ein physikalisches Register (Flip-Flop). Der Wert wird während des Prozesses nur "geplant" und erst mit der nächsten Taktflanke (`clk`) tatsächlich übernommen.
* **Steuerung:** Die Einheit reagiert auf ein `enable`-Signal und verfügt über einen `reset_n` (low-active), der Vorrang vor allen anderen Operationen hat.



---

## 2. Implementierung der Logik
Die Logik wurde in Rust mittels einer `struct` und einer `impl`-Methode umgesetzt, um den Zustand der Hardware-Zellen sicher zu kapseln.

### Hauptkomponenten des Codes:
* **Asynchroner Reset:** Realisiert durch eine `if !reset_n`-Abfrage zu Beginn der Update-Routine. Dies simuliert die höchste Priorität (Priority-Encoding) in der Hardware.
* **Enable-Logik:** Die Zähler werden nur inkrementiert, wenn das `enable`-Signal auf `kurz` steht.
* **Delta-Cycle Simulation:** Um das Signalverhalten von VHDL (Updates erst nach dem Prozess) nachzubilden, nutzt Rust einen `signal_buffer`.

---

## 3. Hardware-Synthese (Konzeptuell)
Obwohl dieses Projekt in Software läuft, bildet der Code Strukturen ab, die direkt in Gatterlogik übersetzt werden könnten:
* Die **Variable** entspricht einer rein **kombinatorischen Logik** (Addierer-Netzwerk).
* Das **Signal** entspricht einem **sequentiellen Element** (D-Flip-Flop mit Enable und Clear).

---

## 4. Simulation
Die Simulation durchläuft mehrere Taktzyklen, um das Zeitverhalten und die Zustandsübergänge zu visualisieren.

### Simulationsablauf:
1.  **Initialisierung:** Alle Zähler starten bei 0.
2.  **Taktphase (Update):** Berechnung der neuen Werte. Hier wird sichtbar, dass die Variable bereits den neuen Wert hat, während das Signal noch den alten Stand anzeigt.
3.  **Taktflanke (Apply):** Das Signal übernimmt den Wert aus dem Puffer in das Register.
4.  **Reset-Test:** Demonstration der sofortigen Löschung aller Speicherinhalte bei `reset_n = false`.

---

## 5. Auswertung der Ergebnisse
Die Beobachtungen aus der Simulation bestätigen das klassische VHDL-Modell:

| Komponente | Zeitpunkt der Änderung | Hardware-Äquivalent |
| :--- | :--- | :--- |
| **Variable** | Sofort (während des Prozesses) | Zwischenergebnis im Kombinatoriknetz |
| **Signal** | Verzögert (nach der Taktflanke) | Ausgang eines Registers (Flip-Flop) |

**Fazit:** Die Variable "eilt" dem Signal innerhalb eines Berechnungsschrittes immer um einen Wert voraus, da sie keinen Taktpuffer benötigt.

---

## 6. Installation und Ausführung

Da Rust bereits installiert ist, kann das Projekt wie folgt gestartet werden:

1.  **Projektordner betreten:**
    ```bash
    cd vhdl_simulator
    ```
2.  **Kompilieren und Starten:**
    ```bash
    cargo run
    ```
3.  **Dokumentation generieren:**
    Um die Code-Dokumentation als Webseite anzuzeigen:
    ```bash
    cargo doc --open
    ```

---
