import { Component, signal, OnInit } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { ConfigService } from './services/config';
import { Settings } from './services/settings';
import { ConfirmDialogModule } from 'primeng/confirmdialog';
import { ToastModule } from 'primeng/toast';
import { ConfirmationService, MessageService } from 'primeng/api';



@Component({
  selector: 'app-root',
  imports: [RouterOutlet, ConfirmDialogModule, ToastModule],
  templateUrl: './app.html',
  providers: [ConfirmationService, MessageService]
})
export class App implements OnInit {
  protected readonly title = signal('image-helper');

  constructor(private config: ConfigService, private settings: Settings, private confirmationService: ConfirmationService) {
  }

  ngOnInit(): void {
    this.config.getConfig();
    this.settings.needUpdate.subscribe((update) => {
      this.confirmationService.confirm({
        message: 'Доступно новое обновление',
        header: 'Обновление',
        icon: 'pi pi-exclamation-triangle',
        acceptLabel: 'Обновить',
        rejectLabel: 'Отмена',
        acceptIcon: 'pi pi-check',
        rejectIcon: 'pi pi-times',
      });
    });
    this.settings.checkUpdate();
  }
}
