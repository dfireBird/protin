import React from "react";

export interface ButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement> {
    buttonColor?: "accent" | "secondary" | "primary"
}

export const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  (props, ref) => {
    let color;
    if (props.buttonColor == "secondary") {
      color = "bg-secondary hover:bg-secondary-hover";
    } else if (props.buttonColor == "accent") {
      color = "bg-accent hover:bg-accent-hover";
    } else {
      color = "bg-primary hover:bg-primary-hover";
    }

    return <button
      className={`inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium transition-colors disabled:pointer-events-none disabled:opacity-50 ring-offset-background ${color} text-background py-2 px-3 cursor-pointer disabled:cursor-none`}
      ref={ref}
      {...props}
    />
  }
);
