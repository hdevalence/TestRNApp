import { NativeModules } from 'react-native';

interface PraxMobileInterface {
  sum(a: number, b: number): Promise<number>;
  startServer(): Promise<string>;
  stopServer(): Promise<string>;
}

const { PraxMobile } = NativeModules;

export default {
  sum: (a: number, b: number): Promise<number> => {
    return PraxMobile.sum(a, b);
  },
  startServer: (): Promise<string> => {
    return PraxMobile.startServer();
  },
  stopServer: (): Promise<string> => {
    return PraxMobile.stopServer();
  }
} as PraxMobileInterface;