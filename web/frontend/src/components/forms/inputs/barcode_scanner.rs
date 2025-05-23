use gloo::timers::callback::{Interval, Timeout};
use wasm_bindgen::JsCast;
use web_sys::{
    CanvasRenderingContext2d, HtmlCanvasElement, HtmlVideoElement, MediaStream, MediaStreamTrack,
};
use yew::prelude::*;

use crate::utils::is_mobile_device;
mod barcode_preprocessing;
mod decode_barcode;
use decode_barcode::decode_barcode;
mod camera_metadata;
use camera_metadata::{apply_torch_filter, get_available_cameras, get_camera_media_stream};

enum ScannerStatus {
    SwitchingCamera,
    RetrievingCameras,
    RetrievingStream,
    RetrievingAuthorization,
    Scanning,
    Closing,
    StartingFlashlight,
    Closed,
}

impl ScannerStatus {
    fn icon(&self) -> &str {
        match self {
            ScannerStatus::SwitchingCamera => "camera-retro",
            ScannerStatus::RetrievingCameras => "camera",
            ScannerStatus::RetrievingStream => "film",
            ScannerStatus::RetrievingAuthorization => "lock",
            ScannerStatus::Scanning => "qrcode",
            ScannerStatus::Closing => "trash",
            ScannerStatus::StartingFlashlight => "bolt",
            ScannerStatus::Closed => "trash",
        }
    }
}

impl std::fmt::Display for ScannerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScannerStatus::SwitchingCamera => write!(f, "Switching Camera"),
            ScannerStatus::RetrievingCameras => write!(f, "Retrieving Cameras"),
            ScannerStatus::RetrievingStream => write!(f, "Retrieving Stream"),
            ScannerStatus::RetrievingAuthorization => write!(f, "Retrieving Authorization"),
            ScannerStatus::Scanning => write!(f, "Scanning"),
            ScannerStatus::Closing => write!(f, "Closing"),
            ScannerStatus::StartingFlashlight => write!(f, "Starting Flashlight"),
            ScannerStatus::Closed => write!(f, "Closed"),
        }
    }
}

pub struct Scanner {
    video_ref: NodeRef,
    canvas_ref: NodeRef,
    stream: Option<MediaStream>,
    is_scanning: bool,
    is_flashlight_on: bool,
    video_ready: bool,
    stream_ready: bool,
    mirrored: bool,
    authorized: bool,
    current_camera: Option<(usize, web_sys::MediaDeviceInfo)>,
    number_of_identical_frames: u32,
    number_of_scan_attempts: u32,
    start_scanning_time: chrono::DateTime<chrono::Local>,
    cameras: Vec<web_sys::MediaDeviceInfo>,
    closing: Option<Timeout>,
    interval: Option<Interval>,
    status: ScannerStatus,
}

impl Default for Scanner {
    fn default() -> Self {
        Self {
            video_ref: NodeRef::default(),
            canvas_ref: NodeRef::default(),
            stream: None,
            is_scanning: false,
            is_flashlight_on: false,
            video_ready: false,
            stream_ready: false,
            current_camera: None,
            authorized: false,
            cameras: Vec::new(),
            mirrored: !is_mobile_device(),
            interval: None,
            number_of_scan_attempts: 0,
            start_scanning_time: chrono::Local::now(),
            closing: None,
            number_of_identical_frames: 0,
            status: ScannerStatus::Closed,
        }
    }
}

impl Scanner {
    /// Returns whether the scanner has cameras.
    pub fn has_cameras(&self) -> bool {
        !self.cameras.is_empty()
    }

    /// Returns the number of cameras currently detected.
    pub fn number_of_cameras(&self) -> usize {
        self.cameras.len()
    }

    /// Get the next camera label in the list of available cameras.
    fn get_next_camera(&self) -> Option<(usize, web_sys::MediaDeviceInfo)> {
        if self.number_of_cameras() < 2 {
            return None;
        }
        if let Some((index, _)) = self.current_camera {
            let next_index = (index + 1) % self.cameras.len();
            Some((next_index, self.cameras[next_index].clone()))
        } else {
            None
        }
    }

    /// Get the average number of scan attempts per second.
    fn average_scan_attempts_per_second(&self) -> usize {
        let elapsed_time = chrono::Local::now() - self.start_scanning_time;
        if elapsed_time.num_seconds() == 0 {
            return 0;
        }
        (self.number_of_scan_attempts / elapsed_time.num_seconds() as u32) as usize
    }
}

fn close_stream(stream: &MediaStream) {
    for track in stream.get_tracks().iter() {
        if let Ok(track) = track.dyn_into::<MediaStreamTrack>() {
            track.stop();
        }
    }
}

