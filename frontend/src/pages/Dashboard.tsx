import React from 'react';
import {
  Container,
  Grid,
  Card,
  CardContent,
  Typography,
  Box,
  LinearProgress,
  Avatar,
  Chip,
  Button,
} from '@mui/material';
import {
  TrendingUp,
  School,
  Assignment,
  EmojiEvents,
  CalendarToday,
} from '@mui/icons-material';
import { useSelector } from 'react-redux';
import { RootState } from '../store';

const Dashboard: React.FC = () => {
  const { user } = useSelector((state: RootState) => state.auth);

  const stats = [
    { title: 'Active Courses', value: '5', icon: <School />, color: '#667eea' },
    { title: 'Completed Lessons', value: '24', icon: <Assignment />, color: '#f093fb' },
    { title: 'Study Streak', value: '12 days', icon: <TrendingUp />, color: '#4facfe' },
    { title: 'Achievements', value: '8', icon: <EmojiEvents />, color: '#43e97b' },
  ];

  const recentCourses = [
    { name: 'Japanese Beginner', progress: 65, color: '#667eea' },
    { name: 'English Grammar', progress: 80, color: '#f093fb' },
    { name: 'Sinhala Writing', progress: 45, color: '#4facfe' },
  ];

  const upcomingSessions = [
    { title: 'Japanese Conversation Practice', time: 'Today, 3:00 PM', teacher: 'Ms. Tanaka' },
    { title: 'English Writing Workshop', time: 'Tomorrow, 10:00 AM', teacher: 'Mr. Smith' },
  ];

  return (
    <Container maxWidth="lg">
      <Box sx={{ mb: 4 }}>
        <Typography variant="h4" gutterBottom sx={{ fontWeight: 600 }}>
          Welcome back, {user?.firstName || 'Student'}! 👋
        </Typography>
        <Typography variant="body1" color="text.secondary">
          Continue your learning journey today
        </Typography>
      </Box>

      {/* Stats Cards */}
      <Grid container spacing={3} sx={{ mb: 4 }}>
        {stats.map((stat, index) => (
          <Grid item xs={12} sm={6} md={3} key={index}>
            <Card
              sx={{
                background: `linear-gradient(135deg, ${stat.color}20 0%, ${stat.color}40 100%)`,
                border: `1px solid ${stat.color}30`,
              }}
            >
              <CardContent>
                <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
                  <Box>
                    <Typography variant="h4" sx={{ fontWeight: 600, color: stat.color }}>
                      {stat.value}
                    </Typography>
                    <Typography variant="body2" color="text.secondary">
                      {stat.title}
                    </Typography>
                  </Box>
                  <Avatar sx={{ bgcolor: stat.color, width: 56, height: 56 }}>
                    {stat.icon}
                  </Avatar>
                </Box>
              </CardContent>
            </Card>
          </Grid>
        ))}
      </Grid>

      <Grid container spacing={3}>
        {/* Recent Courses */}
        <Grid item xs={12} md={8}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom sx={{ fontWeight: 600 }}>
                Continue Learning
              </Typography>
              {recentCourses.map((course, index) => (
                <Box key={index} sx={{ mb: 3 }}>
                  <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 1 }}>
                    <Typography variant="body1">{course.name}</Typography>
                    <Typography variant="body2" color="text.secondary">
                      {course.progress}%
                    </Typography>
                  </Box>
                  <LinearProgress
                    variant="determinate"
                    value={course.progress}
                    sx={{
                      height: 8,
                      borderRadius: 4,
                      backgroundColor: `${course.color}20`,
                      '& .MuiLinearProgress-bar': {
                        backgroundColor: course.color,
                      },
                    }}
                  />
                </Box>
              ))}
            </CardContent>
          </Card>

          {/* Upcoming Sessions */}
          <Card sx={{ mt: 3 }}>
            <CardContent>
              <Typography variant="h6" gutterBottom sx={{ fontWeight: 600 }}>
                Upcoming Live Sessions
              </Typography>
              {upcomingSessions.map((session, index) => (
                <Box
                  key={index}
                  sx={{
                    p: 2,
                    mb: 2,
                    borderRadius: 2,
                    backgroundColor: '#f5f5f5',
                    display: 'flex',
                    justifyContent: 'space-between',
                    alignItems: 'center',
                  }}
                >
                  <Box>
                    <Typography variant="body1" sx={{ fontWeight: 500 }}>
                      {session.title}
                    </Typography>
                    <Typography variant="body2" color="text.secondary">
                      <CalendarToday sx={{ fontSize: 14, mr: 0.5, verticalAlign: 'middle' }} />
                      {session.time} • {session.teacher}
                    </Typography>
                  </Box>
                  <Button variant="contained" size="small">
                    Join
                  </Button>
                </Box>
              ))}
            </CardContent>
          </Card>
        </Grid>

        {/* Quick Actions & Achievements */}
        <Grid item xs={12} md={4}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom sx={{ fontWeight: 600 }}>
                Quick Actions
              </Typography>
              <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1 }}>
                <Button variant="outlined" fullWidth startIcon={<Assignment />}>
                  Submit Homework
                </Button>
                <Button variant="outlined" fullWidth startIcon={<School />}>
                  Browse Courses
                </Button>
                <Button variant="outlined" fullWidth startIcon={<CalendarToday />}>
                  Schedule Session
                </Button>
              </Box>
            </CardContent>
          </Card>

          <Card sx={{ mt: 3 }}>
            <CardContent>
              <Typography variant="h6" gutterBottom sx={{ fontWeight: 600 }}>
                Recent Achievements
              </Typography>
              <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
                <Chip
                  icon={<EmojiEvents />}
                  label="7-Day Streak"
                  color="primary"
                  variant="outlined"
                />
                <Chip
                  icon={<EmojiEvents />}
                  label="Perfect Score"
                  color="secondary"
                  variant="outlined"
                />
                <Chip
                  icon={<EmojiEvents />}
                  label="Fast Learner"
                  color="success"
                  variant="outlined"
                />
              </Box>
            </CardContent>
          </Card>
        </Grid>
      </Grid>
    </Container>
  );
};

export default Dashboard;
