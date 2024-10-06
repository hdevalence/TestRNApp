import { NativeModules } from 'react-native';

interface PraxMobileInterface {
  sum(a: number, b: number): Promise<number>;
}

const { PraxMobile } = NativeModules;

export default {
  sum: (a: number, b: number): Promise<number> => {
    return PraxMobile.sum(a, b);
  }
} as PraxMobileInterface;

