import java.util.Random;

public class Client implements SimEntity {
    private final double lambda;
    private final Server server;
    private final Random rand = new Random();

    public Client(double lambda, Server server) {
        this.lambda = lambda;
        this.server = server;
    }

    @Override
    public void handleEvent(Event event, SimulationEngine engine) {
        switch (event.type) {
            case "GENERATE":
                server.receiveCustomer(engine);
                // plane nächste Generierung
                double nextTime = engine.now() + exp(lambda);
                engine.schedule(new Event(nextTime, this, "GENERATE"));
                break;
        }
    }

    private double exp(double rate) {
        return -Math.log(1 - rand.nextDouble()) / rate;
    }
}
