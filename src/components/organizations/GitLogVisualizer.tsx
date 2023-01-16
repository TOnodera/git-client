import React from 'react';
import Canvas from '../atoms/Canvas';

function GitLogVisualizer() {
  const canvasSize = {
    width: 500,
    height: 500
  };
  return (
    <>
      <Canvas width={canvasSize.width} height={canvasSize.height} />
    </>
  );
}

export default GitLogVisualizer;
