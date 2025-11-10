public class SimulationMain {
  public static void main(String[] args) {
    SimulationEngine engine = new SimulationEngine();
    Statistics stats = new Statistics();
    Server server = new Server(1.0 / 9.0, stats);
    Client client = new Client(1.0 / 10.0, server);

    engine.schedule(new Event(0.0, client, "GENERATE"));

    long wallTimeStart = System.nanoTime();
    while (engine.hasNextEvent() && engine.peekNextTime() < 10_000_000.0) {
      engine.runStep();
    }
    long wallTimeEnd = System.nanoTime();

    double totalTime = engine.now();
    double wallTimeSeconds = (wallTimeEnd - wallTimeStart) / 1_000_000_000.0;
    long eventsProcessed = engine.getEventCount();
    double eventsPerSecond = eventsProcessed / wallTimeSeconds;

    System.out.println("=== Simulationsergebnisse ===");
    System.out.printf("Gesamte Zeit: %.2f\n", totalTime);
    System.out.printf("Bediente Kunden: %d\n", stats.getServedCustomers());
    System.out.printf("Mittlere Wartezeit: %.4f\n", stats.getAverageWaitTime());
    System.out.printf("Mittlere Queue-Länge: %.4f\n", stats.getAverageQueueLength(totalTime));
    System.out.printf("Serverauslastung: %.4f\n", stats.getUtilization(totalTime));
    System.out.println("\n=== Performance Metrics ===");
    System.out.printf("Events verarbeitet: %d\n", eventsProcessed);
    System.out.printf("Wall-Zeit: %.3f Sekunden\n", wallTimeSeconds);
    System.out.printf("Events/Sekunde: %.2f\n", eventsPerSecond);
  }
}
