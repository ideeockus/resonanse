import React, { useEffect, useState } from 'react';
import { BrowserRouter, Navigate, Route, Routes } from 'react-router-dom';
import Main from './components/main/main';
import Good from './components/good/good';
import { Toaster } from 'react-hot-toast';


function App() {

  useEffect(() => {
    window.Telegram.WebApp.expand()
  }, [])
  return (
    <BrowserRouter>
      <div className="app">
        <Toaster />
        <Routes>
          <Route path="/" element={<Main />} />
          <Route path="/good/:id" element={<Good />} />
        </Routes>
      </div>
    </BrowserRouter>
  );
}

export default App;
