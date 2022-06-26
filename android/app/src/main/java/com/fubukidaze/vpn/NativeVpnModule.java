package com.fubukidaze.vpn;

import static android.content.ContentValues.TAG;

import android.content.Intent;
import android.util.Log;

import com.facebook.react.bridge.Arguments;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.bridge.ReactContext;
import com.facebook.react.bridge.ReactContextBaseJavaModule;
import com.facebook.react.bridge.ReactMethod;
import com.facebook.react.bridge.ReadableMap;
import com.facebook.react.bridge.WritableMap;
import com.facebook.react.modules.core.DeviceEventManagerModule;


/**
 * SplashScreen
 * 启动屏
 * from：http://www.devio.org
 * Author:CrazyCodeBoy
 * GitHub:https://github.com/crazycodeboy
 * Email:crazycodeboy@gmail.com
 */
public class NativeVpnModule extends ReactContextBaseJavaModule {
    public NativeVpnModule(ReactApplicationContext reactContext) {
        super(reactContext);
    }

    @Override
    public String getName() {
        return "NativeVpn";
    }

    static {
        System.loadLibrary("fubuki");
    }

    public interface JNICallback {
        void callback(String string);
    }

    public static native String TestJni();
    public static native void LaunchFubukiClient() throws Exception;
    public static native void DestoryFubukiClient();

    public static native void invokeCallbackViaJNI(JNICallback callback);

    /**
     * SendEvent
     */
    @ReactMethod
    public void destory() {
        Log.d(TAG, "destory");

        return;
    }

    /**
     * ReceiveEvent
     */
    @ReactMethod
    public void open(ReadableMap cfg) {
        Log.d(TAG, "open rust: " + TestJni());

        WritableMap idData = Arguments.createMap();
        idData.merge(cfg);

        startVPN(cfg);
    }

    private void startVPN(ReadableMap cfg) {
        Log.d(TAG, "startVPN"
                + ", lanIpAddr: "
                + cfg.getString("lanIpAddr")
                + ", localIp: "
                + cfg.getString("localIp")
                + ", serverIp: "
                + cfg.getString("serverIp")
                + ", serverPort: "
                + cfg.getString("serverPort")
                + ", localMask: "
                + cfg.getString("localMask")
                + ", trySendToLanAddr: "
                + cfg.getBoolean("trySendToLanAddr")
                + ", key: "
                + cfg.getString("key")
                + ", mode: "
                + cfg.getString("mode"));

        try {
            LaunchFubukiClient();
        }catch (Exception e) {
            Log.d(TAG, "startVPN: " + e);
        }



//        Intent intent = NativeVpnService.prepare(getReactApplicationContext());
//        if (intent != null) {
//            Log.e(TAG, "Bad Prepare, er: " + intent);
//            getCurrentActivity().startActivityForResult(intent, 0);
//        } else {
//            Log.e(TAG, "Allowed Prepare, er: " + intent);
//            Intent startIntent = new Intent(getReactApplicationContext(), NativeVpnService.class)
//                    .putExtra("lanIpAddr", cfg.getString("lanIpAddr"))
//                    .putExtra("localIp", cfg.getString("localIp"))
//                    .putExtra("serverIp", cfg.getString("serverIp"))
//                    .putExtra("serverPort", cfg.getString("serverPort"))
//                    .putExtra("localMask", cfg.getString("localMask"))
//                    .putExtra("trySendToLanAddr", cfg.getBoolean("trySendToLanAddr"))
//                    .putExtra("key", cfg.getString("key"))
//                    .putExtra("mode", cfg.getString("mode"));
//
//            getReactApplicationContext().startService(startIntent);
//        }
    }

    public void sendEventToJs(ReactContext reactContext, String eventName, WritableMap params) {
        Log.d(TAG, "sendEventToJs: " + eventName + "===" + params);
        reactContext
                .getJSModule(DeviceEventManagerModule.RCTDeviceEventEmitter.class)
                .emit(eventName, params);
    }
}


