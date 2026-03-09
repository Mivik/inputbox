package moe.mivik.inputbox

import android.content.Context
import android.text.InputType
import android.util.TypedValue
import android.view.Gravity
import android.widget.FrameLayout
import androidx.appcompat.app.AlertDialog
import com.google.android.material.textfield.TextInputEditText
import com.google.android.material.textfield.TextInputLayout

fun Context.dp(dp: Float): Int = TypedValue.applyDimension(
	TypedValue.COMPLEX_UNIT_DIP, dp, resources.displayMetrics
).toInt()

class InputBox {
	companion object {
		@JvmStatic
		fun showInput(
			callback: Long,
			title: String,
			prompt: String?,
			default: String,
			okLabel: String,
			cancelLabel: String,
			mode: String,
			autoWrap: Boolean,
			scrollToEnd: Boolean,
		): String? {
			val ctx = ActivityTracker.currentActivity ?: return "no active activity"
			ctx.runOnUiThread {
				val input = TextInputEditText(ctx).apply {
					setText(default)
					when (mode) {
						"password" -> {
							inputType = InputType.TYPE_CLASS_TEXT or InputType.TYPE_TEXT_VARIATION_PASSWORD
						}

						"multiline" -> {
							inputType = if (autoWrap) {
								InputType.TYPE_CLASS_TEXT or InputType.TYPE_TEXT_FLAG_MULTI_LINE
							} else {
								InputType.TYPE_CLASS_TEXT or InputType.TYPE_TEXT_FLAG_MULTI_LINE or InputType.TYPE_TEXT_VARIATION_VISIBLE_PASSWORD
							}

							if (!autoWrap) {
								setHorizontallyScrolling(true)
								maxLines = 10
							}

							minLines = 3
							gravity = Gravity.TOP
						}

						"text" -> {
							inputType = InputType.TYPE_CLASS_TEXT
						}

						else -> {
							inputType = InputType.TYPE_CLASS_TEXT
						}
					}
				}
				val layout = FrameLayout(ctx).apply {
					ctx.dp(5f).let { setPadding(it, it, it, it) }
					addView(TextInputLayout(ctx).apply {
						hint = prompt
						addView(input)
					})
				}
				AlertDialog.Builder(ctx).setTitle(title).setView(layout).setNegativeButton(cancelLabel, null)
					.setPositiveButton(okLabel) { _, _ ->
						inputCallback(callback, input.text.toString())
					}
					.setCancelable(true)
					.setOnCancelListener { inputCallback(callback, null) }
					.show()
			}
			return null
		}

		@JvmStatic
		external fun inputCallback(callback: Long, text: String?)
	}
}