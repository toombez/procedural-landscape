<script lang="ts">
import init, * as lng from 'landscape-generator'
</script>

<script setup lang="ts">
import { Graphics, Polygon } from 'pixi.js'


const size = 10

const heightMap = ref<Float32Array>()

onBeforeMount(async () => {
    await init()

    const generator = new lng.LandscapeGenerator(0)
    heightMap.value = generator.generate_heightmap(size, size, 1).map((value) => (value + 1) / 2)
})

function drawRectangle(height: number, e: Graphics) {
    const red = typeof height === 'undefined' ? '00' : Math.floor(height * 255).toString(16)

    e.beginFill(`#${red}0000`)
    e.drawRect(0, 0, 20, 20)
}
</script>

<template>
    <Application :width="240" :height="240">
        <text :anchor="0.5" :x="120" :y="120" :style="{ fill: 'white' }">
            Hello NuxtJS World
        </text>

        <template v-if="heightMap">
            <template v-for="x in size">
                <graphics
                    v-for="y in size"
                    :x="x * 20"
                    :y="y * 20"
                    @render="(e: Graphics) => drawRectangle(heightMap![y * size + x], e)"
                />
            </template>
        </template>
    </Application>
</template>
