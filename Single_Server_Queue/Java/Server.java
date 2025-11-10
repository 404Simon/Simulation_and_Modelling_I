import java.util.LinkedList;
import java.util.Queue;
import java.util.Random;

public class Server implements SimEntity {
    private final double mu;
    private final Queue<Double> q = new LinkedList<>();
    private boolean busy = false;
    private final Random rand = new Random();
    private final Statistics stats;

    public Server(double mu, Statistics stats) {
        this.mu = mu;
        this.stats = stats;
    }

    public void receiveCustomer(SimulationEngine engine) {
        q.add(engine.now());
        stats.recordQueueChange(engine.now(), q.size());
        if (!busy) startService(engine);
    }

    private void startService(SimulationEngine engine) {
        if (q.isEmpty()) return;
        double arrivalTime = q.remove();
        stats.recordQueueChange(engine.now(), q.size());

        busy = true;
        double waitTime = engine.now() - arrivalTime;
        stats.recordServiceStart(waitTime);

        double serviceTime = exp(mu);
        stats.recordServiceEnd(serviceTime);

        engine.schedule(new Event(engine.now() + serviceTime, this, "DEPARTURE"));
    }

    @Override
    public void handleEvent(Event event, SimulationEngine engine) {
        if ("DEPARTURE".equals(event.type)) {
            busy = false;
            if (!q.isEmpty()) startService(engine);
        }
    }

    private double exp(double rate) {
        return -Math.log(1 - rand.nextDouble()) / rate;
    }
}
