import React from 'react';
import {
  Drawer,
  List,
  ListItem,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Toolbar,
  Divider,
} from '@mui/material';
import {
  Dashboard as DashboardIcon,
  School,
  MenuBook,
  Psychology,
  Payment,
  Chat,
  VideoCall,
  Assessment,
  Person,
} from '@mui/icons-material';
import { useNavigate, useLocation } from 'react-router-dom';

interface SidebarProps {
  open: boolean;
  onClose: () => void;
}

const menuItems = [
  { text: 'Dashboard', icon: <DashboardIcon />, path: '/dashboard' },
  { text: 'My Courses', icon: <School />, path: '/courses' },
  { text: 'Lessons', icon: <MenuBook />, path: '/lessons' },
  { text: 'AI Practice', icon: <Psychology />, path: '/practice' },
  { text: 'Live Sessions', icon: <VideoCall />, path: '/sessions' },
  { text: 'Messages', icon: <Chat />, path: '/messages' },
  { text: 'Payments', icon: <Payment />, path: '/payments' },
  { text: 'Progress', icon: <Assessment />, path: '/progress' },
  { text: 'Profile', icon: <Person />, path: '/profile' },
];

const Sidebar: React.FC<SidebarProps> = ({ open, onClose }) => {
  const navigate = useNavigate();
  const location = useLocation();

  const handleNavigation = (path: string) => {
    navigate(path);
  };

  const drawer = (
    <>
      <Toolbar />
      <Divider />
      <List>
        {menuItems.map((item) => (
          <ListItem key={item.text} disablePadding>
            <ListItemButton
              selected={location.pathname === item.path}
              onClick={() => handleNavigation(item.path)}
              sx={{
                '&.Mui-selected': {
                  backgroundColor: 'rgba(103, 126, 234, 0.1)',
                  borderRight: '4px solid #667eea',
                },
                '&:hover': {
                  backgroundColor: 'rgba(103, 126, 234, 0.05)',
                },
              }}
            >
              <ListItemIcon sx={{ color: location.pathname === item.path ? '#667eea' : 'inherit' }}>
                {item.icon}
              </ListItemIcon>
              <ListItemText primary={item.text} />
            </ListItemButton>
          </ListItem>
        ))}
      </List>
    </>
  );

  return (
    <Drawer
      variant="persistent"
      open={open}
      sx={{
        width: 240,
        flexShrink: 0,
        '& .MuiDrawer-paper': {
          width: 240,
          boxSizing: 'border-box',
        },
      }}
    >
      {drawer}
    </Drawer>
  );
};

export default Sidebar;
