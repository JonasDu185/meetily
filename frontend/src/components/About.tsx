import React, { useState, useEffect } from "react";
import { getVersion } from '@tauri-apps/api/app';
import Image from 'next/image';
import AnalyticsConsentSwitch from "./AnalyticsConsentSwitch";


export function About() {
    const [currentVersion, setCurrentVersion] = useState<string>('0.4.0');

    useEffect(() => {
        // Get current version on mount
        getVersion().then(setCurrentVersion).catch(console.error);
    }, []);

    return (
        <div className="p-4 space-y-4 h-[80vh] overflow-y-auto">
            {/* Compact Header */}
            <div className="text-center">
                <div className="mb-3">
                    <Image
                        src="meeting-recorder-icon.png"
                        alt="会议录音图标"
                        width={64}
                        height={64}
                        className="mx-auto"
                    />
                </div>
                <h1 className="text-xl font-bold text-gray-900">会议录音</h1>
                <span className="text-sm text-gray-500"> v{currentVersion}</span>
                <p className="text-medium text-gray-600 mt-1">
                    本地录音、本地转写，会议数据保存在你的 Mac 上。
                </p>
            </div>

            {/* Features Grid - Compact */}
            <div className="space-y-3">
                <h2 className="text-base font-semibold text-gray-800">当前能力</h2>
                <div className="grid grid-cols-2 gap-2">
                    <div className="bg-gray-50 rounded p-3 hover:bg-gray-100 transition-colors">
                        <h3 className="font-bold text-sm text-gray-900 mb-1">本机保存</h3>
                        <p className="text-xs text-gray-600 leading-relaxed">录音、转写和配置默认保存在本机。</p>
                    </div>
                    <div className="bg-gray-50 rounded p-3 hover:bg-gray-100 transition-colors">
                        <h3 className="font-bold text-sm text-gray-900 mb-1">本地转写</h3>
                        <p className="text-xs text-gray-600 leading-relaxed">保留 Community 内置的 Whisper 与 Parakeet 模型管理。</p>
                    </div>
                    <div className="bg-gray-50 rounded p-3 hover:bg-gray-100 transition-colors">
                        <h3 className="font-bold text-sm text-gray-900 mb-1">无需订阅</h3>
                        <p className="text-xs text-gray-600 leading-relaxed">核心录音与转写不依赖按量付费服务。</p>
                    </div>
                    <div className="bg-gray-50 rounded p-3 hover:bg-gray-100 transition-colors">
                        <h3 className="font-bold text-sm text-gray-900 mb-1">平台无关</h3>
                        <p className="text-xs text-gray-600 leading-relaxed">面向系统声音和麦克风，不绑定具体会议软件。</p>
                    </div>
                </div>
            </div>

            {/* Footer - Compact */}
            <div className="pt-2 border-t border-gray-200 text-center">
                <p className="text-xs text-gray-400">
                    基于 Meetily Community Edition（MIT License）修改
                </p>
            </div>
            <AnalyticsConsentSwitch />

        </div>

    )
}
