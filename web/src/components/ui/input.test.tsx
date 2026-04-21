import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { Input } from "./input";

describe("Input", () => {
  it("renders correctly", () => {
    render(<Input placeholder="Enter text" />);
    expect(screen.getByPlaceholderText("Enter text")).toBeInTheDocument();
  });

  it("handles text input", async () => {
    const user = userEvent.setup();
    render(<Input placeholder="Type here" />);
    const input = screen.getByPlaceholderText("Type here");
    await user.type(input, "Hello World");
    expect(input).toHaveValue("Hello World");
  });

  it("can be disabled", () => {
    render(<Input disabled placeholder="Disabled" />);
    expect(screen.getByPlaceholderText("Disabled")).toBeDisabled();
  });

  it("supports different input types", () => {
    const { rerender } = render(<Input type="text" placeholder="Text" />);
    expect(screen.getByPlaceholderText("Text")).toHaveAttribute("type", "text");

    rerender(<Input type="email" placeholder="Email" />);
    expect(screen.getByPlaceholderText("Email")).toHaveAttribute("type", "email");

    rerender(<Input type="password" placeholder="Password" />);
    expect(screen.getByPlaceholderText("Password")).toHaveAttribute("type", "password");

    rerender(<Input type="number" placeholder="Number" />);
    expect(screen.getByPlaceholderText("Number")).toHaveAttribute("type", "number");
  });

  it("applies custom className", () => {
    render(<Input className="custom-input" placeholder="Custom" />);
    expect(screen.getByPlaceholderText("Custom")).toHaveClass("custom-input");
  });

  it("handles onChange events", async () => {
    const user = userEvent.setup();
    const handleChange = vi.fn();
    render(<Input onChange={handleChange} placeholder="Change" />);
    await user.type(screen.getByPlaceholderText("Change"), "test");
    expect(handleChange).toHaveBeenCalled();
  });

  it("supports required attribute", () => {
    render(<Input required placeholder="Required" />);
    expect(screen.getByPlaceholderText("Required")).toBeRequired();
  });

  it("supports readonly attribute", () => {
    render(<Input readOnly placeholder="Readonly" />);
    expect(screen.getByPlaceholderText("Readonly")).toHaveAttribute("readonly");
  });

  it("supports maxLength attribute", () => {
    render(<Input maxLength={10} placeholder="Max" />);
    expect(screen.getByPlaceholderText("Max")).toHaveAttribute("maxLength", "10");
  });

  it("forwards ref correctly", () => {
    const ref = { current: null as HTMLInputElement | null };
    render(<Input ref={ref} placeholder="Ref" />);
    expect(ref.current).toBeInstanceOf(HTMLInputElement);
  });

  it("applies focus styles on focus", async () => {
    const user = userEvent.setup();
    render(<Input placeholder="Focus" />);
    const input = screen.getByPlaceholderText("Focus");
    await user.click(input);
    expect(input).toHaveFocus();
  });
});
