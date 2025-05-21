import { useState, useEffect } from 'preact/hooks';
interface ComicViewerProps {
  comicPath: string;
  onClose?: () => void;
}

export function ComicViewer({comicPath,onClose}: ComicViewerProps) {
    const [comicData, setLibraryData] = useState(null);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState(null);
    const [currentPath, setCurrentPath] = useState('');

    const fetchComicData = async (path = '') => {
        try {
            const response = await fetch(`/api/files?path=${encodeURIComponent(path)}`);
            if (!response.ok) {
                throw new Error(`HTTP error! Status: ${response.status}`);
            }

        } catch {


        }
        finally {
            setLoading(false)
        }


    }

    return null;
}