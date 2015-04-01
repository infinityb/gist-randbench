import java.util.Random;

class RandBench {
	public static void main(String [] args) {
		long before = System.currentTimeMillis();

		Random rand = new Random();
		byte[] random = new byte[1000000];
		rand.nextBytes(random);

		long after = System.currentTimeMillis();
		System.out.print(String.format("took %sms", after - before));
	}
}
