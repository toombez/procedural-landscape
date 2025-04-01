<template>
    <div>
        <div style="position: fixed; top: 0; background: white; z-index: 50;">
            <NuxtLink to="/tres">
                tres
            </NuxtLink>
            <NuxtLink to="/pixi">
                pixi
            </NuxtLink>
            <NuxtLink to="/glsl">
                glsl
            </NuxtLink>
        </div>

        <NuxtPage />
    </div>
</template>

<style>
html,
body {
    margin: 0;
    padding: 0;
    height: 100%;
    width: 100%;
}
#app {
    height: 100%;
    width: 100%;
}
</style>
