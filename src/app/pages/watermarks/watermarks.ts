import { ChangeDetectorRef, Component, OnInit } from '@angular/core';
import { InputGroupModule } from 'primeng/inputgroup';
import { InputGroupAddonModule } from 'primeng/inputgroupaddon';
import { InputTextModule } from 'primeng/inputtext';
import { open } from '@tauri-apps/plugin-dialog';
import { ButtonModule } from 'primeng/button';
import { FloatLabelModule } from 'primeng/floatlabel';
import { TooltipModule } from 'primeng/tooltip';
import { FormBuilder, FormControl, FormGroup, FormGroupDirective, FormsModule, NgForm, ReactiveFormsModule, Validators } from '@angular/forms';
import { PanelModule } from 'primeng/panel';
import { ProgressBarModule } from 'primeng/progressbar';
import { ToastModule } from 'primeng/toast';
import { MessageService } from 'primeng/api';
import { RadioButtonModule } from 'primeng/radiobutton';
import { InputNumberModule } from 'primeng/inputnumber';
import { CommonModule } from '@angular/common';
import { listen } from '@tauri-apps/api/event';
import { MessageModule } from 'primeng/message';
import { invoke } from '@tauri-apps/api/core';
import { ConfigService } from '../../services/config';

interface FilesCount {
  watermark: number;
  inputs: number;
}

@Component({
  selector: 'app-watermarks',
  imports: [
    CommonModule, ReactiveFormsModule, FormsModule, InputGroupModule, InputGroupAddonModule, InputTextModule, ButtonModule, FloatLabelModule, TooltipModule, PanelModule, ProgressBarModule, ToastModule, RadioButtonModule, InputNumberModule, MessageModule],
  templateUrl: './watermarks.html',
  styleUrl: './watermarks.scss',
  providers: [MessageService]
})
export class Watermarks {
  generalForm: FormGroup;
  nameOutput: string = "watermark";
  nameOutputFile: string = "origin";
  formatOutput: string = "original";
  valueProgress: number = 0;
  countFiles: FilesCount | null = null;

  constructor(private fb: FormBuilder, private config: ConfigService, private cd: ChangeDetectorRef) {

    this.generalForm = this.fb.group({
      pathInput: ['', [Validators.required]],
      pathWatermark: ['', [Validators.required]],
      pathOutput: ['', [Validators.required]],
      startIndex: [1]
    })
    this.generalForm.controls['pathInput'].disable();
    this.generalForm.controls['pathWatermark'].disable();
    this.generalForm.controls['pathOutput'].disable();

    this.generalForm.valueChanges.subscribe((data) => {
      console.log(data)
      if (data.pathInput && data.pathWatermark && data.pathOutput) {
        invoke('get_count', {
          pathInput: this.generalForm.controls['pathInput'].value,
          watermarkPath: this.generalForm.controls['pathWatermark'].value,
          outputPath: this.generalForm.controls['pathOutput'].value,
          nameOutput: this.nameOutput,
          nameOutputFile: this.nameOutputFile,
          formatOutput: this.formatOutput
        });
      }
    })

  }

  ngOnInit(): void {
    this.config.config.subscribe((data) => {
      if (data) {
        this.generalForm.controls['pathInput'].setValue(data.path_input);
        this.generalForm.controls['pathWatermark'].setValue(data.path_watermark);
        this.generalForm.controls['pathOutput'].setValue(data.path_output);
        this.nameOutput = data.name_output;
        this.nameOutputFile = data.name_output_file;
        this.formatOutput = data.format_output;
        this.cd.detectChanges();
      }
    });
    listen<FilesCount>('files_count', (event) => {
      console.log(event)
      this.countFiles = event.payload;
    });
  }

  setDirectory(type: 'input' | 'watermark' | 'output') {
    open({
      multiple: false,
      directory: true,
    }).then((directory: string | null) => {
      if (directory) {
        if (type === 'input') {
          this.generalForm.controls['pathInput'].setValue(directory);
        } else if (type === 'watermark') {
          this.generalForm.controls['pathWatermark'].setValue(directory);
        } else if (type === 'output') {
          this.generalForm.controls['pathOutput'].setValue(directory);
        }
      }
    });
    // invoke('image_start');
  }

  isInvalid(form: FormGroupDirective, name: string | null = null) {
    if (name) {
      const control = this.generalForm.controls[name];
      return form.submitted && !control.valid;
    }
    return form.submitted && !form.valid;
  }

  submit(form: FormGroupDirective) {
    console.log(form.value)
    if (!this.isInvalid) {
    }
  }
}
