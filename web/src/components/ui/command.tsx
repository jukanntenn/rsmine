"use client";

import * as React from "react";
import { Search } from "lucide-react";
import { cn } from "@/lib/utils";

// Command Context
interface CommandContextValue {
  value: string;
  onValueChange: (value: string) => void;
  searchValue: string;
  onSearchChange: (value: string) => void;
  filteredCount: number;
  setFilteredCount: (count: number) => void;
}

const CommandContext = React.createContext<CommandContextValue | undefined>(undefined);

function useCommand() {
  const context = React.useContext(CommandContext);
  if (!context) {
    throw new Error("useCommand must be used within a Command component");
  }
  return context;
}

// Command Root
interface CommandProps extends React.HTMLAttributes<HTMLDivElement> {
  value?: string;
  onValueChange?: (value: string) => void;
  defaultValue?: string;
}

const Command = React.forwardRef<HTMLDivElement, CommandProps>(
  ({ className, value, onValueChange, defaultValue, children, ...props }, ref) => {
    const [internalValue, setInternalValue] = React.useState(defaultValue || "");
    const [searchValue, setSearchValue] = React.useState("");
    const [filteredCount, setFilteredCount] = React.useState(0);

    const currentValue = value !== undefined ? value : internalValue;
    const handleValueChange = onValueChange || setInternalValue;

    return (
      <CommandContext.Provider
        value={{
          value: currentValue,
          onValueChange: handleValueChange,
          searchValue,
          onSearchChange: setSearchValue,
          filteredCount,
          setFilteredCount,
        }}
      >
        <div
          ref={ref}
          className={cn(
            "flex h-full w-full flex-col overflow-hidden rounded-md bg-popover text-popover-foreground",
            className
          )}
          {...props}
        >
          {children}
        </div>
      </CommandContext.Provider>
    );
  }
);
Command.displayName = "Command";

// Command Input
interface CommandInputProps extends React.InputHTMLAttributes<HTMLInputElement> {
  containerClassName?: string;
}

const CommandInput = React.forwardRef<HTMLInputElement, CommandInputProps>(
  ({ className, containerClassName, ...props }, ref) => {
    const { onSearchChange } = useCommand();

    return (
      <div className={cn("flex items-center border-b px-3", containerClassName)}>
        <Search className="mr-2 h-4 w-4 shrink-0 opacity-50" />
        <input
          ref={ref}
          className={cn(
            "flex h-11 w-full rounded-md bg-transparent py-3 text-sm outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50",
            className
          )}
          onChange={(e) => onSearchChange(e.target.value)}
          {...props}
        />
      </div>
    );
  }
);
CommandInput.displayName = "CommandInput";

// Command List
type CommandListProps = React.HTMLAttributes<HTMLDivElement>;

const CommandList = React.forwardRef<HTMLDivElement, CommandListProps>(
  ({ className, children, ...props }, ref) => {
    const { setFilteredCount } = useCommand();
    const childrenArray = React.Children.toArray(children);
    let itemCount = 0;

    // Count CommandItems
    childrenArray.forEach((child) => {
      if (React.isValidElement(child)) {
        if (child.type === CommandItem) {
          itemCount++;
        } else if (child.type === CommandGroup) {
          const props = child.props as { children?: React.ReactNode };
          const groupChildren = React.Children.toArray(props.children);
          groupChildren.forEach((groupChild) => {
            if (React.isValidElement(groupChild) && groupChild.type === CommandItem) {
              itemCount++;
            }
          });
        }
      }
    });

    React.useEffect(() => {
      setFilteredCount(itemCount);
    }, [itemCount, setFilteredCount]);

    return (
      <div
        ref={ref}
        className={cn("max-h-[300px] overflow-y-auto overflow-x-hidden", className)}
        {...props}
      >
        {children}
      </div>
    );
  }
);
CommandList.displayName = "CommandList";

// Command Empty
type CommandEmptyProps = React.HTMLAttributes<HTMLDivElement>;

const CommandEmpty = React.forwardRef<HTMLDivElement, CommandEmptyProps>(
  ({ className, ...props }, ref) => {
    const { filteredCount } = useCommand();

    if (filteredCount > 0) return null;

    return (
      <div
        ref={ref}
        className={cn("py-6 text-center text-sm", className)}
        {...props}
      />
    );
  }
);
CommandEmpty.displayName = "CommandEmpty";

// Command Group
interface CommandGroupProps extends React.HTMLAttributes<HTMLDivElement> {
  heading?: string;
}

