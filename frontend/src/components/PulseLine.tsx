const TILE = "M0,30 L18,30 L24,14 L30,30 L40,30 L46,52 L52,8 L58,30 L68,30 L74,22 L80,30 L100,30";

type Props = {
  className?: string;
  tiles?: number;
};

export default function PulseLine({ className, tiles = 6 }: Props) {
  return (
    <svg
      viewBox={`0 0 ${100 * (tiles - 1)} 60`}
      preserveAspectRatio="none"
      className={className}
      aria-hidden="true"
    >
      <g className="pulse-scroll">
        {Array.from({ length: tiles }, (_, i) => (
          <path
            key={i}
            d={TILE}
            transform={`translate(${i * 100},0)`}
            fill="none"
            stroke="currentColor"
            strokeWidth="2.5"
            strokeLinecap="round"
            strokeLinejoin="round"
          />
        ))}
      </g>
    </svg>
  );
}
