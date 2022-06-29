package com.fubukidaze.vpn;

import android.app.PendingIntent;
import android.content.Intent;
import android.net.VpnService;
import android.os.Handler;
import android.os.Message;
import android.os.ParcelFileDescriptor;
import android.util.Log;
import android.widget.Toast;

import com.fubukidaze.R;

import org.json.JSONObject;

import java.io.File;
import java.io.FileDescriptor;
import java.io.IOException;
import java.net.InetSocketAddress;
import java.nio.channels.SocketChannel;


public class NativeVpnService extends VpnService implements Handler.Callback, Runnable {
    private static final String TAG = "NativeVpnService";

    private String lanIpAddr;
    private String localIp;
    private String serverIp;
    private String serverPort;
    private String localRoute;
    private String key;

    private byte[] mSharedSecret;
    private PendingIntent mConfigureIntent;

    private Handler mHandler;
    private Thread mThread;

    private ParcelFileDescriptor mInterface;
    private String mParameters;

    @Override
    public int onStartCommand(Intent intent, int flags, int startId) {
        Log.d(TAG, "onStartCommand");
        // The handler is only used to show messages.
        if (mHandler == null) {
            mHandler = new Handler(this);
        }

        // Stop the previous session by interrupting the thread.
        if (mThread != null) {
            return START_STICKY;
        }

        // Extract information from the intent.
        lanIpAddr = intent.getStringExtra("lanIpAddr");
        localIp = intent.getStringExtra("localIp");
        serverIp = intent.getStringExtra("serverIp");
        serverPort = intent.getStringExtra("serverPort");
        localRoute = intent.getStringExtra("localRoute");
        key = intent.getStringExtra("key");

        // Start a new session by creating a new thread.
        mThread = new Thread(this, "NativeVpnServiceThread");
        mThread.start();
        return START_STICKY;
    }

    @Override
    public void onDestroy() {
        Log.d(TAG, "onDestroy");
        if (mThread != null) {
            try {
                mInterface.close();
            } catch (IOException e) {
                e.printStackTrace();
            }
        }
    }

    static {
        System.loadLibrary("fubuki");
    }


    public interface JNICallback {
        void callback(String string);
    }

    public static native String TestJni();

    public static native void LaunchFubukiClient(int fileDescriptor, String dzcfg) throws Exception;

    public static native void DestoryFubukiClient();

    public static native void invokeCallbackViaJNI(JNICallback callback);

    @Override
    public boolean handleMessage(Message message) {
        if (message != null) {
            Toast.makeText(this, message.what, Toast.LENGTH_SHORT).show();
        }
        return true;
    }

    @Override
    public synchronized void run() {
        try {
            Log.i(TAG, "Starting");

            mHandler.sendEmptyMessage(R.string.connecting);
            if (start()) {
                mHandler.sendEmptyMessage(R.string.connected);
            }
        } catch (Exception e) {
            Log.e(TAG, "Got " + e.toString());
        } finally {
            try {
                mInterface.close();
            } catch (Exception e) {
                // ignore
            }
            mInterface = null;
            mParameters = null;

            mHandler.sendEmptyMessage(R.string.disconnected);
            Log.i(TAG, "Exiting");
        }
    }

    private boolean start() throws Exception {
        SocketChannel tunnel = null;
        boolean connected = false;
        try {
            // Packets to be sent are queued in this input stream.
//            FileInputStream in = new FileInputStream(mInterface.getFileDescriptor());
//
//            // Packets received need to be written to this output stream.
//            FileOutputStream out = new FileOutputStream(mInterface.getFileDescriptor());
            JSONObject dzcfg = new JSONObject();
            dzcfg.put("lanIpAddr", lanIpAddr);
            dzcfg.put("localIp", localIp);
            dzcfg.put("serverIp", serverIp);
            dzcfg.put("serverPort", serverPort);
            dzcfg.put("localRoute", localRoute);
            dzcfg.put("key", key);

            configure("a," + localIp + ",32" + " r," + localRoute + " d,8.8.8.8");
            connected = true;

            LaunchFubukiClient(mInterface.getFd(), dzcfg.toString());

            Log.e(TAG, "Not Hold");
        } catch (InterruptedException e) {
            throw e;
        } catch (Exception e) {
            mInterface.close();
            Log.e(TAG, "Got " + e.toString());
        } finally {
            try {
                tunnel.close();
            } catch (Exception e) {
                // ignore
            }
        }
        return connected;
    }

    private void configure(String parameters) throws Exception {
        // If the old interface has exactly the same parameters, use it!
        if (mInterface != null && parameters.equals(mParameters)) {
            Log.i(TAG, "Using the previous interface");
            return;
        }

        // Configure a builder while parsing the parameters.
        Builder builder = new Builder();
        for (String parameter : parameters.split(" ")) {
            String[] fields = parameter.split(",");
            try {
                switch (fields[0].charAt(0)) {
                    case 'm':
                        builder.setMtu(Short.parseShort(fields[1]));
                        break;
                    case 'a':
                        builder.addAddress(fields[1], Integer.parseInt(fields[2]));
                        break;
                    case 'r':
                        builder.addRoute(fields[1], Integer.parseInt(fields[2]));
                        break;
                    case 'd':
                        builder.addDnsServer(fields[1]);
                        break;
                    case 's':
                        builder.addSearchDomain(fields[1]);
                        break;
                }
            } catch (Exception e) {
                throw new IllegalArgumentException("Bad parameter: " + parameter);
            }
        }

        // Close the old interface since the parameters have been changed.
        try {
            mInterface.close();
        } catch (Exception e) {
            // ignore
        }

        // Create a new interface using the builder and save the parameters.
        mInterface = builder.setSession(serverIp)
                .setConfigureIntent(mConfigureIntent)
                .setBlocking(true)
                .establish();
        mParameters = parameters;
        Log.i(TAG, "New interface: " + parameters);
    }
}
