import React, { useState } from 'react';
import { AppStateModel } from '../types';
import { Dashboard } from './Dashboard';
import { EmergencyStopController } from './EmergencyStopController';

interface AppShellProps {
  appState: AppStateModel;
  onEmergencyStopChange: (status: AppStateModel['emergencyStop']) => void;
}

export const AppShell: React.FC<AppShellProps> = ({ appState, onEmergencyStopChange }) => {
  const [activeTab, setActiveTab] = useState<'dashboard' | 'emergency'>('dashboard');

  return (
    <div className="app-shell">
      <nav className="sidebar">
        <h1>🛡️ AI Agent</h1>
        <div className="nav-links">
          <div 
            className={`nav-item ${activeTab === 'dashboard' ? 'active' : ''}`}
            onClick={() => setActiveTab('dashboard')}
          >
            대시보드
          </div>
          <div 
            className={`nav-item ${activeTab === 'emergency' ? 'active' : ''}`}
            onClick={() => setActiveTab('emergency')}
          >
            긴급 정지 컨트롤
          </div>
        </div>
      </nav>

      <main className="main-content">
        {activeTab === 'dashboard' && <Dashboard appState={appState} />}
        {activeTab === 'emergency' && (
          <EmergencyStopController 
            status={appState.emergencyStop} 
            onChange={onEmergencyStopChange} 
          />
        )}
      </main>
    </div>
  );
};
