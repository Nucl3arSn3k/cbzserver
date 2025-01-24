import { createSignal } from 'solid-js';
import type { Component, JSX } from 'solid-js';

// Define custom input props
type DirectoryInputAttributes = JSX.InputHTMLAttributes<HTMLInputElement> & {
    webkitdirectory?: '';
    directory?: '';
    mozdirectory?: '';
};

const Settings: Component = () => {
    const [filePath, setFilePath] = createSignal('');
    const [error, setError] = createSignal('');

    const handleFolderSelect: JSX.EventHandlerUnion<HTMLInputElement, Event> = async (e) => {
        try {
            const files = e.currentTarget.files;
            if (!files?.length) return;
            /*
            if ('webkitGetAsEntry' in files[0]) {
              const entry = (files[0] as any).webkitGetAsEntry();
              if (entry) {
                setFilePath(entry.fullPath);
                return;
              }
            }
            */
            const path = files[0].webkitRelativePath;
            const folder = path.split('/')[0];
            setFilePath(folder);
            setError('');
        } catch (err) {
            console.error('Error accessing file system:', err);
            setError('Unable to access file system');
        }
    };

    const handleSubmit = async (e: Event) => { //Modify to handle real submitdata
        e.preventDefault()
        const response = await fetch('/api/fsub', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            filepath: filePath(),
          })
        })
        if (response.ok) {
          // Redirect or handle successful login
        }
      }

    return (
        <div>
            <input
                {...({
                    type: "file",
                    webkitdirectory: "",
                    directory: "",
                    mozdirectory: "",
                } as DirectoryInputAttributes)}
                onChange={handleFolderSelect}
            />
            <div>Selected Path: {filePath()}</div>
            {error() && <div class="error">{error()}</div>}
            <button onClick={handleSubmit}>Submit</button>
        </div>
    );
};

export default Settings;