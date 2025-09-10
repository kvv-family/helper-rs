import { Component } from '@angular/core';
import { Router } from '@angular/router';
import { CardModule } from 'primeng/card';
import { ButtonModule } from 'primeng/button';


@Component({
  selector: 'app-navigations',
  imports: [CardModule, ButtonModule],
  templateUrl: './navigations.html',
  styleUrl: './navigations.scss'
})
export class Navigations {
  constructor(private router: Router) { }
  navigateToWatermarks() {
    this.router.navigate(['/watermarks']);
  }
}
