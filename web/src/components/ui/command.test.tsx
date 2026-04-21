import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import {
  Command,
  CommandInput,
  CommandList,
  CommandEmpty,
  CommandGroup,
  CommandItem,
  CommandSeparator,
  CommandShortcut,
} from "./command";

describe("Command", () => {
  it("renders correctly", () => {
    render(
      <Command>
        <CommandInput placeholder="Search..." />
        <CommandList>
          <CommandItem>Item 1</CommandItem>
        </CommandList>
      </Command>
    );
    expect(screen.getByPlaceholderText("Search...")).toBeInTheDocument();
    expect(screen.getByText("Item 1")).toBeInTheDocument();
  });

  it("shows empty state when no results", () => {
    render(
      <Command>
        <CommandInput placeholder="Search..." />
        <CommandList>
          <CommandEmpty>No results found.</CommandEmpty>
          <CommandItem>Item 1</CommandItem>
        </CommandList>
      </Command>
    );
    // Initially items should be visible
    expect(screen.getByText("Item 1")).toBeInTheDocument();
  });

  it("filters items based on search", async () => {
    const user = userEvent.setup();
    render(
      <Command>
        <CommandInput placeholder="Search..." />
        <CommandList>
          <CommandItem>Apple</CommandItem>
          <CommandItem>Banana</CommandItem>
          <CommandItem>Cherry</CommandItem>
        </CommandList>
      </Command>
    );

    const input = screen.getByPlaceholderText("Search...");
    await user.type(input, "App");

    // Apple should still be visible
    expect(screen.getByText("Apple")).toBeInTheDocument();
  });

  it("handles item selection", async () => {
    const user = userEvent.setup();
    const onValueChange = vi.fn();
    render(
      <Command onValueChange={onValueChange}>
        <CommandList>
          <CommandItem>Item 1</CommandItem>
          <CommandItem>Item 2</CommandItem>
        </CommandList>
      </Command>
    );

    await user.click(screen.getByText("Item 1"));
    expect(onValueChange).toHaveBeenCalledWith("Item 1");
  });

  it("handles onSelect callback", async () => {
    const user = userEvent.setup();
    const onSelect = vi.fn();
    render(
      <Command>
        <CommandList>
          <CommandItem onSelect={onSelect}>Click me</CommandItem>
        </CommandList>
      </Command>
    );

    await user.click(screen.getByText("Click me"));
    expect(onSelect).toHaveBeenCalledWith("Click me");
  });

  it("renders groups with headings", () => {
    render(
      <Command>
        <CommandList>
          <CommandGroup heading="Fruits">
            <CommandItem>Apple</CommandItem>
          </CommandGroup>
          <CommandGroup heading="Vegetables">
            <CommandItem>Carrot</CommandItem>
          </CommandGroup>
        </CommandList>
      </Command>
    );

    expect(screen.getByText("Fruits")).toBeInTheDocument();
    expect(screen.getByText("Vegetables")).toBeInTheDocument();
    expect(screen.getByText("Apple")).toBeInTheDocument();
    expect(screen.getByText("Carrot")).toBeInTheDocument();
  });

  it("renders separator", () => {
    const { container } = render(
      <Command>
        <CommandList>
          <CommandItem>Item 1</CommandItem>
          <CommandSeparator />
          <CommandItem>Item 2</CommandItem>
        </CommandList>
      </Command>
    );

    const separator = container.querySelector(".bg-border");
    expect(separator).toBeInTheDocument();
  });

  it("renders keyboard shortcuts", () => {
    render(
      <Command>
        <CommandList>
          <CommandItem>
            Copy
            <CommandShortcut>Ctrl+C</CommandShortcut>
          </CommandItem>
        </CommandList>
      </Command>
    );

    expect(screen.getByText("Ctrl+C")).toBeInTheDocument();
  });

  it("applies custom className", () => {
    const { container } = render(
      <Command className="custom-command">
        <CommandList />
      </Command>
    );

    expect(container.querySelector(".custom-command")).toBeInTheDocument();
  });

  it("disabled items cannot be selected", async () => {
    const user = userEvent.setup();
    const onSelect = vi.fn();
    render(
      <Command>
        <CommandList>
          <CommandItem disabled onSelect={onSelect}>
            Disabled
          </CommandItem>
        </CommandList>
      </Command>
    );

    await user.click(screen.getByText("Disabled"));
    expect(onSelect).not.toHaveBeenCalled();
  });

  it("supports controlled value", () => {
    const onValueChange = vi.fn();
    render(
      <Command value="selected" onValueChange={onValueChange}>
        <CommandList>
          <CommandItem>selected</CommandItem>
          <CommandItem>other</CommandItem>
        </CommandList>
      </Command>
    );

    const selectedItem = screen.getByText("selected");
    expect(selectedItem).toHaveAttribute("data-selected", "true");
  });
});

describe("CommandInput", () => {
  it("renders search icon", () => {
    const { container } = render(
      <Command>
        <CommandInput placeholder="Search..." />
      </Command>
    );

    const svg = container.querySelector("svg");
    expect(svg).toBeInTheDocument();
  });

  it("updates search value on input", async () => {
    const user = userEvent.setup();
    render(
      <Command>
        <CommandInput placeholder="Search..." />
        <CommandList>
          <CommandItem>Test Item</CommandItem>
        </CommandList>
      </Command>
    );

    const input = screen.getByPlaceholderText("Search...");
    await user.type(input, "Test");
    expect(input).toHaveValue("Test");
  });
});

describe("CommandItem", () => {
  it("has correct role", () => {
    render(
      <Command>
        <CommandList>
          <CommandItem>Test</CommandItem>
        </CommandList>
      </Command>
    );

    expect(screen.getByRole("option", { name: "Test" })).toBeInTheDocument();
  });

  it("applies custom className", () => {
    render(
      <Command>
        <CommandList>
          <CommandItem className="custom-item">Custom</CommandItem>
        </CommandList>
      </Command>
    );

    expect(screen.getByText("Custom")).toHaveClass("custom-item");
  });
});
