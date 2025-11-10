public class Statistics {
    private double totalWaitTime = 0.0;
    private int servedCustomers = 0;
    private double totalBusyTime = 0.0;
    private double lastEventTime = 0.0;
    private double areaUnderQ = 0.0;
    private int lastQueueLength = 0;

    public void recordQueueChange(double time, int queueLength) {
        double timeDelta = time - lastEventTime;
        areaUnderQ += lastQueueLength * timeDelta;
        lastEventTime = time;
        lastQueueLength = queueLength;
    }

    public void recordServiceStart(double waitTime) {
        totalWaitTime += waitTime;
    }

    public void recordServiceEnd(double serviceDuration) {
        servedCustomers++;
        totalBusyTime += serviceDuration;
    }

    public double getAverageWaitTime() {
        return servedCustomers == 0 ? 0 : totalWaitTime / servedCustomers;
    }

    public double getAverageQueueLength(double totalTime) {
        return totalTime == 0 ? 0 : areaUnderQ / totalTime;
    }

    public double getUtilization(double totalTime) {
        return totalTime == 0 ? 0 : totalBusyTime / totalTime;
    }

    public int getServedCustomers() {
        return servedCustomers;
    }
}
