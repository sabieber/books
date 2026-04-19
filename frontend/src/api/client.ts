import { API_BASE_URL } from '@/api/config'
import { useAuthStore } from '@/stores/auth'
import router from '@/router'

function normalizeHeaders(headers: HeadersInit | undefined): Record<string, string> {
  if (!headers) return {}
  if (headers instanceof Headers) {
    const result: Record<string, string> = {}
    headers.forEach((value, key) => { result[key] = value })
    return result
  }
  if (Array.isArray(headers)) {
    return Object.fromEntries(headers)
  }
  return { ...(headers as Record<string, string>) }
}

export async function apiFetch(path: string, options: RequestInit = {}): Promise<Response> {
  const auth = useAuthStore()

  const isFormData = options.body instanceof FormData
  const headers: Record<string, string> = normalizeHeaders(options.headers)

  if (!isFormData) {
    headers['Content-Type'] = 'application/json'
  }

  if (auth.token) {
    headers['Authorization'] = `Bearer ${auth.token}`
  }

  const response = await fetch(`${API_BASE_URL}${path}`, { ...options, headers })

  if (response.status === 401) {
    auth.logout()
    await router.push('/login')
    throw new Error('Unauthorized')
  }

  return response
}
