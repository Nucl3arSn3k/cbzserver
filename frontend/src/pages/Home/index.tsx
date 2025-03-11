import { h } from 'preact';
import Button from '@mui/material/Button';
import Typography from '@mui/material/Typography';

export function Home() {

	const requestScan = async () => {
		try {
		  const response = await fetch('/api/library', {
			method: 'GET',
			headers: {
			  'Accept': 'application/json'
			}
		  });
		  
		  if (!response.ok) {
			throw new Error(`HTTP error! Status: ${response.status}`);
		  }
		  
		  
		  const data = await response.json();
		  return data;
		} catch (error) {
		  console.error('Error fetching library data:', error);
		  throw error; // Re-throw to allow calling code to handle it
		}
	  };


  return (
    <div>
      <Typography variant="h4">MaterialUI test in preact</Typography>
      <Button variant="contained" color="primary" onClick={requestScan}>
        Click Me
      </Button>
    </div>
  );
}