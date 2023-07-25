// tauri.js
import { invoke } from '@tauri-apps/api/tauri';

export async function closeSplashscreen() {
  return invoke('close_splashscreen');
}
