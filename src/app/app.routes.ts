import { Routes } from '@angular/router';

export const routes: Routes = [
    {
        path: '',
        loadComponent: () => import('./pages/layout/layout').then(m => m.Layout),
        children: [
            {
                path: '',
                loadComponent: () => import('./pages/navigations/navigations').then(m => m.Navigations)
            },
            {
                path: 'watermarks',
                loadComponent: () => import('./pages/watermarks/watermarks').then(m => m.Watermarks)
            }
        ]
    }

];
