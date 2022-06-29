import { NativeModules } from 'react-native'

export interface FubukiConfig {
    serverIp?: string
    serverPort?: string
    localIp?: string
    key?: string
    localRoute?: string
    lanIpAddr?: string
}

interface INativeVpn {
    destory(): void
    open(cfg: FubukiConfig): Promise<null>
}

export const GetNativeVpn = (): INativeVpn => {
    return {
        destory: () => {
            NativeModules.NativeVpn.destory()
        },
        open: async (cfg: FubukiConfig) => {
            await NativeModules.NativeVpn.open(cfg)
            return null
        }
    }
}