import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { AppShell } from "./components/AppShell";
import { AppStateModel, EmergencyStopStatus, ProviderStatus, RelayStatus } from "./types";
import "./index.css";

function App() {
  const [appState, setAppState] = useState<AppStateModel>({
    emergencyStop: 'disabled',
    provider: 'unknown',
    relay: 'disconnected'
  });

  useEffect(() => {
    // Initial fetch of status
    const fetchStatus = async () => {
      try {
        const estop = await invoke<string>('get_emergency_stop_status');
        const providerStatus = await invoke<string>('get_provider_status');
        const relayStatus = await invoke<string>('get_relay_status');
        
        setAppState(prev => ({ 
          ...prev, 
          emergencyStop: estop as EmergencyStopStatus,
          provider: providerStatus as ProviderStatus,
          relay: relayStatus as RelayStatus
        }));
      } catch (err) {
        console.error("Failed to fetch status", err);
      }
    };
    fetchStatus();
  }, []);

  const handleEmergencyStopChange = (status: EmergencyStopStatus) => {
    setAppState(prev => ({ ...prev, emergencyStop: status }));
  };

  return (
    <AppShell 
      appState={appState} 
      onEmergencyStopChange={handleEmergencyStopChange} 
    />
  );
}

export default App;
