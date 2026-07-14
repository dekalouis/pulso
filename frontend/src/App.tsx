import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import { getApiKey } from './api/client';
import Dashboard from './pages/Dashboard';
import Alerts from './pages/Alerts';
import Settings from './pages/Settings';
import Login from './pages/Login';
import Nav from './components/Nav';

function RequireKey({ children }: { children: React.ReactNode }) {
  return getApiKey() ? <>{children}</> : <Navigate to="/login" replace />;
}

export default function App() {
  return (
  <BrowserRouter>
      <Routes>
        <Route path="/login" element={<Login />} />
        <Route path="/*" element={
          <RequireKey>
            <div className="flex min-h-screen bg-background text-foreground">
              <Nav />
              <main className="flex-1 overflow-y-auto p-8">
                <Routes>
                  <Route path="/" element={<Dashboard />} />
                  <Route path="/alerts" element={<Alerts />} />
                  <Route path="/settings" element={<Settings />} />
                </Routes>
              </main>
            </div>
          </RequireKey>
        } />
      </Routes>
  </BrowserRouter>
  )
}
