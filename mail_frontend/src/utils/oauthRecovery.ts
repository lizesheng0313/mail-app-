const AUTO_RECOVERY_LIMIT_MARKER = '__AUTO_RECOVERY_LIMIT__::'

export const isAutoRecoveryLimitError = (message: any) =>
  String(message || '').includes(AUTO_RECOVERY_LIMIT_MARKER)

export const normalizeOAuthRecoveryErrorMessage = (message: any) =>
  String(message || '').replace(AUTO_RECOVERY_LIMIT_MARKER, '').trim()

