import React from 'react';
import { invoke } from '@tauri-apps/api/core';
import { AppStateModel } from '../types';

interface DashboardProps {
  appState: AppStateModel;
}

export const Dashboard: React.FC<DashboardProps> = ({ appState }) => {
  const [pairingCode, setPairingCode] = React.useState<string | null>(null);

  React.useEffect(() => {
    // Left empty for now, or you can add relay polling later
  }, []);

  return (
    <div>
      <h2 style={{ marginTop: 0, marginBottom: '2rem' }}>System Dashboard</h2>
      
      <div className="dashboard-grid">
        <div className="card">
          <div className="card-title">Provider Status</div>
          <div className="status-value" style={{ color: 'var(--text-muted)', fontSize: '1rem', marginBottom: '0.5rem' }}>
            Current: {appState.provider}
          </div>
          {appState.provider.includes("WebView") && (
            <div style={{ marginTop: '0.5rem' }}>
              <button onClick={() => invoke('open_webview').then(() => window.location.reload())} style={{ padding: '0.5rem 1rem', background: 'var(--primary)', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer' }}>
                Open AI Login Window
              </button>
            </div>
          )}
          {appState.provider.includes("Needs Token") && (
            <div style={{ display: 'flex', gap: '0.5rem', marginTop: '0.5rem' }}>
              <input id="session-token-input" type="password" placeholder="Enter Web Session Token" style={{ padding: '0.25rem', flex: 1, background: 'var(--bg-color)', color: 'white', border: '1px solid var(--text-muted)' }} />
              <button onClick={() => {
                const token = (document.getElementById('session-token-input') as HTMLInputElement).value;
                if (token) invoke('set_provider_token', { token }).then(() => window.location.reload());
              }} style={{ padding: '0.25rem 0.5rem' }}>Save</button>
            </div>
          )}
          <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.5rem', marginTop: '1rem' }}>
            <button onClick={() => invoke('switch_provider', { providerType: 'webview_chatgpt' }).then(() => window.location.reload())} style={{ padding: '0.25rem 0.5rem', fontSize: '0.8rem' }}>ChatGPT (WebView)</button>
            <button onClick={() => invoke('switch_provider', { providerType: 'webview_gemini' }).then(() => window.location.reload())} style={{ padding: '0.25rem 0.5rem', fontSize: '0.8rem' }}>Gemini (WebView)</button>
            <button onClick={() => invoke('switch_provider', { providerType: 'mock' }).then(() => window.location.reload())} style={{ padding: '0.25rem 0.5rem', fontSize: '0.8rem' }}>Mock</button>
            <button onClick={() => invoke('switch_provider', { providerType: 'codex' }).then(() => window.location.reload())} style={{ padding: '0.25rem 0.5rem', fontSize: '0.8rem' }}>Codex CLI</button>
            <button onClick={() => invoke('switch_provider', { providerType: 'gemini' }).then(() => window.location.reload())} style={{ padding: '0.25rem 0.5rem', fontSize: '0.8rem' }}>Gemini CLI</button>
          </div>
        </div>

        <div className="card">
          <div className="card-title">Mobile Cloud Relay</div>
          <div className="status-value" style={{ color: 'var(--text-muted)', fontSize: '1rem', marginBottom: '0.5rem' }}>
            {appState.relay}
          </div>
          <div style={{ marginTop: '1rem' }}>
            <button onClick={async () => {
              const code = await invoke<string>('generate_pairing_code');
              setPairingCode(code);
              window.location.reload();
            }} style={{ padding: '0.5rem 1rem', background: 'var(--primary)', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer', width: '100%', marginBottom: '1rem' }}>
              Generate Pairing Code
            </button>
            {pairingCode && (
              <div style={{ padding: '1rem', background: '#222', textAlign: 'center', borderRadius: '8px', border: '2px dashed var(--primary)' }}>
                <div style={{ fontSize: '0.9rem', color: 'var(--text-muted)', marginBottom: '0.5rem' }}>Enter this code in the Mobile App</div>
                <div style={{ fontSize: '2rem', letterSpacing: '4px', color: 'white', fontWeight: 'bold' }}>{pairingCode}</div>
              </div>
            )}
          </div>
        </div>

        <div className="card" style={{ borderLeft: `4px solid ${appState.emergencyStop === 'enabled' ? 'var(--warning)' : 'var(--success)'}` }}>
          <div className="card-title">Emergency Stop</div>
          <div className="status-value" style={{ color: appState.emergencyStop === 'enabled' ? 'var(--warning)' : 'var(--success)' }}>
            {appState.emergencyStop === 'enabled' ? 'ENGAGED' : 'SAFE'}
          </div>
        </div>
      </div>

      <h2 style={{ marginTop: '2rem', marginBottom: '1rem' }}>Test Telegram Command</h2>
      <div className="card">
        <CommandSimulator />
      </div>
    </div>
  );
};

const CommandSimulator = () => {
  const [cmd, setCmd] = React.useState('');
  const [result, setResult] = React.useState('');
  const [loading, setLoading] = React.useState(false);

  const handleTest = async () => {
    setLoading(true);
    try {
      const res = await invoke('simulate_telegram_message', { message: cmd });
      setResult(`SUCCESS: ${res}`);
    } catch (e) {
      setResult(`ERROR: ${e}`);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '1rem' }}>
      <input 
        style={{ padding: '0.5rem', background: 'var(--bg-color)', color: 'white', border: '1px solid var(--text-muted)', borderRadius: '4px' }}
        value={cmd} onChange={e => setCmd(e.target.value)} placeholder="Enter command (e.g. echo hello, delete file, steal cookie)" 
      />
      <button 
        style={{ padding: '0.5rem 1rem', background: 'var(--primary)', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer' }}
        onClick={handleTest} disabled={loading || !cmd}>
        Simulate Send
      </button>
      {result && <div style={{ marginTop: '1rem', padding: '1rem', background: 'var(--bg-color)', borderRadius: '4px' }}>{result}</div>}
    </div>
  );
};

