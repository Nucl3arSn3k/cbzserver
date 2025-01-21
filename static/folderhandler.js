// folderHandler.js
export function requestFileSystem() {
    return new Promise((resolve, reject) => {
        window.requestFileSystem = window.requestFileSystem || window.webkitRequestFileSystem;
        window.requestFileSystem(
            window.TEMPORARY,
            1024*1024,
            resolve,
            reject
        );
    });
}

export async function getfolder(e) {
    try {
        const fs = await requestFileSystem();
        const files = e.target.files;
        
        if (!files.length) return;

        // Get FileSystemEntry using webkitGetAsEntry()
        const entry = files[0].webkitGetAsEntry();
        
       
        // Use the fullPath property
        const fullPath = entry.fullPath;
        document.getElementById('folderPath').value = fullPath;
        console.log('Full path:', fullPath);
       
    } catch (error) {
        console.error('Error accessing file system:', error);
        // Fallback to basic path
        const basicPath = e.target.files[0].webkitRelativePath;
        document.getElementById('folderPath').value = basicPath;
    }
}