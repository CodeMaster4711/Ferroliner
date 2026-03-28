import type { LayoutServerLoad } from './$types';
import jwt from 'jsonwebtoken';
import { JWT_SECRET } from '$env/static/private';

export const load: LayoutServerLoad = async ({ cookies }) => {
  const token = cookies.get('auth_token');

  if (!token) {
    return { user: null, token: null };
  }

  try {
    const decoded = jwt.verify(token, JWT_SECRET) as {
      sub: string;
      user_id: string;
      username: string;
      two_factor_enabled?: boolean;
      force_password_change?: boolean;
    };

    return {
      user: {
        id: decoded.user_id,
        username: decoded.username,
        two_factor_enabled: decoded.two_factor_enabled ?? false,
        force_password_change: decoded.force_password_change ?? false,
      },
      token,
    };
  } catch {
    // Token invalid or expired — pass it through and let the backend reject it with 401
    // rather than immediately logging the user out on every page refresh
    try {
      const decoded = jwt.decode(token) as {
        user_id?: string;
        username?: string;
      } | null;
      if (decoded?.user_id && decoded?.username) {
        return {
          user: {
            id: decoded.user_id,
            username: decoded.username,
            two_factor_enabled: false,
            force_password_change: false,
          },
          token,
        };
      }
    } catch {}
    cookies.delete('auth_token', { path: '/' });
    return { user: null, token: null };
  }
};
