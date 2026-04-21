import { getRequestConfig } from "next-intl/server";
import type { RequestConfig } from "next-intl/server";

export default getRequestConfig(async (): Promise<RequestConfig> => {
  // Default to English, can be overridden by user preference or cookie
  const locale = "en";

  return {
    locale,
    messages: (await import(`./messages/${locale}.json`)).default,
  };
});
