export type EmergencyStopStatus = 'enabled' | 'disabled';
export type ProviderStatus = 'unknown' | 'missing' | 'logged_out' | 'ready' | 'error';
export type RelayStatus = 'disconnected' | 'testing' | 'connected' | 'error';
export type CommandStatus = 'received' | 'classified' | 'approval_required' | 'blocked' | 'executing' | 'succeeded' | 'failed';
export type ApprovalStatus = 'pending' | 'approved' | 'denied' | 'expired';

export interface AppStateModel {
  emergencyStop: EmergencyStopStatus;
  provider: ProviderStatus;
  relay: RelayStatus;
}
