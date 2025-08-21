# Tauri Plugin Push Notification

This plugin provides push notification capabilities for Tauri applications, primarily supporting iOS platforms. It works alongside `tauri-plugin-notification` for complete notification functionality.

## Overview

This plugin works alongside the official `tauri-plugin-notification` to add push notification functionality. It provides:

- Push notification registration and management
- Remote notification event handling
- Cross-platform compatibility with graceful fallbacks

Note: This plugin no longer re-exports `tauri-plugin-notification`. You need to install and use both plugins separately for complete notification functionality.

## Installation

### Install Dependencies

Add both the base notification plugin and this extension to your `Cargo.toml`:

```toml
[dependencies]
tauri-plugin-notification = "2.0.0"
tauri-plugin-push-notification = { git = "https://github.com/stratorys/tauri-plugin-push-notification" }
```

### Install JavaScript Guest Bindings

Install the JavaScript bindings for both plugins:

```sh
npm add @tauri-apps/plugin-notification
npm add github:stratorys/tauri-plugin-push-notification
```

## Usage

### Rust Setup

Initialize both plugins in your Tauri application:

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_push_notification::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### JavaScript Usage

You need to import from both plugins separately:

```typescript
// Standard notification functions (from tauri-plugin-notification)
import {
    isPermissionGranted,
    requestPermission,
    sendNotification,
} from "@tauri-apps/plugin-notification";

// Push notification functions (from this plugin)
import {
    checkRegistrationStatus,
    registerForRemoteNotifications,
    watchNotifications,
} from "@stratorys/tauri-plugin-push-notification";

// Use standard notification features
if (await isPermissionGranted()) {
    await sendNotification({
        title: "Hello",
        body: "World!",
    });
}

// Use push notification features
const registrationResult = await registerForRemoteNotifications();
if (registrationResult.success) {
    console.log("Device token:", registrationResult.token);
}

// Watch for push notification events
await watchNotifications((event) => {
    console.log("Push notification received:", event);
});
```

## Available Functions

### Standard Notification Functions

Use the official `@tauri-apps/plugin-notification` for standard notification functionality:

- `isPermissionGranted(): Promise<boolean>`
- `requestPermission(): Promise<Permission>`
- `sendNotification(options: Options): void`
- And all other standard notification APIs...

### Push Notification Functions (from this plugin)

1. **`checkRegistrationStatus(): Promise<NotificationRegistrationStatus>`**
    - Checks if the app is registered for remote notifications
    - Returns registration status and device token if available

2. **`registerForRemoteNotifications(): Promise<NotificationRegistrationResult>`**
    - Registers the app for remote notifications
    - Returns success status and device token or error message

3. **`watchNotifications(callback: (event: NotificationEvent) => void): Promise<WatchNotificationResult>`**
    - Sets up a listener for push notification events
    - Returns success status of the watch operation

## Types

### Push Notification Types (added by this plugin)

```typescript
interface NotificationRegistrationStatus {
    isRegistered: boolean;
    token?: string;
}

interface NotificationRegistrationResult {
    success: boolean;
    token?: string;
    error?: string;
}

type NotificationEventType =
    | "BACKGROUND_TAP"
    | "FOREGROUND_TAP"
    | "FOREGROUND_DELIVERY"
    | "BACKGROUND_DELIVERY";

interface NotificationEvent {
    type: NotificationEventType;
    payload: Record<string, string>;
}

interface WatchNotificationResult {
    success: boolean;
}
```

### Standard Types

Import standard notification types from `@tauri-apps/plugin-notification` separately.

## Configuration

Add the notification permissions to your Tauri capabilities file (e.g., `src-tauri/capabilities/default.json`):

```json
{
    "permissions": ["notification:default"]
}
```

## iOS Setup

For push notifications on iOS:

1. Enable Push Notifications capability in your Xcode project
2. Configure your Apple Developer account for push notifications
3. Add required entitlements:
    - `aps-environment` (development or production)
4. Configure your app's Info.plist with required background modes:
    - Remote notifications

## Platform Support

| Platform | Standard Notifications | Push Notifications |
| -------- | ---------------------- | ------------------ |
| iOS      | ✅ (via base plugin)   | ✅                 |
| Android  | ✅ (via base plugin)   | 🚧 (planned)       |
| Desktop  | ✅ (via base plugin)   | ❌ (not supported) |

Desktop platforms will return appropriate "not supported" responses for push notification operations, but standard notifications work normally.

## Complete Example

```typescript
// Import from both plugins
import {
    isPermissionGranted,
    requestPermission,
    sendNotification,
} from "@tauri-apps/plugin-notification";

import {
    registerForRemoteNotifications,
    watchNotifications,
} from "@stratorys/tauri-plugin-push-notification";

async function setupNotifications() {
    try {
        // Check and request standard notification permissions
        let granted = await isPermissionGranted();
        if (!granted) {
            const permission = await requestPermission();
            granted = permission === "granted";
        }

        if (!granted) {
            console.log("Notification permissions denied");
            return;
        }

        // Send a local notification
        await sendNotification({
            title: "Welcome!",
            body: "Notifications are now enabled",
        });

        // Register for push notifications
        const registration = await registerForRemoteNotifications();
        if (registration.success) {
            console.log("Successfully registered for push notifications");
            console.log("Device token:", registration.token);

            // Set up push notification listener
            const watchResult = await watchNotifications((event) => {
                switch (event.type) {
                    case "BACKGROUND_TAP":
                        console.log("User tapped push notification in background");
                        break;
                    case "FOREGROUND_TAP":
                        console.log("User tapped push notification in foreground");
                        break;
                    case "FOREGROUND_DELIVERY":
                        console.log("Push notification received in foreground");
                        break;
                    case "BACKGROUND_DELIVERY":
                        console.log("Push notification received in background");
                        break;
                }
                console.log("Notification payload:", event.payload);
            });

            if (watchResult.success) {
                console.log("Successfully set up push notification listener");
            }
        } else {
            console.error("Push notification registration failed:", registration.error);
        }
    } catch (error) {
        console.error("Error setting up notifications:", error);
    }
}

// Initialize notifications
setupNotifications();
```

## Migration from Previous Version

If you were using an earlier version of this plugin that re-exported `tauri-plugin-notification`:

1. Install `@tauri-apps/plugin-notification` separately as shown above
2. Update your imports to import from both plugins separately (see examples above)
3. Update your Rust code to initialize both plugins
4. Standard notification functions now come from `@tauri-apps/plugin-notification`
5. Push notification specific functions remain in this plugin

## Contributing

We welcome contributions! Please feel free to submit issues and pull requests.

## License

This plugin is licensed under either of:

- Apache License, Version 2.0
- MIT license

at your option.

## Acknowledgments

This plugin extends the official `tauri-plugin-notification` and is maintained by the Stratorys team and the Tauri community.
