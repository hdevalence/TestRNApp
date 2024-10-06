import React, { useState } from 'react';
import { View, Text, Button, TextInput } from 'react-native';
import PraxMobile from './PraxMobile';

const ExampleComponent: React.FC = () => {
  const [num1, setNum1] = useState<string>('0');
  const [num2, setNum2] = useState<string>('0');
  const [result, setResult] = useState<number | null>(null);

  const handleSum = async () => {
    try {
      const sum = await PraxMobile.sum(parseInt(num1), parseInt(num2));
      setResult(sum);
    } catch (error) {
      console.error('Error calling sum function:', error);
    }
  };

  return (
    <View>
      <TextInput
        value={num1}
        onChangeText={setNum1}
        keyboardType="numeric"
        placeholder="Enter first number"
      />
      <TextInput
        value={num2}
        onChangeText={setNum2}
        keyboardType="numeric"
        placeholder="Enter second number"
      />
      <Button title="Calculate Sum" onPress={handleSum} />
      {result !== null && <Text>Result: {result}</Text>}
    </View>
  );
};

export default ExampleComponent;
