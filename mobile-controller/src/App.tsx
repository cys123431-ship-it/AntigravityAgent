import { useState, useEffect, useRef } from 'react';
import Peer, { DataConnection } from 'peerjs';
import { Send, LogOut, CheckCircle2, XCircle } from 'lucide-react';
import './index.css';

type Message = {
  id: string;
  sender: 'user' | 'agent';
  text: string;
};

function App() {
  const [pairingCode, setPairingCode] = useState('');
  const [status, setStatus] = useState<'disconnected' | 'connecting' | 'connected'>('disconnected');
  const [messages, setMessages] = useState<Message[]>([]);
  const [inputText, setInputText] = useState('');
  
  const peerRef = useRef<Peer | null>(null);
  const connRef = useRef<DataConnection | null>(null);
  const chatEndRef = useRef<HTMLDivElement>(null);

  // Auto scroll to bottom of chat
  useEffect(() => {
    chatEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  const connectToDesktop = (e: React.FormEvent) => {
    e.preventDefault();
    if (pairingCode.length !== 7) return; // format: 123-456

    setStatus('connecting');
    
    const peer = new Peer();
    peerRef.current = peer;

    peer.on('open', () => {
      // Connect to the desktop agent's peer ID
      const targetId = `antigravity-agent-${pairingCode}`;
      const conn = peer.connect(targetId);
      connRef.current = conn;

      conn.on('open', () => {
        setStatus('connected');
        setMessages(prev => [...prev, { id: Date.now().toString(), sender: 'agent', text: 'Connected to Desktop! Type a command.' }]);
      });

      conn.on('data', (data: any) => {
        if (typeof data === 'string') {
          setMessages(prev => [...prev, { id: Date.now().toString(), sender: 'agent', text: data }]);
        }
      });

      conn.on('close', () => {
        setStatus('disconnected');
        setMessages(prev => [...prev, { id: Date.now().toString(), sender: 'agent', text: 'Connection closed by Desktop.' }]);
      });
      
      conn.on('error', () => {
        setStatus('disconnected');
        alert("Failed to connect to Desktop. Check the pairing code.");
      });
    });

    peer.on('error', (err) => {
      console.error(err);
      setStatus('disconnected');
      alert("PeerJS error: " + err.message);
    });
  };

  const disconnect = () => {
    connRef.current?.close();
    peerRef.current?.destroy();
    setStatus('disconnected');
    setPairingCode('');
    setMessages([]);
  };

  const sendMessage = (e: React.FormEvent) => {
    e.preventDefault();
    if (!inputText.trim() || status !== 'connected' || !connRef.current) return;

    // Send to desktop
    connRef.current.send(inputText);
    
    // Add to local UI
    setMessages(prev => [...prev, { id: Date.now().toString(), sender: 'user', text: inputText }]);
    setInputText('');
  };

  if (status === 'disconnected' || status === 'connecting') {
    return (
      <div className="screen-container">
        <div className="pairing-header">
          <div className="pairing-title">Antigravity</div>
          <p style={{ color: 'var(--text-muted)' }}>Mobile Controller</p>
        </div>

        <form onSubmit={connectToDesktop} className="glass" style={{ padding: '32px' }}>
          <div className="pairing-input-container">
            <label style={{ fontSize: '0.9rem', color: 'var(--text-muted)' }}>Enter 6-Digit Pairing Code</label>
            <input 
              type="text" 
              className="pairing-input" 
              placeholder="000-000" 
              value={pairingCode}
              onChange={e => {
                // Auto format with dash
                let val = e.target.value.replace(/[^0-9]/g, '');
                if (val.length > 3) val = val.slice(0, 3) + '-' + val.slice(3, 6);
                setPairingCode(val);
              }}
              maxLength={7}
              required
              disabled={status === 'connecting'}
            />
            <button type="submit" className="primary-btn" disabled={pairingCode.length !== 7 || status === 'connecting'}>
              {status === 'connecting' ? 'Connecting...' : 'Connect to Desktop'}
            </button>
          </div>
        </form>
      </div>
    );
  }

  return (
    <div className="screen-container" style={{ padding: '16px' }}>
      <div className="controller-header">
        <div className="status-badge">
          <div className="status-dot"></div>
          Connected
        </div>
        <button onClick={disconnect} style={{ background: 'none', border: 'none', color: 'var(--text-muted)', cursor: 'pointer' }}>
          <LogOut size={20} />
        </button>
      </div>

      <div className="chat-history">
        {messages.map(msg => (
          <div key={msg.id} className={`message ${msg.sender}`}>
            {msg.text}
          </div>
        ))}
        <div ref={chatEndRef} />
      </div>

      <form onSubmit={sendMessage} className="input-area">
        <textarea 
          className="chat-input" 
          placeholder="Command your agent..."
          value={inputText}
          onChange={e => setInputText(e.target.value)}
          onKeyDown={e => {
            if (e.key === 'Enter' && !e.shiftKey) {
              e.preventDefault();
              sendMessage(e);
            }
          }}
          rows={1}
        />
        <button type="submit" className="send-btn" disabled={!inputText.trim()}>
          <Send size={20} />
        </button>
      </form>
    </div>
  );
}

export default App;
