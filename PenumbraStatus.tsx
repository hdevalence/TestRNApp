import React, { useState, useEffect } from 'react';
import { View, Text, Button, StyleSheet } from 'react-native';
import { penumbraClient } from './penumbraClient';

const PenumbraStatus: React.FC = () => {
  const [status, setStatus] = useState<string>('');
  const [error, setError] = useState<string>('');

  const fetchStatus = async () => {
    try {
      console.log("Got Here");
      const response1 = await penumbraClient.status({});
      console.log(response1);

      setStatus(JSON.stringify(response1.toJson(), null, 2));
      setError('');
    } catch (err) {
      setError('Error fetching status: ' + (err instanceof Error ? err.message : String(err)));
      setStatus('');
    }
  };

  useEffect(() => {
    fetchStatus();
  }, []);

  return (
    <View style={styles.container}>
      <Text style={styles.title}>Penumbra Status</Text>
      {status ? (
        <Text style={styles.status}>{status}</Text>
      ) : error ? (
        <Text style={styles.error}>{error}</Text>
      ) : (
        <Text>Loading...</Text>
      )}
      <Button title="Refresh Status" onPress={fetchStatus} />
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    padding: 20,
  },
  title: {
    fontSize: 20,
    fontWeight: 'bold',
    marginBottom: 10,
  },
  status: {
    marginBottom: 10,
  },
  error: {
    color: 'red',
    marginBottom: 10,
  },
});

export default PenumbraStatus;
