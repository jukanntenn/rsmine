import { describe, it, expect, beforeEach } from "vitest";
import { useUIStore } from "./ui-store";

describe("ui-store", () => {
  beforeEach(() => {
    // Reset store state before each test
    useUIStore.setState({
      commandPaletteOpen: false,
    });
  });

  describe("initial state", () => {
    it("should have commandPaletteOpen as false by default", () => {
      const state = useUIStore.getState();

      expect(state.commandPaletteOpen).toBe(false);
    });
  });

  describe("toggleCommandPalette", () => {
    it("should toggle from false to true", () => {
      useUIStore.getState().toggleCommandPalette();

      expect(useUIStore.getState().commandPaletteOpen).toBe(true);
    });

    it("should toggle from true to false", () => {
      useUIStore.getState().openCommandPalette();

      useUIStore.getState().toggleCommandPalette();

      expect(useUIStore.getState().commandPaletteOpen).toBe(false);
    });

    it("should toggle multiple times correctly", () => {
      useUIStore.getState().toggleCommandPalette();
      expect(useUIStore.getState().commandPaletteOpen).toBe(true);

      useUIStore.getState().toggleCommandPalette();
      expect(useUIStore.getState().commandPaletteOpen).toBe(false);

      useUIStore.getState().toggleCommandPalette();
      expect(useUIStore.getState().commandPaletteOpen).toBe(true);
    });
  });

  describe("openCommandPalette", () => {
    it("should set commandPaletteOpen to true", () => {
      useUIStore.getState().openCommandPalette();

      expect(useUIStore.getState().commandPaletteOpen).toBe(true);
    });

    it("should remain true if already open", () => {
      useUIStore.getState().openCommandPalette();
      useUIStore.getState().openCommandPalette();

      expect(useUIStore.getState().commandPaletteOpen).toBe(true);
    });
  });

  describe("closeCommandPalette", () => {
    it("should set commandPaletteOpen to false", () => {
      useUIStore.getState().openCommandPalette();

      useUIStore.getState().closeCommandPalette();

      expect(useUIStore.getState().commandPaletteOpen).toBe(false);
    });

    it("should remain false if already closed", () => {
      useUIStore.getState().closeCommandPalette();

      expect(useUIStore.getState().commandPaletteOpen).toBe(false);
    });
  });
});