import { Component, signal } from '@angular/core';
import { InputGroupModule } from 'primeng/inputgroup';
import { InputGroupAddonModule } from 'primeng/inputgroupaddon';
import { InputTextModule } from 'primeng/inputtext';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { ButtonModule } from 'primeng/button';
import { FloatLabelModule } from 'primeng/floatlabel';
import { TooltipModule } from 'primeng/tooltip';
import { FormControl, FormsModule, ReactiveFormsModule } from '@angular/forms';
import { PanelModule } from 'primeng/panel';
import { ProgressBarModule } from 'primeng/progressbar';
import { ToastModule } from 'primeng/toast';
import { MessageService } from 'primeng/api';
import { RadioButtonModule } from 'primeng/radiobutton';
import { InputNumberModule } from 'primeng/inputnumber';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-watermarks',
  imports: [
    CommonModule, ReactiveFormsModule, FormsModule, InputGroupModule, InputGroupAddonModule, InputTextModule, ButtonModule, FloatLabelModule, TooltipModule, PanelModule, ProgressBarModule, ToastModule, RadioButtonModule, InputNumberModule],
  templateUrl: './watermarks.html',
  styleUrl: './watermarks.scss',
  providers: [MessageService]
})
export class Watermarks {
  pathInput: FormControl = new FormControl('');
  pathWatermark: FormControl = new FormControl('');
  pathOutput: FormControl = new FormControl('');
  nameOutput: string = 'watermark';
  nameOutputFile: string = 'origin';
  formatOutput: string = 'original';
  startIndex = new FormControl(1);
  valueProgress: number = 0;

  setDirectory(type: 'input' | 'watermark' | 'output') {
    open({
      multiple: false,
      directory: true,
    }).then((directory: string | null) => {
      if (directory) {
        if (type === 'input') {
          this.pathInput.setValue(directory);
        } else if (type === 'watermark') {
          this.pathWatermark.setValue(directory);
        } else if (type === 'output') {
          this.pathOutput.setValue(directory);
        }
      }
    });
    // invoke('image_start');
  }
}
