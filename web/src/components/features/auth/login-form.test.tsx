import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { LoginForm } from "./login-form";

// Setup userEvent
const user = userEvent.setup();

describe("LoginForm", () => {
  const mockOnSubmit = vi.fn();

  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe("rendering", () => {
    it("should render username field placeholder", () => {
      render(<LoginForm onSubmit={mockOnSubmit} />);

      expect(screen.getByPlaceholderText(/enter your username/i)).toBeInTheDocument();
    });

    it("should render password field placeholder", () => {
      render(<LoginForm onSubmit={mockOnSubmit} />);

      expect(screen.getByPlaceholderText(/enter your password/i)).toBeInTheDocument();
    });

    it("should render submit button", () => {
      render(<LoginForm onSubmit={mockOnSubmit} />);

      expect(screen.getByRole("button", { name: /sign in/i })).toBeInTheDocument();
    });

    it("should render username label", () => {
      render(<LoginForm onSubmit={mockOnSubmit} />);

      expect(screen.getByText(/username/i)).toBeInTheDocument();
    });

    it("should render password label", () => {
      render(<LoginForm onSubmit={mockOnSubmit} />);

      expect(screen.getByText(/password/i)).toBeInTheDocument();
    });
  });

  describe("form validation", () => {
    it("should show error when username is empty", async () => {
      render(<LoginForm onSubmit={mockOnSubmit} />);

      const submitButton = screen.getByRole("button", { name: /sign in/i });
      await user.click(submitButton);

      await waitFor(() => {
        expect(screen.getByText(/username is required/i)).toBeInTheDocument();
      });
    });

    it("should show error when password is empty", async () => {
      render(<LoginForm onSubmit={mockOnSubmit} />);

      const usernameInput = screen.getByPlaceholderText(/enter your username/i);
      await user.type(usernameInput, "testuser");

      const submitButton = screen.getByRole("button", { name: /sign in/i });
      await user.click(submitButton);

      await waitFor(() => {
        expect(screen.getByText(/password is required/i)).toBeInTheDocument();
      });
    });

    it("should show both errors when both fields are empty", async () => {
      render(<LoginForm onSubmit={mockOnSubmit} />);

      const submitButton = screen.getByRole("button", { name: /sign in/i });
      await user.click(submitButton);

      await waitFor(() => {
        expect(screen.getByText(/username is required/i)).toBeInTheDocument();
        expect(screen.getByText(/password is required/i)).toBeInTheDocument();
      });
    });

    it("should not call onSubmit when form is invalid", async () => {
      render(<LoginForm onSubmit={mockOnSubmit} />);

      const submitButton = screen.getByRole("button", { name: /sign in/i });
      await user.click(submitButton);

      await waitFor(() => {
        expect(mockOnSubmit).not.toHaveBeenCalled();
      });
    });
  });

  describe("form submission", () => {
    it("should call onSubmit with credentials when valid", async () => {
      mockOnSubmit.mockResolvedValueOnce(undefined);
      render(<LoginForm onSubmit={mockOnSubmit} />);

      const usernameInput = screen.getByPlaceholderText(/enter your username/i);
      const passwordInput = screen.getByPlaceholderText(/enter your password/i);

      await user.type(usernameInput, "testuser");
      await user.type(passwordInput, "password123");

      const submitButton = screen.getByRole("button", { name: /sign in/i });
      await user.click(submitButton);

      await waitFor(() => {
        expect(mockOnSubmit).toHaveBeenCalledWith({
          username: "testuser",
          password: "password123",
        });
      });
    });

    it("should show error message when submission fails", async () => {
      mockOnSubmit.mockRejectedValueOnce(new Error("Invalid credentials"));
      render(<LoginForm onSubmit={mockOnSubmit} />);

      const usernameInput = screen.getByPlaceholderText(/enter your username/i);
      const passwordInput = screen.getByPlaceholderText(/enter your password/i);

      await user.type(usernameInput, "wronguser");
      await user.type(passwordInput, "wrongpass");

      const submitButton = screen.getByRole("button", { name: /sign in/i });
      await user.click(submitButton);

      await waitFor(() => {
        expect(screen.getByText(/invalid credentials/i)).toBeInTheDocument();
      });
    });

    it("should show error when submission fails with non-Error rejection", async () => {
      mockOnSubmit.mockRejectedValueOnce("Network error");
      render(<LoginForm onSubmit={mockOnSubmit} />);

      const usernameInput = screen.getByPlaceholderText(/enter your username/i);
      const passwordInput = screen.getByPlaceholderText(/enter your password/i);

      await user.type(usernameInput, "user");
      await user.type(passwordInput, "pass");

      const submitButton = screen.getByRole("button", { name: /sign in/i });
      await user.click(submitButton);

      await waitFor(() => {
        expect(screen.getByText(/invalid username or password/i)).toBeInTheDocument();
      });
    });
  });

  describe("loading state", () => {
    it("should disable username input when loading", () => {
      render(<LoginForm onSubmit={mockOnSubmit} isLoading={true} />);

      const usernameInput = screen.getByPlaceholderText(/enter your username/i);
      expect(usernameInput).toBeDisabled();
    });

    it("should disable password input when loading", () => {
      render(<LoginForm onSubmit={mockOnSubmit} isLoading={true} />);

      const passwordInput = screen.getByPlaceholderText(/enter your password/i);
      expect(passwordInput).toBeDisabled();
    });

    it("should disable submit button when loading", () => {
      render(<LoginForm onSubmit={mockOnSubmit} isLoading={true} />);

      // When loading, the button text changes to "Signing in..."
      const button = screen.getByRole("button", { name: /signing in/i });
      expect(button).toBeDisabled();
    });

    it("should show loading text when loading", () => {
      render(<LoginForm onSubmit={mockOnSubmit} isLoading={true} />);

      expect(screen.getByText(/signing in/i)).toBeInTheDocument();
    });

    it("should show spinner when loading", () => {
      render(<LoginForm onSubmit={mockOnSubmit} isLoading={true} />);

      // Loader2 icon should be present (has animate-spin class)
      const spinner = document.querySelector(".animate-spin");
      expect(spinner).toBeInTheDocument();
    });
  });

  describe("accessibility", () => {
    it("should be submittable via Enter key", async () => {
      mockOnSubmit.mockResolvedValueOnce(undefined);
      render(<LoginForm onSubmit={mockOnSubmit} />);

      const usernameInput = screen.getByPlaceholderText(/enter your username/i);
      const passwordInput = screen.getByPlaceholderText(/enter your password/i);

      await user.type(usernameInput, "testuser");
      await user.type(passwordInput, "password123{enter}");

      await waitFor(() => {
        expect(mockOnSubmit).toHaveBeenCalledWith({
          username: "testuser",
          password: "password123",
        });
      });
    });
  });

  describe("error display", () => {
    it("should display error alert when there is an error", async () => {
      mockOnSubmit.mockRejectedValueOnce(new Error("First error"));

      render(<LoginForm onSubmit={mockOnSubmit} />);

      await user.type(screen.getByPlaceholderText(/enter your username/i), "user1");
      await user.type(screen.getByPlaceholderText(/enter your password/i), "pass1");
      await user.click(screen.getByRole("button", { name: /sign in/i }));

      await waitFor(() => {
        expect(screen.getByRole("alert")).toBeInTheDocument();
        expect(screen.getByText(/first error/i)).toBeInTheDocument();
      });
    });
  });
});
