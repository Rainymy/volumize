package com.volumize.rainy

import android.content.pm.ActivityInfo
import android.os.Bundle
import android.view.WindowManager
import androidx.activity.enableEdgeToEdge

import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat

class MainActivity : TauriActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    super.onCreate(savedInstanceState)

    // Enable Edge-to-Edge first (makes bars transparent)
    enableEdgeToEdge()

    // Force Setting device Orientation to Landscape.
    requestedOrientation = ActivityInfo.SCREEN_ORIENTATION_SENSOR_LANDSCAPE
    // Request to keep the screen on.
    window.addFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)

    setContentView(R.layout.activity_main)

    // Use WindowInsetsController to HIDE the bars
    // We wrap this in a post call or listener to ensure the decorView is ready
    // and to re-apply if the user interacts with the screen.
    val windowInsetsController = WindowCompat.getInsetsController(window, window.decorView)

    windowInsetsController.systemBarsBehavior =
      WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE

    // Hiding immediately
    windowInsetsController.hide(WindowInsetsCompat.Type.systemBars())

    // Sometimes regaining focus re-shows the bars. This listener ensures they stay hidden.
    window.decorView.setOnApplyWindowInsetsListener { view, insets ->
      windowInsetsController.hide(WindowInsetsCompat.Type.systemBars())
      view.onApplyWindowInsets(insets)
    }
  }
}