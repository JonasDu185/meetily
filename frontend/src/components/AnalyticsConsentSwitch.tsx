export default function AnalyticsConsentSwitch() {
  return (
    <div>
      <h3 className="text-base font-semibold text-gray-800 mb-2">使用情况统计</h3>
      <div className="p-3 bg-gray-50 rounded-lg border border-gray-200">
        <p className="font-semibold text-gray-800">已永久关闭</p>
        <p className="text-sm text-gray-600 mt-1">
          此个人版本不会向 Meetily 或 PostHog 发送使用情况统计。
        </p>
      </div>
    </div>
  );
}
