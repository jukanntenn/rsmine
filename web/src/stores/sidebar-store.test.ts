import { describe, it, expect, beforeEach } from "vitest";
import { useSidebarStore } from "./sidebar-store";

describe("sidebar-store", () => {
  beforeEach(() => {
    // Reset store state before each test
    useSidebarStore.setState({
      collapsed: false,
    });
  });

  describe("initial state", () => {
    it("should have collapsed as false by default", () => {
      const state = useSidebarStore.getState();

      expect(state.collapsed).toBe(false);
    });
  });

  describe("toggle", () => {
    it("should toggle collapsed state from false to true", () => {
      expect(useSidebarStore.getState().collapsed).toBe(false);

      useSidebarStore.getState().toggle();

      expect(useSidebarStore.getState().collapsed).toBe(true);
    });

    it("should toggle collapsed state from true to false", () => {
      useSidebarStore.getState().setCollapsed(true);
      expect(useSidebarStore.getState().collapsed).toBe(true);

      useSidebarStore.getState().toggle();

      expect(useSidebarStore.getState().collapsed).toBe(false);
    });

    it("should toggle multiple times correctly", () => {
      useSidebarStore.getState().toggle();
      expect(useSidebarStore.getState().collapsed).toBe(true);

      useSidebarStore.getState().toggle();
      expect(useSidebarStore.getState().collapsed).toBe(false);

      useSidebarStore.getState().toggle();
      expect(useSidebarStore.getState().collapsed).toBe(true);
    });
  });

  describe("setCollapsed", () => {
    it("should set collapsed to true", () => {
      useSidebarStore.getState().setCollapsed(true);

      expect(useSidebarStore.getState().collapsed).toBe(true);
    });

    it("should set collapsed to false", () => {
      useSidebarStore.getState().setCollapsed(true);
      useSidebarStore.getState().setCollapsed(false);

      expect(useSidebarStore.getState().collapsed).toBe(false);
    });

    it("should not change state when setting same value", () => {
      useSidebarStore.getState().setCollapsed(false);
      expect(useSidebarStore.getState().collapsed).toBe(false);

      useSidebarStore.getState().setCollapsed(false);

      expect(useSidebarStore.getState().collapsed).toBe(false);
    });
  });
});