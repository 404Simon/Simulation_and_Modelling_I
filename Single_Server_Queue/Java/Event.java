public class Event implements Comparable<Event> {
    public double time;
    public SimEntity target;
    public String type;

    public Event(double time, SimEntity target, String type) {
        this.time = time;
        this.target = target;
        this.type = type;
    }

    @Override
    public int compareTo(Event o) {
        return Double.compare(this.time, o.time);
    }
}
