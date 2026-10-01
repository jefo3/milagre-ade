import { useEffect, useState } from "react";

const chevron = Array.from({ length: 9 }, (_, index) => {
  const row = Math.floor(index / 3);
  const column = index % 3;
  return (column + Math.abs(row - 1)) * 90;
});

const orbitOrder = [0, 1, 2, 5, 8, 7, 6, 3];
const orbit = Array.from({ length: 9 }, (_, index) => {
  const position = orbitOrder.indexOf(index);
  return position === -1 ? null : position * 110;
});

const patterns = {
  Drive: { delays: chevron, dur: 650, round: false },
  Dots: { delays: chevron, dur: 650, round: true },
  Orbit: { delays: orbit, dur: 950, round: false },
} satisfies Record<string, { delays: (number | null)[]; dur: number; round: boolean }>;

function LoaderGrid({ delays, dur, round }: { delays: (number | null)[]; dur: number; round: boolean }) {
  return (
    <span aria-hidden className="grid shrink-0 grid-cols-[repeat(3,4px)] gap-[1.5px]">
      {delays.map((delay, index) => (
        <span
          key={index}
          className={`thinking-pixel size-[4px] bg-ink ${round ? "rounded-full" : "rounded-[1px]"}`}
          style={{
            opacity: delay === null ? 0.07 : 0.15,
            animation: delay === null ? "none" : `pixel-on ${dur}ms ease-in-out ${delay}ms infinite`,
          }}
        />
      ))}
    </span>
  );
}

function useElapsed() {
  const [tenths, setTenths] = useState(0);

  useEffect(() => {
    const timer = window.setInterval(() => setTenths((current) => current + 1), 100);
    return () => window.clearInterval(timer);
  }, []);

  const total = tenths / 10;
  return total < 60 ? `${total.toFixed(1)}s` : `${Math.floor(total / 60)}m ${(total % 60).toFixed(1)}s`;
}

export function ThinkingIndicator({ label, variant = "Drive" }: { label: string; variant?: keyof typeof patterns }) {
  const elapsed = useElapsed();
  const pattern = patterns[variant];

  return (
    <div role="status" aria-live="polite" className="flex w-fit items-center gap-2.5 px-1 py-1">
      <LoaderGrid {...pattern} />
      <span
        className="bg-clip-text text-[13px] font-medium text-transparent"
        style={{
          backgroundImage: "linear-gradient(90deg, var(--ink-3) 35%, var(--ink) 50%, var(--ink-3) 65%)",
          backgroundSize: "200% 100%",
          animation: "shimmer-text 1.4s linear infinite",
        }}
      >
        {label}
      </span>
      <span className="font-mono text-[12px] tabular-nums text-ink-3">{elapsed}</span>
    </div>
  );
}
