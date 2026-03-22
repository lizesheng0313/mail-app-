export const normalizeSmtpEmail = (email?: string | null) =>
  String(email || '').trim().toLowerCase()

export const canDesktopSmtpSend = (account: any) => {
  const status = String(account?.status || '').trim().toLowerCase()
  const authType = String(account?.auth_type || 'password').trim().toLowerCase()
  const password = String(account?.smtp_password || account?.password || '').trim()
  const smtpHost = String(account?.smtp_host || '').trim()
  const smtpPort = Number(account?.smtp_port || 0)

  return status === 'active' && authType !== 'oauth2' && !!password && !!smtpHost && smtpPort > 0
}

export const buildDesktopSendableSmtpAccountMap = (accounts: any[]) => {
  return new Map(
    (accounts || [])
      .filter((account) => canDesktopSmtpSend(account))
      .map((account) => [normalizeSmtpEmail(account?.email), account])
  )
}
