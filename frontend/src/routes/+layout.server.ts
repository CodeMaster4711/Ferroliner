import type { LayoutServerLoad } from './$types';
import jwt from 'jsonwebtoken';

export const load: LayoutServerLoad = async ({ cookies }) => {
  const token = cookies.get('auth_token');

  if (!token) {
    return { user: null, token: null };
  }

  const decoded = jwt.decode(token) as {
    user_id?: string;
    username?: string;
    two_factor_enabled?: boolean;
    force_password_change?: boolean;
    exp?: number;
  } | null;

  if (!decoded?.user_id || !decoded?.username) {
    cookies.delete('auth_token', { path: '/' });
    return { user: null, token: null };
  }

  const isExpired = decoded.exp ? decoded.exp < Math.floor(Date.now() / 1000) : false;
  if (isExpired) {
    cookies.delete('auth_token', { path: '/' });
    return { user: null, token: null };
  }

  return {
    user: {
      id: decoded.user_id,
      username: decoded.username,
      two_factor_enabled: decoded.two_factor_enabled ?? false,
      force_password_change: decoded.force_password_change ?? false,
    },
    token,
  };
};
