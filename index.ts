// Import specific types and functions we need
import { invoke, Channel } from "@tauri-apps/api/core";

// Extended types for push notifications
export type NotificationRegistrationStatus = {
  isRegistered: boolean;
  token?: string;
};

export type NotificationRegistrationResult = {
  success: boolean;
  token?: string;
  error?: string;
};

export type WatchNotificationResult = {
  success: boolean;
};

export type NotificationEventType =
  | "BACKGROUND_TAP"
  | "FOREGROUND_TAP"
  | "FOREGROUND_DELIVERY"
  | "BACKGROUND_DELIVERY";

export type NotificationEvent = {
  type: NotificationEventType;
  payload: Record<string, string>;
};

// Push notification specific functions
export async function checkRegistrationStatus(): Promise<NotificationRegistrationStatus> {
  return await invoke("plugin:push-notification|check_registration_status");
}

export async function registerForRemoteNotifications(): Promise<NotificationRegistrationResult> {
  return await invoke("plugin:push-notification|register_for_remote_notifications");
}

export async function watchNotifications(
  callback: (event: NotificationEvent) => void,
): Promise<WatchNotificationResult> {
  const channel = new Channel<NotificationEvent>();

  channel.onmessage = (message) => {
    callback(message);
  };

  return await invoke("plugin:push-notification|watch_notifications", {
    channel,
  });
}
