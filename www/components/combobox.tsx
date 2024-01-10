"use client";

import * as React from "react";
import { Check, ChevronsUpDown } from "lucide-react";

import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button";
import { Command, CommandGroup, CommandItem } from "@/components/ui/command";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";

export function Combobox<T extends string>(props: {
  initialValue: T;
  values: T[];
  onChange: (version: T) => void;
}) {
  const [open, setOpen] = React.useState(false);
  const [value, setValue] = React.useState(props.initialValue);

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button
          variant="outline"
          role="combobox"
          aria-expanded={open}
          className="w-[120px] justify-between"
        >
          {props.values.find((version) => version === value) ?? "Select"}
          <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-[120px] p-0">
        <Command>
          <CommandGroup>
            {props.values.map((version) => (
              <CommandItem
                key={version}
                value={version}
                onSelect={(currentValue) => {
                  setValue(currentValue as T);
                  props.onChange(currentValue as T);
                  setOpen(false);
                }}
              >
                <Check
                  className={cn(
                    "mr-2 h-4 w-4",
                    value === version ? "opacity-100" : "opacity-0"
                  )}
                />
                {version}
              </CommandItem>
            ))}
          </CommandGroup>
        </Command>
      </PopoverContent>
    </Popover>
  );
}
