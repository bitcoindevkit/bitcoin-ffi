buildscript {
    repositories {
        google()
    }
    dependencies {
        classpath("com.android.tools.build:gradle:7.1.2")
    }
}

// library version is defined in gradle.properties
val libraryVersion: String by project

// These properties are required here so that the nexus publish-plugin
// finds a staging profile with the correct group (group is otherwise set as "")
// and knows whether to publish to a SNAPSHOT repository or not
// https://github.com/gradle-nexus/publish-plugin#applying-the-plugin
group = "org.bitcoindevkit"
version = libraryVersion
