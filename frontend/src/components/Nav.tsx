import { NavLink, useNavigate } from 'react-router-dom';
import { LogOut } from 'lucide-react';
import { clearApiKey } from '../api/client';
import PulseLine from './PulseLine';

const links = [
  { to: '/', label: 'Dashboard' },
  { to: '/alerts', label: 'Alerts' },
  { to: '/settings', label: 'Settings' },
];

export default function Nav() {
  const navigate = useNavigate();

  function logout() {
    clearApiKey();
    navigate('/login');
  }

  return (
    <nav className="flex w-56 shrink-0 flex-col border-r border-sidebar-border bg-sidebar">
      <div className="border-b border-sidebar-border px-5 pt-6 pb-5">
        <span className="font-display text-lg tracking-tight text-sidebar-foreground">PULSO</span>
        <PulseLine tiles={4} className="mt-3 h-5 w-full text-primary/70" />
      </div>

      <div className="flex flex-1 flex-col gap-0.5 px-3 py-4">
        {links.map(l => (
          <NavLink
            key={l.to}
            to={l.to}
            end={l.to === '/'}
            className={({ isActive }) =>
              `flex items-center gap-2.5 rounded-md px-3 py-2 text-xs font-medium tracking-widest uppercase transition-colors ${
                isActive
                  ? 'bg-sidebar-accent text-sidebar-accent-foreground'
                  : 'text-muted-foreground hover:bg-sidebar-accent/50 hover:text-sidebar-accent-foreground'
              }`
            }
          >
            {({ isActive }) => (
              <>
                <span
                  className={`h-1.5 w-1.5 shrink-0 rounded-full ${
                    isActive ? 'bg-primary led-live' : 'bg-muted-foreground/30'
                  }`}
                />
                {l.label}
              </>
            )}
          </NavLink>
        ))}
      </div>

      <div className="border-t border-sidebar-border px-3 py-4">
        <button
          onClick={logout}
          className="flex w-full items-center gap-2.5 rounded-md px-3 py-2 text-xs font-medium tracking-widest text-muted-foreground uppercase transition-colors hover:bg-sidebar-accent/50 hover:text-sidebar-accent-foreground"
        >
          <LogOut size={13} />
          Sign out
        </button>
      </div>
    </nav>
  );
}
