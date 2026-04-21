"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { toast } from "sonner";
import { LoginForm } from "@/components/features/auth";
import { GuestGuard } from "@/components/features/auth/auth-guard";
import { useAuth } from "@/hooks";
import { DASHBOARD_ROUTES } from "@/lib/constants/routes";

/**
 * Login page component
 */
function LoginPageContent() {
  const router = useRouter();
  const { signIn } = useAuth();
  const [isLoading, setIsLoading] = useState(false);

  const handleLogin = async (credentials: {
    username: string;
    password: string;
  }) => {
    setIsLoading(true);
    try {
      await signIn(credentials);
      toast.success("Welcome back!");
      router.push(DASHBOARD_ROUTES.HOME);
    } catch (error) {
      const message =
        error instanceof Error
          ? error.message
          : "Invalid username or password. Please try again.";
      toast.error(message);
      throw error;
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="space-y-6">
      <div className="text-center space-y-2">
        <h2 className="text-2xl font-semibold tracking-tight">Sign In</h2>
        <p className="text-sm text-muted-foreground">
          Enter your credentials to access your account
        </p>
      </div>

      <LoginForm onSubmit={handleLogin} isLoading={isLoading} />
    </div>
  );
}

/**
 * Login page with GuestGuard to redirect authenticated users
 */
export default function LoginPage() {
  return (
    <GuestGuard>
      <LoginPageContent />
    </GuestGuard>
  );
}
