import { Component, signal, OnInit } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { ConfigService } from './services/config';
import { Settings } from './services/settings';


@Component({
  selector: 'app-root',
  imports: [RouterOutlet],
  templateUrl: './app.html',
})
export class App implements OnInit {
  protected readonly title = signal('image-helper');

  constructor(private config: ConfigService, private settings: Settings) {
  }

  ngOnInit(): void {
    this.config.getConfig();
    this.settings.checkUpdate();
  }
}
