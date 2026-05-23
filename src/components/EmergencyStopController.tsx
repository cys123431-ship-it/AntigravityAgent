import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { EmergencyStopStatus } from '../types';

interface EmergencyStopControllerProps {
  status: EmergencyStopStatus;
  onChange: (status: EmergencyStopStatus) => void;
}

export const EmergencyStopController: React.FC<EmergencyStopControllerProps> = ({ status, onChange }) => {
  const [loading, setLoading] = useState(false);

  const toggleStatus = async () => {
    setLoading(true);
    try {
      const newStatus = status === 'enabled' ? 'disabled' : 'enabled';
      await invoke('set_emergency_stop_status', { status: newStatus });
      onChange(newStatus);
    } catch (err) {
      console.error("Failed to toggle emergency stop", err);
      // In a real app we'd show an error toast here
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="emergency-stop-container">
      <button 
        className={`emergency-btn ${status}`}
        onClick={toggleStatus}
        disabled={loading}
        aria-label={status === 'enabled' ? "긴급 정지 해제" : "긴급 정지 가동"}
      >
        <span>{status === 'enabled' ? 'RESUME' : 'STOP'}</span>
      </button>
      <div className="emergency-status-text">
        {status === 'enabled' 
          ? '모든 에이전트 작업이 일시 정지되었습니다.' 
          : '에이전트가 정상 작동 중입니다.'}
      </div>
    </div>
  );
};
