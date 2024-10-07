import React, { useState } from 'react';
import { View, Text, TextInput, Button, StyleSheet } from 'react-native';
import PraxMobile from './PraxMobile';

const ServerExampleComponent: React.FC = () => {
  const [serverStatus, setServerStatus] = useState<string>('Not started');
  const [name, setName] = useState<string>('');
  const [response, setResponse] = useState<string>('');

  const startServer = async () => {
    try {
      const result = await PraxMobile.startServer();
      setServerStatus(result);
    } catch (error) {
      console.error('Failed to start server:', error);
      setServerStatus('Failed to start server');
    }
  };

  const stopServer = async () => {
    try {
      const result = await PraxMobile.stopServer();
      setServerStatus(result);
    } catch (error) {
      console.error('Failed to stop server:', error);
      setServerStatus('Failed to stop server');
    }
  };

  const makeRequest = async () => {
    try {
      const response = await fetch(`http://localhost:3030/hello/${name}`);
      const text = await response.text();
      setResponse(text);
    } catch (error) {
      console.error('Error making request:', error);
      setResponse('Error making request');
    }
  };

  return (
    <View style={styles.container}>
      <Text style={styles.status}>Server Status: {serverStatus}</Text>
      <Button title="Start Server" onPress={startServer} />
      <Button title="Stop Server" onPress={stopServer} />

      <TextInput
        style={styles.input}
        value={name}
        onChangeText={setName}
        placeholder="Enter your name"
      />
      <Button title="Submit" onPress={makeRequest} />

      <Text style={styles.response}>{response}</Text>
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    padding: 20,
  },
  status: {
    fontSize: 18,
    marginBottom: 10,
  },
  input: {
    height: 40,
    borderColor: 'gray',
    borderWidth: 1,
    marginTop: 20,
    marginBottom: 10,
    paddingHorizontal: 10,
  },
  response: {
    marginTop: 20,
    fontSize: 16,
  },
});

export default ServerExampleComponent;