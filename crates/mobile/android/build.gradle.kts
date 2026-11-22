// Root build.gradle.kts for the Umuryango Mobile Android project.
//
// Build steps:
//   1. Compile the Rust library:
//      cargo ndk -t arm64-v8a -o crates/mobile/android/app/src/main/jniLibs --platform 31 build --release -p umuryango-mobile
//
//   2. Build the APK:
//      ./gradlew assembleSideloadRelease -PallowDebugSigning=true
//
//   3. Install on device/emulator:
//      adb install -r app/build/outputs/apk/sideload/release/app-sideload-release.apk

buildscript {
    repositories {
        google()
        mavenCentral()
    }
    dependencies {
        classpath("com.android.tools.build:gradle:9.1.0")
        classpath("org.jetbrains.kotlin:kotlin-gradle-plugin:1.9.22")
    }
}

tasks.register("clean", Delete::class) {
    delete(rootProject.layout.buildDirectory)
}

tasks.register<Exec>("buildRustRelease") {
    group = "rust"
    description = "Compile the Umuryango Rust native library for arm64-v8a using cargo-ndk."
    workingDir = rootProject.projectDir.parentFile.parentFile.parentFile
    commandLine(
        "cargo", "ndk",
        "-t", "arm64-v8a",
        "-o", "crates/mobile/android/app/src/main/jniLibs",
        "--platform", "31",
        "build", "--release", "-p", "umuryango-mobile"
    )
}

tasks.register<Exec>("buildRustDebug") {
    group = "rust"
    description = "Compile the Umuryango Rust native library for arm64-v8a (debug) using cargo-ndk."
    workingDir = rootProject.projectDir.parentFile.parentFile.parentFile
    commandLine(
        "cargo", "ndk",
        "-t", "arm64-v8a",
        "-o", "crates/mobile/android/app/src/main/jniLibs",
        "--platform", "31",
        "build", "-p", "umuryango-mobile"
    )
}

tasks.register("buildAll") {
    group = "rust"
    description = "Build Rust library (release) and then assemble the sideload release APK."
    dependsOn("buildRustRelease")
    finalizedBy(":app:assembleSideloadRelease")
}
