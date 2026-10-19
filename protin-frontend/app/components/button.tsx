import React from "react";

export interface ButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement> {}

export const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  (props, ref) => (
    <button
      className="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium transition-colors disabled:pointer-events-none disabled:opacity-50 ring-offset-background bg-primary text-background hover:bg-primary-hover py-2 px-3 cursor-pointer disabled:cursor-none"
      ref={ref}
      {...props}
    />
  ),
);
