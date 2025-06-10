import React, { StrictMode, Suspense } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import { createBrowserRouter, RouterProvider } from 'react-router-dom';

const LoginPage = React.lazy(() => import('./pages/Login'));

const router = createBrowserRouter([
  {
    path: "/",
    element: <Suspense fallback={<div>Loading...</div>}><LoginPage /></Suspense>,
  },
]);

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <RouterProvider router={router} />
  </StrictMode>,
)
