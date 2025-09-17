import { Routes } from '@angular/router';
import { Layout } from './pages/layout/layout';
import { Navigations } from './pages/navigations/navigations';
import { Watermarks } from './pages/watermarks/watermarks';

export const routes: Routes = [
    {
        path: '',
        component: Layout,
        children: [
            {
                path: '',
                component: Navigations
            },
            {
                path: 'watermarks',
                component: Watermarks
            }
        ]
    }

];
