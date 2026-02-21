/// ==========================================================================
/// PROJEKT: VHDL-Simulator in Rust
/// AUFGABE: Simulation von Reset, Enable und Signal vs. Variable
/// ==========================================================================

/// Architektur (Schritt 1):
/// Wir definieren eine Struktur, die einen FPGA-Prozess nachbildet.
struct CounterUnit {
    variable_counter: u8, // VHDL Variable: Wird sofort aktualisiert
    signal_counter: u8,   // VHDL Signal: Wird erst am Ende des Taktes übernommen
    signal_buffer: u8,    // Hilfsspeicher: Simuliert die Hardware-Verzögerung
}

impl CounterUnit {
    /// Konstruktor: Initialisierung der Hardware-Komponenten
    fn new() -> Self {
        Self {
            variable_counter: 0,
            signal_counter: 0,
            signal_buffer: 0,
        }
    }

    /// Implementierung der Logik (Schritt 2 & 4):
    /// Diese Methode simuliert die Reaktion auf Eingänge (reset_n, enable).
    fn update(&mut self, reset_n: bool, enable: bool) {
        // 1. Asynchroner Reset (Höchste Priorität laut Aufgabenstellung)
        if !reset_n {
            self.variable_counter = 0;
            self.signal_counter = 0;
            self.signal_buffer = 0;
            println!(">>> RESET: Alle Zähler wurden auf 0 gesetzt.");
        } 
        // 2. Synchroner Teil: Nur aktiv, wenn Enable = true
        else if enable {
            // VARIABLE: In VHDL erfolgt die Zuweisung sofort.
            // Wir erhöhen den Wert direkt im aktuellen Schritt.
            self.variable_counter += 1;
            
            // SIGNAL: In VHDL wird der Wert erst am Ende des Prozesses gültig.
            // Wir berechnen den Wert nur vor und "parken" ihn im Buffer.
            self.signal_buffer = self.signal_counter + 1;

            println!("Innerhalb des Taktes:");
            println!("   Variable-Wert: {}", self.variable_counter);
            println!("   Signal-Wert (noch alt!): {}", self.signal_counter);
        }
    }

    /// Taktende (Schritt 4):
    /// Simuliert die steigende Taktflanke, bei der Signale ihre Werte übernehmen.
    fn apply_signal_update(&mut self) {
        self.signal_counter = self.signal_buffer;
    }
}

/// Simulation (Schritt 4 & 5):
/// Hauptprogramm zur Demonstration des Verhaltens.
fn main() {
    let mut mein_chip = CounterUnit::new();

    println!("========================================");
    println!("START DER VHDL-SIMULATION IN RUST");
    println!("========================================");

    // Wir simulieren 3 Taktzyklen
    for takt in 1..=3 {
        println!("\n--- TAKTZYKLUS {} ---", takt);
        
        // Simuliere: Reset ist inaktiv (true), Enable ist aktiv (true)
        mein_chip.update(true, true);
        
        // Jetzt beenden wir den Taktzyklus
        mein_chip.apply_signal_update();
        
        println!("Nach dem Takt-Update:");
        println!("   Signal-Wert (jetzt aktualisiert): {}", mein_chip.signal_counter);
    }

    // Zum Abschluss testen wir den asynchronen Reset
    println!("\n--- TEST: ASYNCHRONER RESET ---");
    mein_chip.update(false, true);
    println!("Zustand nach Reset: Variable={}, Signal={}", 
             mein_chip.variable_counter, mein_chip.signal_counter);

    println!("\n========================================");
    println!("SIMULATION ERFOLGREICH BEENDET");
    println!("========================================");
}
