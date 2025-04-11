import { h, Fragment } from 'preact';
import { useState, useEffect } from 'preact/hooks';
import Button from '@mui/material/Button';
import Typography from '@mui/material/Typography';
import Breadcrumbs from '@mui/material/Breadcrumbs';
import Link from '@mui/material/Link';
import Grid from '@mui/material/Grid'; // Using the standard Grid import in v7
import Card from '@mui/material/Card';
import CardContent from '@mui/material/CardContent';
import CardMedia from '@mui/material/CardMedia';
import FolderIcon from '@mui/icons-material/Folder';
import NavigateNextIcon from '@mui/icons-material/NavigateNext';
import Box from '@mui/material/Box';
import CircularProgress from '@mui/material/CircularProgress';
import Alert from '@mui/material/Alert';

export function ComicLibrary() {
  const [libraryData, setLibraryData] = useState(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);
  const [currentPath, setCurrentPath] = useState('');

  const fetchLibraryData = async (path = '') => {
    setLoading(true);
    setError(null);
    try {
      const response = await fetch(`/api/library${path ? `?path=${encodeURIComponent(path)}` : ''}`);
      
      if (!response.ok) {
        throw new Error(`HTTP error! Status: ${response.status}`);
      }
      
      const data = await response.json();
      setLibraryData(data);
      setCurrentPath(data.id);
    } catch (err) {
      setError(err.message);
      console.error('Error fetching library data:', err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    // Initial load
    fetchLibraryData();
  }, []);

  // Generate breadcrumbs from path
  const generateBreadcrumbs = (path) => {
    if (!path) return [];
    
    const parts = path.split('\\');
    const breadcrumbs = parts.map((part, index) => {
      const currentPath = parts.slice(0, index + 1).join('\\');
      return {
        name: part,
        path: currentPath
      };
    });
    
    return breadcrumbs;
  };

  const handleBreadcrumbClick = (path, event) => {
    event.preventDefault();
    fetchLibraryData(`?path=${encodeURIComponent(path)}`);
  };

  const handleFolderClick = (path) => {
    fetchLibraryData(`?path=${encodeURIComponent(path)}`);
  };

  const handleComicClick = (path) => {
    // Send request to /api/files endpoint with the comic's filepath
    try {
      fetch(`/api/files?path=${encodeURIComponent(path)}`)
        .then(response => {
          if (!response.ok) {
            throw new Error(`HTTP error! Status: ${response.status}`);
          }
          return response.json();
        })
        .then(data => {
          // Handle the response data as needed
          console.log('Comic file data:', data);
          // You could navigate to a comic viewer or open the comic in a new view
        })
        .catch(err => {
          setError(err.message);
          console.error('Error fetching comic file:', err);
        });
    } catch (err) {
      setError(err.message);
      console.error('Error handling comic click:', err);
    }
  };

  // Render breadcrumbs
  const renderBreadcrumbs = () => {
    const breadcrumbs = generateBreadcrumbs(currentPath);
    
    return (
      <Breadcrumbs separator={<NavigateNextIcon fontSize="small" />}>
        {breadcrumbs.map((crumb, index) => {
          const isLast = index === breadcrumbs.length - 1;
          
          return isLast ? (
            <Typography key={crumb.path} color="text.primary">
              {crumb.name}
            </Typography>
          ) : (
            <Link 
              key={crumb.path} 
              href="#" 
              onClick={(e) => handleBreadcrumbClick(crumb.path, e)}
              underline="hover"
              color="inherit"
            >
              {crumb.name}
            </Link>
          );
        })}
      </Breadcrumbs>
    );
  };

  // Render comics and folders
  const renderContent = () => {
    if (!libraryData) return null;

    return (
      <Grid container spacing={3} sx={{ width: '100%' }}>
        {/* Render folders (children) first */}
        {libraryData.children && libraryData.children.map((folderPath) => {
          const folderName = folderPath.split('\\').pop();
          return (
            <Grid size={{ xs: 6, sm: 4, md: 3, lg: 2 }} key={folderPath}>
              <Card 
                onClick={() => handleFolderClick(folderPath)}
                sx={{ 
                  cursor: 'pointer',
                  height: '100%',
                  display: 'flex',
                  flexDirection: 'column',
                  '&:hover': { boxShadow: 6 }
                }}
              >
                <Box sx={{ display: 'flex', justifyContent: 'center', pt: 2, pb: 1 }}>
                  <FolderIcon sx={{ fontSize: 80, color: 'primary.main' }} />
                </Box>
                <CardContent sx={{ flexGrow: 1 }}>
                  <Typography align="center" noWrap title={folderName}>
                    {folderName}
                  </Typography>
                </CardContent>
              </Card>
            </Grid>
          );
        })}
        
        {/* Render comic files */}
        {libraryData.contents && libraryData.contents.map((item) => {
          // Skip the directory itself which has dirornot = 1
          if (item.dirornot === 1) return null;
          
          return (
            <Grid size={{ xs: 6, sm: 4, md: 3, lg: 2 }} key={item.filepath}>
              <Card sx={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
                {item.cover_path ? (
                  <CardMedia
                    onClick={() => handleComicClick(item.filepath)}
                    component="img"
                    height="200"
                    image={`/api/cover?path=${encodeURIComponent(item.cover_path)}`}
                    alt={item.name}
                    sx={{ 
                      objectFit: 'contain', 
                      p: 1,
                      cursor: 'pointer',
                      '&:hover': { opacity: 0.9 }
                    }}
                  />
                ) : (
                  <Box 
                    sx={{ 
                      height: 200, 
                      display: 'flex', 
                      alignItems: 'center', 
                      justifyContent: 'center',
                      cursor: 'pointer',
                      '&:hover': { backgroundColor: 'rgba(0,0,0,0.04)' }
                    }}
                    onClick={() => handleComicClick(item.filepath)}
                  >
                    <Typography color="text.secondary">No Cover</Typography>
                  </Box>
                )}
                <CardContent 
                  sx={{ 
                    flexGrow: 1,
                    cursor: 'pointer',
                    '&:hover': { backgroundColor: 'rgba(0,0,0,0.04)' }
                  }}
                  onClick={() => handleComicClick(item.filepath)}
                >
                  <Typography variant="body2" noWrap title={item.name}>
                    {item.name.replace(/\.(cbr|cbz)$/i, '')}
                  </Typography>
                </CardContent>
              </Card>
            </Grid>
          );
        })}
      </Grid>
    );
  };

  return (
    <Box sx={{ p: 3 }}>
      <Typography variant="h4" gutterBottom>Comic Library</Typography>
      
      {/* Breadcrumbs */}
      <Box sx={{ mb: 3 }}>
        {currentPath && renderBreadcrumbs()}
      </Box>
      
      {/* Action buttons */}
      <Box sx={{ mb: 3 }}>
        <Button 
          variant="contained" 
          color="primary" 
          onClick={() => fetchLibraryData()}
          disabled={loading}
        >
          Rescan Library
        </Button>
      </Box>
      
      {/* Error message */}
      {error && (
        <Alert severity="error" sx={{ mb: 3 }}>
          {error}
        </Alert>
      )}
      
      {/* Loading indicator */}
      {loading ? (
        <Box sx={{ display: 'flex', justifyContent: 'center', p: 4 }}>
          <CircularProgress />
        </Box>
      ) : (
        renderContent()
      )}
    </Box>
  );
}