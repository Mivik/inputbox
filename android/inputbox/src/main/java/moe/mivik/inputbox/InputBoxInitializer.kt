package moe.mivik.inputbox

import android.app.Application
import android.content.Context
import androidx.startup.Initializer

class InputBoxInitializer : Initializer<Unit> {
	override fun create(context: Context) {
		val app = context.applicationContext as Application
		app.registerActivityLifecycleCallbacks(ActivityTracker)
	}

	override fun dependencies(): List<Class<out Initializer<*>>> {
		return emptyList()
	}
}