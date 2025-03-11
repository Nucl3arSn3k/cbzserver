import { h } from 'preact';
import Button from '@mui/material/Button';
import Typography from '@mui/material/Typography';

export function Home() {
  return (
    <div>
      <Typography variant="h4">MaterialUI test in preact</Typography>
      <Button variant="contained" color="primary">
        Click Me
      </Button>
    </div>
  );
}