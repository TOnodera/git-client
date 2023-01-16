import React, { ReactNode, useEffect, useRef, useState } from 'react';

interface Props {
  width: number;
  height: number;
}

// Pathを描画する
function drawPath(
  ctx: CanvasRenderingContext2D,
  color: string,
  startPosition: CanvasPosition,
  endPosition: CanvasPosition
) {
  ctx.beginPath();
  ctx.strokeStyle = color;
  ctx.moveTo(startPosition.x, startPosition.y);
  ctx.lineTo(startPosition.x, endPosition.y);
  ctx.stroke();
}

// 点を描画する
// コメントを表示する

function Canvas(props: Props) {
  const canvas = useRef<HTMLCanvasElement>(null);
  useEffect(() => {
    const ctx = canvas.current?.getContext('2d');
    ctx?.fillRect(0, 0, props.width, props.height);

    //
  }, []);

  return (
    <>
      <canvas width={props.width} height={props.height} ref={canvas}></canvas>
    </>
  );
}

export default Canvas;
