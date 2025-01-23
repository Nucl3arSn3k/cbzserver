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
      
      if ('webkitGetAsEntry' in files[0]) {
        const entry = (files[0] as any).webkitGetAsEntry();
        if (entry) {
          setFilePath(entry.fullPath);
          return;
        }
      }
      setFilePath(files[0].webkitRelativePath || files[0].name);
      setError('');
    } catch (err) {
      console.error('Error accessing file system:', err);
      setError('Unable to access file system');
    }
  };

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
    </div>
  );
};

export default Settings;