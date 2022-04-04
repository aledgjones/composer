export interface ToastConfig {
  text: string;
  button?: string;
  duration?: number;
  onClick?: () => void;
  onTimeout?: () => void;
}

export interface ToastInstance extends ToastConfig {
  key: string;
}
