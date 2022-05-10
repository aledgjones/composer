import { FC } from "react";
import { RenderInstruction } from "../../render/instructions";
import { useInView } from "react-intersection-observer";
import { Canvas } from "./canvas";

interface Props {
  instructions: RenderInstruction<any>[];
  x: number;
  y: number;
  width: number;
  height: number;
}

export const Cell: FC<Props> = ({ instructions, x, y, width, height }) => {
  const { inView, ref } = useInView();

  return (
    <div
      style={{
        position: "absolute",
        top: y,
        left: x,
        width,
        height,
      }}
      ref={ref}
    >
      {inView && (
        <Canvas
          instructions={instructions}
          x={x}
          y={y}
          width={width}
          height={height}
        />
      )}
    </div>
  );
};
