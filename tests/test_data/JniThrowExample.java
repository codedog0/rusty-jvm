package samples.jni.exceptions;

class JniThrowExample {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native void throwViaThrow(RuntimeException e);

    public static void main(String[] args) {
        System.out.println("=== Throw ===");
        try {
            throwViaThrow(new RuntimeException("thrown via Throw"));
        } catch (RuntimeException e) {
            System.out.println("Caught: " + e.getMessage());
        }
    }
}