const CommandGroup = React.forwardRef<HTMLDivElement, CommandGroupProps>(
  ({ className, heading, children, ...props }, ref) => {
    const { searchValue, setFilteredCount } = useCommand();
    
    // Filter children based on search
    const filteredChildren = React.Children.toArray(children).filter((child) => {
      if (React.isValidElement(child) && child.type === CommandItem) {
        const props = child.props as { children?: React.ReactNode };
        const text = props.children?.toString().toLowerCase() || "";
        return text.includes(searchValue.toLowerCase());
      }
      return true;
    });

    // Update filtered count
    React.useEffect(() => {
      setFilteredCount(filteredChildren.length);
    }, [filteredChildren.length, setFilteredCount]);

    if (filteredChildren.length === 0) return null;

    return (
      <div
        ref={ref}
        className={cn(
          "overflow-hidden p-1 text-foreground",
          className
        )}
        {...props}
      >
        {heading && (
          <div className="px-2 py-1.5 text-xs font-medium text-muted-foreground">
            {heading}
          </div>
        )}
        {filteredChildren}
      </div>
    );
  }
);
CommandGroup.displayName = "CommandGroup";

// Command Item
interface CommandItemProps extends Omit<React.HTMLAttributes<HTMLDivElement>, "onSelect"> {
  onSelect?: (value: string) => void;
  disabled?: boolean;
}

const CommandItem = React.forwardRef<HTMLDivElement, CommandItemProps>(
  ({ className, children, onSelect, disabled, ...props }, ref) => {
    const { value, onValueChange, searchValue } = useCommand();
    const [isFocused, setIsFocused] = React.useState(false);

    // Get text content for selection
    const textContent = typeof children === "string" ? children : "";
    const isSelected = value === textContent;

    // Check if item matches search
    const matchesSearch = textContent.toLowerCase().includes(searchValue.toLowerCase());
    
    if (searchValue && !matchesSearch) return null;

    const handleClick = () => {
      if (!disabled) {
        onValueChange(textContent);
        onSelect?.(textContent);
      }
    };

    return (
      <div
        ref={ref}
        role="option"
        aria-selected={isSelected}
        aria-disabled={disabled}
        data-selected={isSelected}
        data-disabled={disabled}
        className={cn(
          "relative flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none",
          "hover:bg-accent hover:text-accent-foreground",
          "data-[selected=true]:bg-accent data-[selected=true]:text-accent-foreground",
          "data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50",
          isFocused && "bg-accent text-accent-foreground",
          className
        )}
        onClick={handleClick}
        onFocus={() => setIsFocused(true)}
        onBlur={() => setIsFocused(false)}
        tabIndex={disabled ? -1 : 0}
        {...props}
      >
        {children}
      </div>
    );
  }
);
CommandItem.displayName = "CommandItem";

// Command Separator
type CommandSeparatorProps = React.HTMLAttributes<HTMLDivElement>;

const CommandSeparator = React.forwardRef<HTMLDivElement, CommandSeparatorProps>(
  ({ className, ...props }, ref) => {
    return (
      <div
        ref={ref}
        className={cn("-mx-1 h-px bg-border", className)}
        {...props}
      />
    );
  }
);
CommandSeparator.displayName = "CommandSeparator";

// Command Shortcut
type CommandShortcutProps = React.HTMLAttributes<HTMLSpanElement>;

const CommandShortcut = React.forwardRef<HTMLSpanElement, CommandShortcutProps>(
  ({ className, ...props }, ref) => {
    return (
      <span
        ref={ref}
        className={cn(
          "ml-auto text-xs tracking-widest text-muted-foreground",
          className
        )}
        {...props}
      />
    );
  }
);
CommandShortcut.displayName = "CommandShortcut";

// Command Dialog (for modal command palette)
interface CommandDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  children: React.ReactNode;
  className?: string;
}

function CommandDialog({ open, onOpenChange, children, className }: CommandDialogProps) {
  if (!open) return null;

  return (
    <div className="fixed inset-0 z-50">
      <div
        className="fixed inset-0 bg-black/80"
        onClick={() => onOpenChange(false)}
      />
      <div className="fixed left-[50%] top-[50%] z-50 w-full max-w-lg translate-x-[-50%] translate-y-[-50%]">
        <Command className={cn("shadow-lg", className)}>{children}</Command>
      </div>
    </div>
  );
}

export {
  Command,
  CommandInput,
  CommandList,
  CommandEmpty,
  CommandGroup,
  CommandItem,
  CommandSeparator,
  CommandShortcut,
  CommandDialog,
};
