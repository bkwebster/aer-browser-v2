import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

function App() {
  const [url, setUrl] = useState('https://example.com');
  const [isLoading, setIsLoading] = useState(false);
  const [hasWebview, setHasWebview] = useState(false);

  const navigateToUrl = async () => {
    setIsLoading(true);
    try {
      // Ensure URL has protocol
      let navigateUrl = url;
      if (!url.startsWith('http://') && !url.startsWith('https://')) {
        // If it looks like a search query, use Google
        if (!url.includes('.') || url.includes(' ')) {
          navigateUrl = `https://www.google.com/search?q=${encodeURIComponent(url)}`;
        } else {
          navigateUrl = `https://${url}`;
        }
      }
      
      await invoke('navigate_to_url', { url: navigateUrl });
      setUrl(navigateUrl);
      setHasWebview(true);
    } catch (error) {
      console.error('Navigation failed:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const goBack = async () => {
    try {
      await invoke('go_back');
    } catch (error) {
      console.error('Failed to go back:', error);
    }
  };

  const goForward = async () => {
    try {
      await invoke('go_forward');
    } catch (error) {
      console.error('Failed to go forward:', error);
    }
  };

  const refresh = async () => {
    try {
      await invoke('refresh_page');
    } catch (error) {
      console.error('Failed to refresh:', error);
    }
  };

  return (
    <div className="h-screen flex flex-col bg-gray-100">
      {/* Browser Chrome */}
      <div className="bg-white border-b border-gray-200 p-3 flex items-center space-x-3">
        {/* Navigation buttons */}
        <div className="flex space-x-2">
          <button 
            onClick={goBack}
            className="p-2 rounded-md hover:bg-gray-100 text-gray-600 disabled:text-gray-300"
            disabled={!hasWebview}
            title="Go back"
          >
            ←
          </button>
          <button 
            onClick={goForward}
            className="p-2 rounded-md hover:bg-gray-100 text-gray-600 disabled:text-gray-300"
            disabled={!hasWebview}
            title="Go forward"
          >
            →
          </button>
          <button 
            onClick={refresh}
            className="p-2 rounded-md hover:bg-gray-100 text-gray-600 disabled:text-gray-300"
            disabled={!hasWebview || isLoading}
            title="Refresh"
          >
            {isLoading ? '⏳' : '↻'}
          </button>
        </div>

        {/* Address bar */}
        <div className="flex-1">
          <input
            type="text"
            value={url}
            onChange={(e) => setUrl(e.target.value)}
            onKeyPress={(e) => {
              if (e.key === 'Enter') {
                navigateToUrl();
              }
            }}
            className="w-full px-4 py-2 rounded-lg border border-gray-300 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            placeholder="Enter URL or search..."
          />
        </div>

        {/* Menu button */}
        <button className="p-2 rounded-md hover:bg-gray-100 text-gray-600">
          ⋮
        </button>
      </div>

      {/* Content area - This is where the webview will be positioned */}
      <div className="flex-1 bg-white relative">
        {!hasWebview && (
          <div className="h-full flex items-center justify-center text-gray-500">
            <div className="text-center">
              <h1 className="text-4xl font-bold text-gray-700 mb-4">
                🌬️ Aer Browser
              </h1>
              <p className="text-lg">
                Initializing webview...
              </p>
              <p className="text-sm text-gray-400 mt-4">
                Phase 2: WebView Loading... ⏳
              </p>
            </div>
          </div>
        )}
        
        {hasWebview && isLoading && (
          <div className="absolute top-0 left-0 right-0 h-1 bg-blue-500 animate-pulse" />
        )}
        
        {/* The webview will be rendered here by Tauri */}
      </div>
    </div>
  );
}

export default App;
