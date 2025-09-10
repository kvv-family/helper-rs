import { Component, signal } from '@angular/core';
import { ActivatedRoute, NavigationEnd, Router, RouterOutlet } from '@angular/router';
import { MenubarModule } from 'primeng/menubar';
import { ButtonModule } from 'primeng/button';
import { MenuItem } from 'primeng/api';
import { filter, pairwise } from 'rxjs';


@Component({
  selector: 'app-root',
  imports: [RouterOutlet, MenubarModule, ButtonModule],
  templateUrl: './app.html',
})
export class App {
  protected readonly title = signal('image-helper');
  items: MenuItem[] | undefined = [
    {
      label: 'Назад',
      icon: 'pi pi-arrow-left',
      command: () => {
        this.router.navigate(['/']);
      }
    }
  ];
  needBack: boolean = false;

  constructor(private route: ActivatedRoute, private router: Router) {
    const urls_temp = this.router.url.split('/');
    this.needBack = !(urls_temp.length > 0);
    this.router.events.pipe(
      filter((event) => event instanceof NavigationEnd),
      pairwise()
    ).subscribe(([previous, current]: [NavigationEnd, NavigationEnd]) => {
      const urls = current.url.split('/');
      let result: boolean = false;
      for (let i = 0; i < urls.length; i++) {
        if (urls[i] !== '') {
          result = true;
          break;
        }
      }
      this.needBack = result;
    });
  }
}
