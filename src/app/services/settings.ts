import { Injectable } from '@angular/core';
import { check } from '@tauri-apps/plugin-updater';

@Injectable({
  providedIn: 'root'
})
export class Settings {
  

  checkUpdate() {
    check({timeout: 30000}).then((update) => {
      console.log(update);
    });
  }
}
