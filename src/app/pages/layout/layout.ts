import { Component, OnInit } from '@angular/core';
import { filter, pairwise } from 'rxjs';
import { NavigationEnd, Router, RouterOutlet } from '@angular/router';
import { CommonModule } from '@angular/common';
import { MenubarModule } from 'primeng/menubar';
import { MenuItem } from 'primeng/api';


@Component({
  selector: 'app-layout',
  imports: [CommonModule, RouterOutlet, MenubarModule],
  templateUrl: './layout.html',
  styleUrl: './layout.scss'
})
export class Layout implements OnInit {
  needBack: boolean = false;
  items: MenuItem[] | undefined = [
    {
      label: 'Назад',
      icon: 'pi pi-arrow-left',
      command: () => {
        this.router.navigate(['/']);
      }
    }
  ];

  constructor(private router: Router) {

  }

  ngOnInit(): void {
    const urls_temp = this.router.url.split('/');
    for (let i in urls_temp) {
      if (urls_temp[i] !== '') {
        this.needBack = true;
        break;
      }
    }
    this.router.events.pipe(
    ).subscribe((current) => {
      if (current instanceof NavigationEnd) {
        const urls = current.url.split('/');
        let result: boolean = false;
        for (let i = 0; i < urls.length; i++) {
          if (urls[i] !== '') {
            result = true;
            break;
          }
        }
        this.needBack = result;
      }
    });
  }
}
