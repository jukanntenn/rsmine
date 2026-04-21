"use client";

import * as React from "react";
import { cva, type VariantProps } from "class-variance-authority";
import { cn } from "@/lib/utils";

const separatorVariants = cva("shrink-0 bg-border", {
  variants: {
    orientation: {
      horizontal: "h-[1px] w-full",
      vertical: "h-full w-[1px]",
    },
  },
  defaultVariants: {
    orientation: "horizontal",
  },
});

export interface SeparatorProps
  extends React.HTMLAttributes<HTMLDivElement>,
    VariantProps<typeof separatorVariants> {
  decorative?: boolean;
}

const Separator = React.forwardRef<HTMLDivElement, SeparatorProps>(
  ({ className, orientation = "horizontal", decorative = true, ...props }, ref) => {
    const ariaOrientation = !decorative ? orientation as "horizontal" | "vertical" : undefined;
    
    return (
      <div
        ref={ref}
        role={decorative ? "none" : "separator"}
        aria-orientation={ariaOrientation}
        className={cn(separatorVariants({ orientation }), className)}
        {...props}
      />
    );
  }
);
Separator.displayName = "Separator";

export { Separator };