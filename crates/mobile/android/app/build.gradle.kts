import java.io.File
import java.util.Properties

// App module build.gradle.kts for Umuryango.
//
// This module packages the pre-compiled Rust native library into an APK
// that uses Android's NativeActivity to host the GPUI application.
//
// The Rust library must be compiled separately and placed into
// app/src/main/jniLibs/<abi>/ before building the APK.

plugins {
    id("com.android.application")
}

val keystorePropertiesFile = rootProject.file("keystore.properties")
val keystoreProperties = Properties().apply {
    if (keystorePropertiesFile.isFile) {
        keystorePropertiesFile.inputStream().use(::load)
    }
}

fun signingValue(property: String, environment: String): String? =
    keystoreProperties.getProperty(property)?.takeIf(String::isNotBlank)
        ?: System.getenv(environment)?.takeIf(String::isNotBlank)

val configuredStoreFile = signingValue("storeFile", "ANDROID_KEYSTORE_PATH")?.let { path ->
    File(path).let { file ->
        if (file.isAbsolute) file else keystorePropertiesFile.parentFile.resolve(file)
    }
}
val configuredStorePassword = signingValue("storePassword", "ANDROID_KEYSTORE_PASSWORD")
val configuredKeyAlias = signingValue("keyAlias", "ANDROID_KEY_ALIAS")
val configuredKeyPassword = signingValue("keyPassword", "ANDROID_KEY_PASSWORD")
val releaseSigningConfigured = configuredStoreFile?.isFile == true &&
    configuredStorePassword != null && configuredKeyAlias != null && configuredKeyPassword != null
val allowDebugSigning = providers.gradleProperty("allowDebugSigning").orNull == "true"

// Automatically read workspace version from root Cargo.toml so Android APK versionName
// and versionCode stay synchronized with Cargo releases.
val cargoTomlFile = rootProject.file("../../../Cargo.toml")
val cargoVersionRegex = Regex("""(?m)^version\s*=\s*"(\d+)\.(\d+)\.(\d+)"""")
val cargoMatch = if (cargoTomlFile.isFile) {
    cargoVersionRegex.find(cargoTomlFile.readText())
} else {
    null
}
val resolvedVersionName = providers.gradleProperty("appVersionName").orNull
    ?: cargoMatch?.let { "${it.groupValues[1]}.${it.groupValues[2]}.${it.groupValues[3]}" }
    ?: "0.1.1"
val resolvedVersionCode = providers.gradleProperty("appVersionCode").orNull?.toIntOrNull()
    ?: cargoMatch?.let {
        val major = it.groupValues[1].toInt()
        val minor = it.groupValues[2].toInt()
        val patch = it.groupValues[3].toInt()
        (major * 10000 + minor * 100 + patch).coerceAtLeast(1)
    }
    ?: 101

gradle.taskGraph.whenReady {
    val buildsRelease = allTasks.any { task -> task.name.contains("Release", ignoreCase = true) }
    if (buildsRelease && !releaseSigningConfigured && !allowDebugSigning) {
        throw GradleException(
            "Release signing is not configured. Add android/keystore.properties or set the " +
                "ANDROID_KEYSTORE_* environment variables. For local testing or debug-signed " +
                "builds, pass -PallowDebugSigning=true."
        )
    }
}

android {
    namespace = "rw.umuryango.app"
    compileSdk = 34

    defaultConfig {
        applicationId = "rw.umuryango.app"
        minSdk = 26          // Vulkan 1.0 is mandatory from API 26+
        targetSdk = 34
        versionCode = resolvedVersionCode
        versionName = resolvedVersionName

        // Tell NativeActivity which .so to load.
        // This must match the cdylib output name in crates/mobile/Cargo.toml.
        ndk {
            abiFilters += listOf("arm64-v8a")
        }

        // Forward the library name to the manifest via a placeholder.
        manifestPlaceholders["nativeLibraryName"] = "umuryango_mobile"
    }

    flavorDimensions += "distribution"
    productFlavors {
        create("sideload") {
            dimension = "distribution"
        }
        create("play") {
            dimension = "distribution"
        }
    }

    signingConfigs {
        getByName("debug") {
            enableV2Signing = true
            enableV3Signing = true
        }
        create("release") {
            if (releaseSigningConfigured) {
                storeFile = configuredStoreFile
                storePassword = configuredStorePassword
                keyAlias = configuredKeyAlias
                keyPassword = configuredKeyPassword
            }
            enableV2Signing = true
            enableV3Signing = true
        }
    }

    buildTypes {
        release {
            signingConfig = signingConfigs.getByName(
                if (releaseSigningConfigured) "release" else "debug"
            )
            isMinifyEnabled = true
            isShrinkResources = true
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
        debug {
            isDebuggable = true
            isJniDebuggable = true
        }
    }

    // We do NOT use CMake / ndk-build — the native library is compiled
    // externally via cargo-ndk and placed directly into jniLibs.
    externalNativeBuild {
        // Intentionally left empty.
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }

    // Tell Gradle where the pre-built .so files live.
    sourceSets {
        getByName("main") {
            jniLibs.srcDirs("src/main/jniLibs")
        }
    }

    // Lint configuration — relaxed for mobile builds.
    lint {
        abortOnError = false
        checkReleaseBuilds = false
    }
}

dependencies {
    // AndroidX back dispatcher used by MainActivity.
    implementation("androidx.activity:activity:1.9.3")
    // AndroidX core for NotificationCompat & FileProvider
    implementation("androidx.core:core:1.12.0")
    // AndroidX SplashScreen compat (used by GpuiActivity to hold splash until native init)
    implementation("androidx.core:core-splashscreen:1.0.1")
}
