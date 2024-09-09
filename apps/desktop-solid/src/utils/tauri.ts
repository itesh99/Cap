
// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

/** user-defined commands **/


export const commands = {
async getRecordingOptions() : Promise<Result<RecordingOptions, null>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("get_recording_options") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async setRecordingOptions(options: RecordingOptions) : Promise<Result<null, null>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("set_recording_options", { options }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async createCameraWindow() : Promise<void> {
    await TAURI_INVOKE("create_camera_window");
},
async startRecording() : Promise<Result<null, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("start_recording") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async stopRecording() : Promise<Result<null, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("stop_recording") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async listCameras() : Promise<string[]> {
    return await TAURI_INVOKE("list_cameras");
},
async listCaptureWindows() : Promise<CaptureWindow[]> {
    return await TAURI_INVOKE("list_capture_windows");
},
async listAudioDevices() : Promise<string[]> {
    return await TAURI_INVOKE("list_audio_devices");
},
async getPrevRecordings() : Promise<Result<string[], null>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("get_prev_recordings") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async showPreviousRecordingsWindow() : Promise<void> {
    await TAURI_INVOKE("show_previous_recordings_window");
},
async closePreviousRecordingsWindow() : Promise<void> {
    await TAURI_INVOKE("close_previous_recordings_window");
},
async setFakeWindowBounds(name: string, bounds: Bounds) : Promise<Result<null, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("set_fake_window_bounds", { name, bounds }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async removeFakeWindow(name: string) : Promise<Result<null, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("remove_fake_window", { name }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async focusCapturesPanel() : Promise<void> {
    await TAURI_INVOKE("focus_captures_panel");
},
async getCurrentRecording() : Promise<Result<JsonValue<InProgressRecording | null>, null>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("get_current_recording") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async renderToFile(outputPath: string, videoId: string, project: ProjectConfiguration, progressChannel: TAURI_CHANNEL<RenderProgress>) : Promise<void> {
    await TAURI_INVOKE("render_to_file", { outputPath, videoId, project, progressChannel });
},
async getRenderedVideo(videoId: string, project: ProjectConfiguration) : Promise<Result<string, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("get_rendered_video", { videoId, project }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async copyFileToPath(src: string, dst: string) : Promise<Result<null, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("copy_file_to_path", { src, dst }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async copyRenderedVideoToClipboard(videoId: string, project: ProjectConfiguration) : Promise<Result<null, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("copy_rendered_video_to_clipboard", { videoId, project }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async getVideoMetadata(videoId: string, videoType: VideoType | null) : Promise<Result<[number, number], string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("get_video_metadata", { videoId, videoType }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async createEditorInstance(videoId: string) : Promise<Result<SerializedEditorInstance, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("create_editor_instance", { videoId }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async startPlayback(videoId: string, project: ProjectConfiguration) : Promise<void> {
    await TAURI_INVOKE("start_playback", { videoId, project });
},
async stopPlayback(videoId: string) : Promise<void> {
    await TAURI_INVOKE("stop_playback", { videoId });
},
async setPlayheadPosition(videoId: string, frameNumber: number) : Promise<void> {
    await TAURI_INVOKE("set_playhead_position", { videoId, frameNumber });
},
async openInFinder(path: string) : Promise<void> {
    await TAURI_INVOKE("open_in_finder", { path });
},
async saveProjectConfig(videoId: string, config: ProjectConfiguration) : Promise<void> {
    await TAURI_INVOKE("save_project_config", { videoId, config });
},
async openEditor(id: string) : Promise<void> {
    await TAURI_INVOKE("open_editor", { id });
},
async openMainWindow() : Promise<void> {
    await TAURI_INVOKE("open_main_window");
},
async openPermissionSettings(settings: OSPermissionSettings) : Promise<void> {
    await TAURI_INVOKE("open_permission_settings", { settings });
},
async doPermissionsCheck() : Promise<OSPermissionsCheck> {
    return await TAURI_INVOKE("do_permissions_check");
}
}

/** user-defined events **/


export const events = __makeEvents__<{
editorStateChanged: EditorStateChanged,
newRecordingAdded: NewRecordingAdded,
recordingOptionsChanged: RecordingOptionsChanged,
renderFrameEvent: RenderFrameEvent,
showCapturesPanel: ShowCapturesPanel
}>({
editorStateChanged: "editor-state-changed",
newRecordingAdded: "new-recording-added",
recordingOptionsChanged: "recording-options-changed",
renderFrameEvent: "render-frame-event",
showCapturesPanel: "show-captures-panel"
})

/** user-defined constants **/



/** user-defined types **/

export type AspectRatio = "wide" | "vertical" | "square" | "classic" | "tall"
export type Audio = { duration: number; sample_rate: number; channels: number }
export type AudioConfiguration = { mute: boolean; improve: boolean }
export type BackgroundConfiguration = { source: BackgroundSource; blur: number; padding: number; rounding: number; inset: number; crop: Crop | null }
export type BackgroundSource = { type: "wallpaper"; id: number } | { type: "image"; path: string | null } | { type: "color"; value: [number, number, number] } | { type: "gradient"; from: [number, number, number]; to: [number, number, number] }
export type Bounds = { x: number; y: number; width: number; height: number }
export type CameraConfiguration = { hide: boolean; mirror: boolean; position: CameraPosition; rounding: number; shadow: number }
export type CameraPosition = { x: CameraXPosition; y: CameraYPosition }
export type CameraXPosition = "left" | "center" | "right"
export type CameraYPosition = "top" | "bottom"
export type CaptureTarget = { type: "screen" } | { type: "window"; id: number }
export type CaptureWindow = { id: number; name: string }
export type Crop = { position: XY<number>; size: XY<number> }
export type CursorConfiguration = { hideWhenIdle: boolean; size: number; type: CursorType }
export type CursorType = "pointer" | "circle"
export type DisplaySource = { variant: "screen" } | { variant: "window"; bounds: Bounds }
export type EditorStateChanged = { playhead_position: number }
export type HotkeysConfiguration = { show: boolean }
export type InProgressRecording = { recordingDir: string; displaySource: DisplaySource }
export type JsonValue<T> = [T]
export type MacOSPermissionSettings = "screenRecording"
export type MacOSPermissionsCheck = { screenRecording: boolean }
export type NewRecordingAdded = { path: string }
export type OSPermissionSettings = { macOS: MacOSPermissionSettings }
export type OSPermissionsCheck = ({ os: "macOS" } & MacOSPermissionsCheck) | { os: "other" }
export type ProjectConfiguration = { aspectRatio: AspectRatio | null; background: BackgroundConfiguration; camera: CameraConfiguration; audio: AudioConfiguration; cursor: CursorConfiguration; hotkeys: HotkeysConfiguration }
export type ProjectRecordings = { display: Video; camera: Video | null; audio: Audio | null }
export type RecordingOptions = { captureTarget: CaptureTarget; cameraLabel: string | null; audioInputName: string | null }
export type RecordingOptionsChanged = null
export type RenderFrameEvent = { frame_number: number; project: ProjectConfiguration }
export type RenderProgress = { type: "Starting"; total_frames: number } | { type: "EstimatedTotalFrames"; total_frames: number } | { type: "FrameRendered"; current_frame: number }
export type SerializedEditorInstance = { framesSocketUrl: string; recordingDuration: number; savedProjectConfig: ProjectConfiguration | null; recordings: ProjectRecordings; path: string }
export type ShowCapturesPanel = null
export type Video = { duration: number; width: number; height: number; fps: number }
export type VideoType = "screen" | "output"
export type XY<T> = { x: T; y: T }

/** tauri-specta globals **/

import {
	invoke as TAURI_INVOKE,
	Channel as TAURI_CHANNEL,
} from "@tauri-apps/api/core";
import * as TAURI_API_EVENT from "@tauri-apps/api/event";
import { type WebviewWindow as __WebviewWindow__ } from "@tauri-apps/api/webviewWindow";

type __EventObj__<T> = {
	listen: (
		cb: TAURI_API_EVENT.EventCallback<T>,
	) => ReturnType<typeof TAURI_API_EVENT.listen<T>>;
	once: (
		cb: TAURI_API_EVENT.EventCallback<T>,
	) => ReturnType<typeof TAURI_API_EVENT.once<T>>;
	emit: T extends null
		? (payload?: T) => ReturnType<typeof TAURI_API_EVENT.emit>
		: (payload: T) => ReturnType<typeof TAURI_API_EVENT.emit>;
};

export type Result<T, E> =
	| { status: "ok"; data: T }
	| { status: "error"; error: E };

function __makeEvents__<T extends Record<string, any>>(
	mappings: Record<keyof T, string>,
) {
	return new Proxy(
		{} as unknown as {
			[K in keyof T]: __EventObj__<T[K]> & {
				(handle: __WebviewWindow__): __EventObj__<T[K]>;
			};
		},
		{
			get: (_, event) => {
				const name = mappings[event as keyof T];

				return new Proxy((() => {}) as any, {
					apply: (_, __, [window]: [__WebviewWindow__]) => ({
						listen: (arg: any) => window.listen(name, arg),
						once: (arg: any) => window.once(name, arg),
						emit: (arg: any) => window.emit(name, arg),
					}),
					get: (_, command: keyof __EventObj__<any>) => {
						switch (command) {
							case "listen":
								return (arg: any) => TAURI_API_EVENT.listen(name, arg);
							case "once":
								return (arg: any) => TAURI_API_EVENT.once(name, arg);
							case "emit":
								return (arg: any) => TAURI_API_EVENT.emit(name, arg);
						}
					},
				});
			},
		},
	);
}
