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

function drawDot(ctx: CanvasRenderingContext2D, color: string, x: number, y: number, r: number) {
  ctx.beginPath();
  ctx.arc(x, y, r, 0, Math.PI * 2, true);
  ctx.fillStyle = color;
  ctx.fill();
}

const testData = [
  {
    start: {
      x: 100,
      y: 10
    },
    end: { x: 100, y: 20 }
  },
  // {
  //   start: {
  //     x: 100,
  //     y: 20
  //   },
  //   end: { x: 100, y: 30 }
  // },
  {
    start: {
      x: 100,
      y: 30
    },
    end: { x: 100, y: 40 }
  },
  {
    start: {
      x: 100,
      y: 40
    },
    end: { x: 100, y: 50 }
  }
];

// 点を描画する
// コメントを表示する

function Canvas(props: Props) {
  const canvas = useRef<HTMLCanvasElement>(null);
  useEffect(() => {
    const ctx = canvas.current?.getContext('2d');
    ctx?.fillRect(0, 0, props.width, props.height);
    if (ctx) {
      for (const data of testData) {
        drawPath(ctx, 'white', data.start, data.end);
        // drawDot(ctx, 'white', data.start.x, data.start.y, 3);
        // drawDot(ctx, 'white', data.end.x, data.end.y, 3);
      }
    }
  }, []);

  return (
    <>
      <canvas width={props.width} height={props.height} ref={canvas}></canvas>
    </>
  );
}

export default Canvas;
