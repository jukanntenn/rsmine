import type { FullConfig } from "@playwright/test";

export default async function globalTeardown(config: FullConfig) {
  void config;
}
