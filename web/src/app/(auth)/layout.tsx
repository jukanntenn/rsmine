import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "Sign In - Rsmine",
  description: "Sign in to your Rsmine account",
};

/**
 * Layout for authentication pages (login, register, etc.)
 * Provides a centered, minimal layout without sidebar
 */
export default function AuthLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-background via-background to-muted/20 p-4">
      <div className="w-full max-w-md space-y-8">
        {/* Logo and title */}
        <div className="text-center space-y-2">
          <h1 className="text-3xl font-bold tracking-tight text-primary">
            Rsmine
          </h1>
          <p className="text-muted-foreground text-sm">
            Project Management & Issue Tracking
          </p>
        </div>

        {/* Auth form container */}
        <div className="bg-card border rounded-xl shadow-lg p-8">{children}</div>

        {/* Footer */}
        <p className="text-center text-xs text-muted-foreground">
          &copy; {new Date().getFullYear()} Rsmine. All rights reserved.
        </p>
      </div>
    </div>
  );
}
