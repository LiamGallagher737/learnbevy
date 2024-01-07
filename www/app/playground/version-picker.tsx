"use client";

import * as React from "react";
import { Check, ChevronsUpDown } from "lucide-react";

import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button";
import {
  Command,
  CommandGroup,
  CommandItem,
} from "@/components/ui/command";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";
import { VERSIONS, Version } from "@/lib/versions";

const versions = VERSIONS.map((version) => {
  return {
    label: `v${version}`,
    value: version,
  };
});

export function VersionPicker(props: {
  initialValue: Version;
  onChange: (version: Version) => void;
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
          {value
            ? versions.find((version) => version.value === value)?.label
            : "Select version..."}
          <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-[120px] p-0">
        <Command>
          <CommandGroup>
            {versions.map((version) => (
              <CommandItem
                key={version.value}
                value={version.value}
                onSelect={(currentValue) => {
                  setValue(currentValue as Version);
                  setOpen(false);
                }}
              >
                <Check
                  className={cn(
                    "mr-2 h-4 w-4",
                    value === version.value ? "opacity-100" : "opacity-0"
                  )}
                />
                {version.label}
              </CommandItem>
            ))}
          </CommandGroup>
        </Command>
      </PopoverContent>
    </Popover>
  );
}
