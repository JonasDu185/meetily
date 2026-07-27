'use client';

import React, { useEffect, ReactNode, createContext } from 'react';
import { load } from '@tauri-apps/plugin-store';
import { invoke } from '@tauri-apps/api/core';

const ANALYTICS_DEFAULT_OFF_MIGRATION_KEY = 'analyticsDefaultOffMigrationV1';

interface AnalyticsProviderProps {
  children: ReactNode;
}

interface AnalyticsContextType {
  isAnalyticsOptedIn: boolean;
  setIsAnalyticsOptedIn: (optedIn: boolean) => void;
}

export const AnalyticsContext = createContext<AnalyticsContextType>({
  isAnalyticsOptedIn: false,
  setIsAnalyticsOptedIn: () => { },
});

export default function AnalyticsProvider({ children }: AnalyticsProviderProps) {
  useEffect(() => {
    const disableAnalytics = async () => {
      const store = await load('analytics.json', {
        autoSave: false,
        defaults: {
          analyticsOptedIn: false
        }
      });

      // 覆盖官方版本可能遗留的选择，确保个人版始终关闭遥测。
      await store.set('analyticsOptedIn', false);
      await store.set(ANALYTICS_DEFAULT_OFF_MIGRATION_KEY, true);
      await store.save();
      await invoke('disable_analytics');
    };

    disableAnalytics().catch((error) => {
      console.error('Failed to enforce disabled analytics state:', error);
    });
  }, []);

  return (
    <AnalyticsContext.Provider
      value={{ isAnalyticsOptedIn: false, setIsAnalyticsOptedIn: () => { } }}
    >
      {children}
    </AnalyticsContext.Provider>
  );
}
