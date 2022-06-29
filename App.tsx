import React, { useState } from 'react';
import {
  Button,
  Dimensions,
  ImageBackground,
  KeyboardAvoidingView,
  SafeAreaView,
  StyleSheet,
  TextInput,
  ToastAndroid,
} from 'react-native';

const nh = Dimensions.get("screen").height
const nw = Dimensions.get("screen").width

import { GetNativeVpn } from './src/vpn';

const App = () => {
  const [lanIpAddr, setLanIpAddr] = useState<string>()
  const [localIp, setLocalIp] = useState<string>()
  const [serverIp, setServerIp] = useState<string>()
  const [serverPort, setServerPort] = useState<string>()
  const [localRoute, setLocalRoute] = useState<string>()
  const [key, setKey] = useState<string>()

  return (
    <SafeAreaView>
      <KeyboardAvoidingView enabled behavior='padding' style={styles.Container}>
        <ImageBackground source={require("./src/assert/fubuki.jpg")} style={styles.Container}>
          <TextInput
            style={styles.InputBase}
            placeholder={"lanIpAddr(192.x.x.x)"}
            placeholderTextColor={"blue"}
            value={lanIpAddr}
            onChangeText={(v) => {
              setLanIpAddr(v)
            }} />
          <TextInput
            style={styles.InputBase}
            placeholder={"localIp(x.x.x.x)"}
            placeholderTextColor={"blue"}
            value={localIp}
            onChangeText={(v) => {
              setLocalIp(v)
            }} />
          <TextInput
            style={styles.InputBase}
            placeholder={"serverIp(x.x.x.x)"}
            placeholderTextColor={"blue"}
            value={serverIp}
            onChangeText={(v) => {
              setServerIp(v)
            }} />
          <TextInput
            style={styles.InputBase}
            placeholder={"serverPort(xxx)"}
            placeholderTextColor={"blue"}
            value={serverPort}
            onChangeText={(v) => {
              setServerPort(v)
            }} />
          <TextInput
            style={styles.InputBase}
            placeholder={"localRoute(192.x.x.x) Don't use 0.0.0.0,0"}
            placeholderTextColor={"blue"}
            value={localRoute}
            onChangeText={(v) => {
              setLocalRoute(v)
            }} />
          <TextInput
            style={styles.InputBase}
            placeholder={"key(xxx)"}
            placeholderTextColor={"blue"}
            value={key}
            onChangeText={(v) => {
              setKey(v)
            }} />
          <Button
            title='Connect'
            color={"green"}
            onPress={() => {
              GetNativeVpn().open({
                lanIpAddr: lanIpAddr,
                localIp: localIp,
                serverIp: serverIp,
                serverPort: serverPort,
                localRoute: localRoute,
                key: key,
              }).then(_ => {
                ToastAndroid.show("Connected!", ToastAndroid.SHORT)
              }).catch(e => {
                ToastAndroid.show("Connect Error: !" + e, ToastAndroid.SHORT)
              })
            }} />
            
          <Button
            title='Disconnect'
            color={"pink"}
            onPress={() => {
              GetNativeVpn().destory()
              ToastAndroid.show("Disconnected!", ToastAndroid.SHORT)
            }} />
        </ImageBackground>
      </KeyboardAvoidingView>
    </SafeAreaView>
  );
};

const styles = StyleSheet.create({
  Background: {
    width: "100%",
    height: "100%"
  },
  Container: {
    width: nw,
    height: nh,
    display: "flex",
    justifyContent: "center",
    alignItems: "center",
    overflow: "scroll"
  },
  InputBase: {
    width: "80%",
    height: "6%",
    backgroundColor: "yellow",
    opacity: 0.4,
    margin: "1%"
  },
  BtntBase: {
    width: "80%",
    height: "6%",
    opacity: 0.4,
    margin: "1%"
  }
});

export default App;
