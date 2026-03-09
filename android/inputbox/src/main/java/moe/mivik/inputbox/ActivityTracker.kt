package moe.mivik.inputbox

import android.app.Activity
import android.app.Application
import android.os.Bundle
import java.lang.ref.WeakReference

object ActivityTracker : Application.ActivityLifecycleCallbacks {
	private var topActivity: WeakReference<Activity?> = WeakReference(null)

	val currentActivity: Activity?
		get() = topActivity.get()

	override fun onActivityResumed(activity: Activity) {
		topActivity = WeakReference(activity)
	}

	override fun onActivityPaused(activity: Activity) {
		if (topActivity.get() === activity) {
			topActivity.clear()
		}
	}

	override fun onActivityDestroyed(activity: Activity) {
		if (topActivity.get() === activity) {
			topActivity.clear()
		}
	}

	override fun onActivityCreated(activity: Activity, savedInstanceState: Bundle?) {}
	override fun onActivityStarted(activity: Activity) {}
	override fun onActivityStopped(activity: Activity) {}
	override fun onActivitySaveInstanceState(activity: Activity, outState: Bundle) {}
}