pub enum ScannerMessage {
    ReceivedStream(MediaStream),
    CapturedImage,
    Error(ApiError),
    Start,
    Close,
    ToggleFlashlight,
    VideoTimeUpdate,
    VideoReady,
    StreamReady,
    EffectivelyClose,
    SwitchCamera,
    Mirrored,
    RequireAuthorization,
    Authorized(web_sys::MediaStream),
    Cameras(Vec<web_sys::MediaDeviceInfo>),
}

#[derive(Properties, PartialEq, Clone)]
pub struct ScannerProps {
    #[prop_or_default]
    pub onscan: Callback<rxing::RXingResult>,
    #[prop_or_default]
    pub onerror: Callback<ApiError>,
    #[prop_or_default]
    pub onclose: Callback<()>,
    #[prop_or(30)]
    pub refresh_milliseconds: u32,
    #[prop_or(0.5)]
    crop_percentage: f64,
    #[prop_or(300)]
    crop_dimension: u32,
}

impl Component for Scanner {
    type Message = ScannerMessage;
    type Properties = ScannerProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ScannerMessage::RequireAuthorization => {
                let promise = match web_sys::window()
                    .unwrap()
                    .navigator()
                    .media_devices()
                    .unwrap()
                    .get_user_media_with_constraints(
                        &web_sys::MediaStreamConstraints::new().video(
                            &web_sys::MediaTrackConstraints::default()
                                .facing_mode(&web_sys::VideoFacingModeEnum::Environment.into()),
                        ),
                    ) {
                    Ok(promise) => promise,
                    Err(error) => {
                        ctx.link().send_message(ScannerMessage::Error(ApiError::from(error)));
                        return false;
                    }
                };
                ctx.link().send_future(async {
                    match wasm_bindgen_futures::JsFuture::from(promise).await {
                        Ok(stream) => {
                            ScannerMessage::Authorized(stream.unchecked_into::<MediaStream>())
                        }
                        Err(error) => ScannerMessage::Error(ApiError::from(error)),
                    }
                });
                false
            }
            ScannerMessage::Authorized(stream) => {
                self.authorized = true;
                ctx.link().send_message(ScannerMessage::ReceivedStream(stream));
                false
            }
            ScannerMessage::VideoTimeUpdate => {
                if self.interval.is_some() {
                    return false;
                }
                let link = ctx.link().clone();
                self.interval = Some(Interval::new(ctx.props().refresh_milliseconds, move || {
                    link.send_message(ScannerMessage::CapturedImage);
                }));
                false
            }
            ScannerMessage::VideoReady => {
                if self.video_ready {
                    return false;
                }
                self.video_ready = true;
                true
            }
            ScannerMessage::StreamReady => {
                if self.stream_ready {
                    return false;
                }
                self.stream_ready = true;
                true
            }
            ScannerMessage::ReceivedStream(stream) => {
                self.stream = Some(stream);
                self.video_ref
                    .cast::<HtmlVideoElement>()
                    .expect("video should be an HtmlVideoElement")
                    .set_src_object(self.stream.as_ref().clone());

                ctx.link().send_message(ScannerMessage::Start);

                false
            }
            ScannerMessage::CapturedImage => {
                if !self.is_scanning || !self.stream_ready || !self.video_ready {
                    return false;
                }

                let canvas = self
                    .canvas_ref
                    .cast::<HtmlCanvasElement>()
                    .expect("canvas should be an HtmlCanvasElement");

                // We prepare context options with desynchronized flag to avoid blocking the
                // main thread.

                let context_options = js_sys::Object::new();
                js_sys::Reflect::set(
                    &context_options,
                    &wasm_bindgen::JsValue::from_str("alpha"),
                    &wasm_bindgen::JsValue::from_bool(false),
                )
                .unwrap();
                // DESYNCHRONIZED IS NOT SUPPORTED IN SEVERAL ANDROID DEVICES
                // js_sys::Reflect::set(
                //     &context_options,
                //     &wasm_bindgen::JsValue::from_str("desynchronized"),
                //     &wasm_bindgen::JsValue::from_bool(true),
                // )
                // .unwrap();
                js_sys::Reflect::set(
                    &context_options,
                    &wasm_bindgen::JsValue::from_str("willReadFrequently"),
                    &wasm_bindgen::JsValue::from_bool(true),
                )
                .unwrap();

                let context = canvas
                    .get_context_with_context_options("2d", &context_options)
                    .expect("context should be available")
                    .unwrap()
                    .unchecked_into::<CanvasRenderingContext2d>();

                let video = self
                    .video_ref
                    .cast::<HtmlVideoElement>()
                    .expect("video should be an HtmlVideoElement");

                if video.video_width() == 0 || video.video_height() == 0 {
                    return false;
                }

                let previous_image_data = context
                    .get_image_data(
                        0.0,
                        0.0,
                        video.video_width() as f64,
                        video.video_height() as f64,
                    )
                    .unwrap();

                if let Err(error) = context.draw_image_with_html_video_element(&video, 0.0, 0.0) {
                    ctx.link().send_message(ScannerMessage::Error(ApiError::from(error)));
                    return false;
                }

                let image_data = context
                    .get_image_data(
                        0.0,
                        0.0,
                        video.video_width() as f64,
                        video.video_height() as f64,
                    )
                    .unwrap();

                if image_data.data().len() == 0 {
                    return false;
                }

                if image_data.width() == 0 || image_data.height() == 0 {
                    return false;
                }

                // If the two image data are exactly the same, something went wrong
                // and we assume that the video has stopped.
                if previous_image_data.data() == image_data.data() {
                    self.number_of_identical_frames += 1;
                    if self.number_of_identical_frames > 10 {
                        ctx.link().send_message(ScannerMessage::Error(
                            DeviceError::DeviceStoppedResponding.into(),
                        ));
                        return false;
                    }
                }

                self.number_of_identical_frames = 0;

                match decode_barcode(
                    image_data,
                    ctx.props().crop_percentage,
                    ctx.props().crop_dimension,
                ) {
                    Ok(s) => {
                        ctx.props().onscan.emit(s);
                        self.number_of_scan_attempts = 0;
                        ctx.link().send_message(ScannerMessage::Close);
                    }
                    Err(error) => {
                        match error {
                            rxing::Exceptions::NotFoundException(_) => {
                                // No barcode found, continue scanning
                                self.number_of_scan_attempts += 1;
                            }
                            error => {
                                ctx.link().send_message(ScannerMessage::Error(ApiError::from(
                                    vec![error.to_string()],
                                )));
                            }
                        }
                    }
                }
                true
            }
            ScannerMessage::Error(error) => {
                ctx.props().onerror.emit(error);
                ctx.link().send_message(ScannerMessage::Close);
                false
            }
            ScannerMessage::Mirrored => {
                self.mirrored = !self.mirrored;
                true
            }
            ScannerMessage::Start => {
                self.stream_ready = false;
                self.is_scanning = true;

                if !self.authorized {
                    self.status = ScannerStatus::RetrievingAuthorization;
                    ctx.link().send_message(ScannerMessage::RequireAuthorization);
                    return true;
                }

                if !self.has_cameras() {
                    self.status = ScannerStatus::RetrievingCameras;
                    ctx.link().send_future(async {
                        match get_available_cameras().await {
                            Ok(cameras) => ScannerMessage::Cameras(cameras),
                            Err(error) => ScannerMessage::Error(ApiError::from(error)),
                        }
                    });
                    return true;
                }

                if self.stream.is_none() {
                    self.status = ScannerStatus::RetrievingStream;
                    let (_, camera) = self.current_camera.as_ref().unwrap().clone();
                    ctx.link().send_future(async move {
                        match get_camera_media_stream(&camera.device_id()).await {
                            Ok(stream) => ScannerMessage::ReceivedStream(stream),
                            Err(error) => ScannerMessage::Error(ApiError::from(error)),
                        }
                    });
                    return true;
                }

                self.status = ScannerStatus::Scanning;
                self.start_scanning_time = chrono::Local::now();
                self.number_of_scan_attempts = 0;
                ctx.link().send_message(ScannerMessage::StreamReady);
                false
            }
            ScannerMessage::Close => {
                if self.closing.is_some() {
                    return false;
                }

                self.status = ScannerStatus::Closing;
                let link = ctx.link().clone();
                self.closing = Some(Timeout::new(300, move || {
                    link.send_message(ScannerMessage::EffectivelyClose);
                }));
                true
            }
            ScannerMessage::EffectivelyClose => {
                // close event
                if let Some(stream) = self.stream.take() {
                    close_stream(&stream);
                }
                if let Some(interval) = self.interval.take() {
                    interval.cancel();
                }

                *self = Self::default();
                ctx.props().onclose.emit(());
                true
            }
            ScannerMessage::ToggleFlashlight => {
                self.is_flashlight_on = !self.is_flashlight_on;
                self.stream_ready = false;
                let is_flashlight_on = self.is_flashlight_on;
                let stream = self.stream.as_ref().unwrap().clone();
                self.status = ScannerStatus::StartingFlashlight;
                ctx.link().send_future(async move {
                    if apply_torch_filter(&stream, is_flashlight_on).await {
                        ScannerMessage::StreamReady
                    } else {
                        ScannerMessage::Error(ApiError::from(vec![
                            "Failed to apply torch filter".to_string()
                        ]))
                    }
                });
                true
            }
            ScannerMessage::Cameras(cameras) => {
                self.current_camera = Some((0, cameras[0].clone()));
                self.cameras = cameras;
                ctx.link().send_message(ScannerMessage::Start);
                false
            }
            ScannerMessage::SwitchCamera => {
                self.status = ScannerStatus::SwitchingCamera;
                let (index, _) = self.current_camera.as_ref().unwrap().clone();
                let next_index = (index + 1) % self.cameras.len();
                self.current_camera = Some((next_index, self.cameras[next_index].clone()));
                if let Some(stream) = self.stream.take() {
                    close_stream(&stream);
                }
                self.stream_ready = false;
                ctx.link().send_message(ScannerMessage::Start);
                true
            }
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        if let Some(stream) = self.stream.take() {
            for track in stream.get_tracks().iter() {
                if let Ok(track) = track.dyn_into::<MediaStreamTrack>() {
                    track.stop();
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let time_update = ctx.link().callback(|_| ScannerMessage::VideoTimeUpdate);
        let toggle_scanner = ctx.link().callback(|event: MouseEvent| {
            event.prevent_default();
            event.stop_propagation();
            ScannerMessage::Start
        });
        let close_scanner = ctx.link().callback(|event: MouseEvent| {
            event.prevent_default();
            event.stop_propagation();
            ScannerMessage::Close
        });
        let toggle_flashlight = ctx.link().callback(|event: MouseEvent| {
            event.prevent_default();
            event.stop_propagation();
            ScannerMessage::ToggleFlashlight
        });
        let toggle_camera = ctx.link().callback(|event: MouseEvent| {
            event.prevent_default();
            event.stop_propagation();
            ScannerMessage::SwitchCamera
        });
        let mirror = ctx.link().callback(|event: MouseEvent| {
            event.prevent_default();
            event.stop_propagation();
            ScannerMessage::Mirrored
        });

        let classes = format!(
            "active-scanner-ui{}{}{}",
            if self.video_ready && self.stream_ready { "" } else { " loading" },
            if self.closing.is_some() { " closing" } else { " opening" },
            if self.mirrored { " mirrored" } else { "" }
        );

        let flash_light_message =
            if self.is_flashlight_on { "Turn off flashlight" } else { "Turn on flashlight" };

        html! {
            <>
            if !self.is_scanning {
                <button onclick={toggle_scanner} title="Start Scanner" class="start-scanner">
                    <i class="fas fa-qrcode"></i>
                </button>
            } else {
                <div class={classes} onclick={&close_scanner}>
                    <video ref={&self.video_ref} ontimeupdate={time_update} onplaying={ctx.link().callback(|_| ScannerMessage::VideoReady)} muted={true} playsinline={true} autoplay={true}></video>
                    if let Some(video) = self.video_ref.cast::<HtmlVideoElement>() {
                        <canvas ref={&self.canvas_ref} style="display:none;" width={video.video_width().to_string()} height={video.video_height().to_string()}></canvas>
                    }
                    <div class="scanner-focus-container">
                        <div class="scanner-focus"></div>
                        <ul class="operations">
                            <li title={flash_light_message} onclick={toggle_flashlight}>
                                <i class="fas fa-bolt"></i>
                            </li>
                            if let Some((_, camera)) = self.get_next_camera() {
                                <li class="switch-camera" camera-number={(self.current_camera.as_ref().unwrap().0 + 1).to_string()} camera-total={self.number_of_cameras().to_string()} title={camera.label()} onclick={toggle_camera}>
                                    <i class="fas fa-sync-alt"></i>
                                </li>
                            }
                            <li title="Mirror" onclick={mirror}>
                                <i class="fas fa-arrows-alt-h"></i>
                            </li>
                        </ul>
                    </div>
                    <div class="scan-per-second">
                        <span>{self.average_scan_attempts_per_second()}</span>
                        <span>{" scans/s"}</span>
                    </div>
                    <div class="ongoing-operation">
                        <i class={format!("fas fa-{}", self.status.icon())}></i>
                        {"\u{00A0}"}
                        <span>{self.status.to_string()}</span>
                    </div>
                </div>
            }
            </>
        }
    }
}
