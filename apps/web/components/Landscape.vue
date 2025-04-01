<script lang="ts">
import init, * as lng from 'landscape-generator'
import { OrbitControls } from '@tresjs/cientos'
// import {  } from '@tresjs/'

interface Heightmap {
    width: number;
    height: number;
    data: Float32Array;
}
</script>

<script setup lang="ts">
import { BufferGeometry, Float32BufferAttribute, Mesh, MeshToonMaterial } from 'three'

const geometry = ref(new BufferGeometry())
const size = 10

const mesh = computed(() => {
    const mesh = new Mesh(geometry.value, new MeshToonMaterial({ color: 0x00ff00 }))

    mesh.castShadow = true
    return mesh
})

onBeforeMount(async () => {
    await init()

    const generator = new lng.LandscapeGenerator(0)
    const heightMap = generator.generate_heightmap(size, size, 5.0)

    const vertices = []
    const segments = size

    for (let i = 0; i < segments; i++) {
        for (let j = 0; j < segments; j++) {
            vertices.push(i + 1, heightMap[(i + 1) * segments + j], j)
            vertices.push(i, heightMap[i * segments + j], j)
            vertices.push(i, heightMap[i * segments + (j + 1)], j + 1)

            vertices.push(i + 1, heightMap[(i + 1) * segments + j], j)
            vertices.push(i, heightMap[i * segments + (j + 1)], j + 1)
            vertices.push(i + 1, heightMap[(i + 1) * segments + (j + 1)], j + 1)
        }
    }

    geometry.value.setAttribute('position', new Float32BufferAttribute(vertices, 3))
    geometry.value.computeVertexNormals()
})
</script>

<template>
    <TresCanvas
        window-size
        shadows
        clear-color="#111"
    >
        <TresPerspectiveCamera
            :position="[20, 20, 20]"
            :look-at="[0, 0, 0]"
        />

        <OrbitControls />

        <primitive
            cast-shadow
            receive-shadow
            :object="mesh"
        />

        <TresMesh cast-shadow :position="[2, -2, 0]">
            <TresSphereGeometry />
            <TresMeshToonMaterial color="#FBB03B" />
        </TresMesh>

        <TresMesh
            receive-shadow
            :position="[0, -3, 0]"
            :rotation="[-Math.PI / 2, 0, 0]"
        >
            <TresPlaneGeometry :args="[10, 10, 10, 10]" />
            <!-- <TresMeshToonMaterial color="#f7f7f7" /> -->
            <TresMeshStandardMaterial color="#f7f7f7" />
        </TresMesh>

        <TresAmbientLight
            :intensity="1"
        />
        <TresDirectionalLight
            cast-shadow
            :position="[0, 10, 0]"
            :intensity="1"
        />
    </TresCanvas>
</template>
