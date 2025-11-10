import java.util.PriorityQueue;

public class SimulationEngine {
  private PriorityQueue<Event> queue = new PriorityQueue<>();
  private double now = 0.0;

  public void schedule(Event e) {
    queue.add(e);
  }

  public double now() {
    return now;
  }

  public void run() {
    while (!queue.isEmpty()) {
      Event e = queue.poll();
      now = e.time;
      e.target.handleEvent(e, this);
    }
  }

  public boolean hasNextEvent() {
    return !queue.isEmpty();
  }

  public double peekNextTime() {
    return queue.isEmpty() ? Double.POSITIVE_INFINITY : queue.peek().time;
  }

  public void runStep() {
    if (queue.isEmpty())
      return;
    Event e = queue.poll();
    now = e.time;
    e.target.handleEvent(e, this);
  }
}
