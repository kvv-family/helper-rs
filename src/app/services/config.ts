import { Injectable } from '@angular/core';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { BehaviorSubject } from 'rxjs';

export interface Config {
  path_input: string;
  path_watermark: string;
  path_output: string;
  name_output: string;
  name_output_file: string;
  format_output: string;
}

@Injectable({
  providedIn: 'root'
})
export class ConfigService {
  config: BehaviorSubject<Config | null> = new BehaviorSubject<Config | null>(null);
  constructor() {
    listen<Config>('handler_config', (event) => {
      console.log("Event ", event.payload);
      this.config.next(event.payload);
    });
  }

  getConfig() {
    invoke('handler_config');
  }
}
