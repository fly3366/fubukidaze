/**
 * Sample React Native App
 * https://github.com/facebook/react-native
 *
 * Generated with the TypeScript template
 * https://github.com/react-native-community/react-native-template-typescript
 *
 * @format
 */

 import React, { ReactNode, useState } from 'react';
 import {
   Alert,
   Button,
   NativeModules,
   SafeAreaView,
   ScrollView,
   StatusBar,
   StyleSheet,
   Text,
   useColorScheme,
   View,
 } from 'react-native';
 
 import {
   Colors,
   DebugInstructions,
   Header,
   LearnMoreLinks,
   ReloadInstructions,
 } from 'react-native/Libraries/NewAppScreen';
 import { FubukiConfig, GetNativeVpn } from './src/vpn';
 
 const Section: React.FC<{
   title: string;
   children?: ReactNode;
 }> = ({ children, title }) => {
   const isDarkMode = useColorScheme() === 'dark';
   return (
     <View style={styles.sectionContainer}>
       <Text
         style={[
           styles.sectionTitle,
           {
             color: isDarkMode ? Colors.white : Colors.black,
           },
         ]}>
         {title}
       </Text>
       <Text
         style={[
           styles.sectionDescription,
           {
             color: isDarkMode ? Colors.light : Colors.dark,
           },
         ]}>
         {children}
       </Text>
     </View>
   );
 };
 
 const App = () => {
   const isDarkMode = useColorScheme() === 'dark';
 
   const backgroundStyle = {
     backgroundColor: isDarkMode ? Colors.darker : Colors.lighter,
   };
 
   const [cfg, setCfg] = useState<FubukiConfig>({lanIpAddr: "2222"})
   const [data, setData] = useState<any>("Null")
 
   return (
     <SafeAreaView style={backgroundStyle}>
       <Text>
         {data}</Text>
         <Button title='Fubuki!' onPress={()=> {
           GetNativeVpn().open({
             lanIpAddr: "192.168.123.20",
             localIp: "192.0.0.11",
             serverIp: "8.130.13.97",
             serverPort: "12345",
             localMask: "255.255.255.0",
             trySendToLanAddr: false,
             key: "dxk",
             mode: "UDP_AND_TCP"
           }, {
             onData: (data) => {
               setData(JSON.stringify(data))
             }
           }).then(e => {
             setData("Enable")
           }).catch(e => {
             
           })
         }}/>
     </SafeAreaView>
   );
 };
 
 const styles = StyleSheet.create({
   sectionContainer: {
     marginTop: 32,
     paddingHorizontal: 24,
   },
   sectionTitle: {
     fontSize: 24,
     fontWeight: '600',
   },
   sectionDescription: {
     marginTop: 8,
     fontSize: 18,
     fontWeight: '400',
   },
   highlight: {
     fontWeight: '700',
   },
 });
 
 export default App;
 