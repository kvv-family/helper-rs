import { Injectable } from '@angular/core';
import { check, Update } from '@tauri-apps/plugin-updater';
import { Subject } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class Settings {
  needUpdate: Subject<Update> = new Subject<Update>();



  checkUpdate() {
    check({ timeout: 30000 }).then((update) => {
      console.log("update", update);
      if (update) {
        this.needUpdate.next(update);
      }
    });
  }
}
