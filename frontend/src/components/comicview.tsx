import { useState, useEffect } from 'preact/hooks';
import Snackbar from '@mui/material/Snackbar';

interface ComicViewerProps {
    comicPath: string;
    onClose?: () => void;
}

export function ComicViewer({ comicPath, onClose }: ComicViewerProps) {
    const [comicData, setLibraryData] = useState(null);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState(null);
    const [currentPath, setCurrentPath] = useState('');
    useEffect(() => {
        if (comicPath) {
            const fetchComicData = async () => {
                try {
                    const response = await fetch(`/api/files?path=${encodeURIComponent(comicPath)}`);
                    if (!response.ok) {
                        throw new Error(`HTTP error! Status: ${response.status}`);
                    }

                } catch (err: any) {
                    console.error('ComicViewer: Error fetching comic data:', err);
                    setError(err.message || 'An unknown error occurred while fetching comic data.');

                }
                finally {
                    setLoading(false)
                }


            }
            fetchComicData();


        }



    },[])


    return null;
}