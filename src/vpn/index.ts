import { Alert, DeviceEventEmitter, EmitterSubscription, EventEmitter, NativeEventEmitter, NativeModules } from 'react-native'

interface IPacketReader {
    onData(data: any): void
}

interface IPacketWriter {
    write(data: Uint8Array): Promise<void>
}

export interface FubukiConfig {
    serverIp?: string
    serverPort?: string
    localIp?: string
    localMask?: string
    key?: string
    mode?: "UDP_AND_TCP" | "UDP" | "TCP"
    lanIpAddr?: string
    trySendToLanAddr?: boolean
}

interface INativeVpn {
    destory(): void
    open(cfg: FubukiConfig, reader: IPacketReader): Promise<IPacketWriter | null>
}

interface IPacket {
    body: Uint8Array
}

export const GetNativeVpn = (): INativeVpn => {
    let sb: EmitterSubscription

    return {
        destory: () => {
            NativeModules.NativeVpn.destory()

            sb.remove()
        },
        open: async (cfg: FubukiConfig, reader: IPacketReader) => {
            sb = await DeviceEventEmitter.addListener("recvvpndata", (packet: IPacket) => {
                reader.onData(packet)
            })

            await NativeModules.NativeVpn.open(cfg)
            return null
        }
    }
}