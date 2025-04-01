<script setup lang="ts">
import { Vector2 } from 'three';
// @ts-ignore
import fragmentShader from '~/assets/shaders/fragment.glsl'
// @ts-ignore
import vertexShader from '~/assets/shaders/vertex.glsl'

const blobRef = shallowRef<TresObject | null>(null);

const uniforms = {
    uTime: { value: 0 },
    uAmplitude: { value: new Vector2(0.1, 0.1) },
    uFrequency: { value: new Vector2(20, 5) },
}

const { onLoop } = useRenderLoop();

onLoop(({ delta, elapsed }) => {
    if (blobRef.value) {
        blobRef.value.material.uniforms.uTime.value = elapsed
    }
});
</script>

<template>
      <TresCanvas clear-color="#111" shadows alpha window-size>
        <OrbitControls />
        <TresPerspectiveCamera :position="[11, 11, 11]" :fov="45" :aspect="1" :near="0.1" :far="1000" />

        <TresMesh ref="blobRef" :position="[0, 4, 0]">
            <TresSphereGeometry :args="[2, 32, 32]" />
            <TresShaderMaterial :vertexShader="vertexShader" :fragmentShader="fragmentShader" :uniforms="uniforms" />
        </TresMesh>

        <TresMesh :rotation="[-Math.PI / 2, 0, 0]">
            <TresPlaneGeometry :args="[10, 10, 10, 10]" />
            <TresMeshBasicMaterial color="#444" />
        </TresMesh>
    </TresCanvas>
</template>
