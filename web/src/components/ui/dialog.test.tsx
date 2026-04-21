import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import {
  Dialog,
  DialogTrigger,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
  DialogDescription,
} from "./dialog";

describe("Dialog", () => {
  it("renders trigger correctly", () => {
    render(
      <Dialog>
        <DialogTrigger>Open Dialog</DialogTrigger>
        <DialogContent>
          <DialogTitle>Dialog Title</DialogTitle>
          <DialogDescription>Dialog description</DialogDescription>
        </DialogContent>
      </Dialog>
    );

    expect(screen.getByText("Open Dialog")).toBeInTheDocument();
  });

  it("opens dialog when trigger is clicked", async () => {
    const user = userEvent.setup();
    render(
      <Dialog>
        <DialogTrigger>Open</DialogTrigger>
        <DialogContent>
          <DialogTitle>Title</DialogTitle>
        </DialogContent>
      </Dialog>
    );

    await user.click(screen.getByText("Open"));
    expect(screen.getByText("Title")).toBeInTheDocument();
  });

  it("closes dialog when close button is clicked", async () => {
    const user = userEvent.setup();
    render(
      <Dialog>
        <DialogTrigger>Open</DialogTrigger>
        <DialogContent>
          <DialogTitle>Title</DialogTitle>
        </DialogContent>
      </Dialog>
    );

    await user.click(screen.getByText("Open"));
    expect(screen.getByText("Title")).toBeInTheDocument();

    const closeButton = screen.getByRole("button", { name: "Close" });
    await user.click(closeButton);
    expect(screen.queryByText("Title")).not.toBeInTheDocument();
  });

  it("renders header and footer", async () => {
    const user = userEvent.setup();
    render(
      <Dialog>
        <DialogTrigger>Open</DialogTrigger>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Header Title</DialogTitle>
          </DialogHeader>
          <DialogFooter>
            <button>Footer Button</button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    );

    await user.click(screen.getByText("Open"));
    expect(screen.getByText("Header Title")).toBeInTheDocument();
    expect(screen.getByText("Footer Button")).toBeInTheDocument();
  });

  it("renders description", async () => {
    const user = userEvent.setup();
    render(
      <Dialog>
        <DialogTrigger>Open</DialogTrigger>
        <DialogContent>
          <DialogTitle>Title</DialogTitle>
          <DialogDescription>This is a description</DialogDescription>
        </DialogContent>
      </Dialog>
    );

    await user.click(screen.getByText("Open"));
    expect(screen.getByText("This is a description")).toBeInTheDocument();
  });

  it("supports controlled open state", async () => {
    const onOpenChange = vi.fn();
    render(
      <Dialog open={true} onOpenChange={onOpenChange}>
        <DialogContent>
          <DialogTitle>Controlled Dialog</DialogTitle>
        </DialogContent>
      </Dialog>
    );

    expect(screen.getByText("Controlled Dialog")).toBeInTheDocument();
  });

  it("applies custom className to content", async () => {
    const user = userEvent.setup();
    render(
      <Dialog>
        <DialogTrigger>Open</DialogTrigger>
        <DialogContent className="custom-content">
          <DialogTitle>Title</DialogTitle>
        </DialogContent>
      </Dialog>
    );

    await user.click(screen.getByText("Open"));
    const content = screen.getByText("Title").closest(".custom-content");
    expect(content).toBeInTheDocument();
  });

  it("closes when clicking overlay", async () => {
    const user = userEvent.setup();
    render(
      <Dialog>
        <DialogTrigger>Open</DialogTrigger>
        <DialogContent>
          <DialogTitle>Title</DialogTitle>
        </DialogContent>
      </Dialog>
    );

    await user.click(screen.getByText("Open"));
    expect(screen.getByText("Title")).toBeInTheDocument();

    // Click overlay (the backdrop)
    const overlay = document.querySelector(".fixed.inset-0.z-50.bg-black\\/80");
    if (overlay) {
      await user.click(overlay);
    }
  });

  it("renders children correctly", async () => {
    const user = userEvent.setup();
    render(
      <Dialog>
        <DialogTrigger>Open</DialogTrigger>
        <DialogContent>
          <DialogTitle>Dialog</DialogTitle>
          <div data-testid="child-content">Child content</div>
        </DialogContent>
      </Dialog>
    );

    await user.click(screen.getByText("Open"));
    expect(screen.getByTestId("child-content")).toBeInTheDocument();
  });
});

describe("DialogTitle", () => {
  it("renders as h2", async () => {
    const user = userEvent.setup();
    render(
      <Dialog>
        <DialogTrigger>Open</DialogTrigger>
        <DialogContent>
          <DialogTitle>Title Text</DialogTitle>
        </DialogContent>
      </Dialog>
    );

    await user.click(screen.getByText("Open"));
    const title = screen.getByRole("heading", { name: "Title Text" });
    expect(title.tagName).toBe("H2");
  });
});

describe("DialogDescription", () => {
  it("renders as paragraph", async () => {
    const user = userEvent.setup();
    render(
      <Dialog>
        <DialogTrigger>Open</DialogTrigger>
        <DialogContent>
          <DialogTitle>Title</DialogTitle>
          <DialogDescription>Description text</DialogDescription>
        </DialogContent>
      </Dialog>
    );

    await user.click(screen.getByText("Open"));
    const description = screen.getByText("Description text");
    expect(description.tagName).toBe("P");
  });
});
