//! Submodule providing the file input component for the frontend.

use super::InputErrors;
use crate::workers::ws_worker::WebsocketMessage;
use crate::workers::WebsocketWorker;
use base64::engine::general_purpose;
use base64::Engine;
use gloo::timers::callback::Timeout;
use image::ImageFormat;
use validator::Validate;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_common::api::form_traits::TryFromCallback;
use web_common::api::ApiError;
use web_common::custom_validators::Image;
use web_common::file_formats::GenericFileFormat;
use yew::prelude::*;
use yew_agent::prelude::WorkerBridgeHandle;
use yew_agent::scope_ext::AgentScopeExt;

/// Trait defining the preview of a file.
pub trait HTMLPreview {
    fn preview(&self) -> Html;
}

impl HTMLPreview for Image {
    fn preview(&self) -> Html {
        let data: &[u8] = self.as_ref();

        match Image::is_image(data).unwrap() {
            ImageFormat::Png => {
                let url = format!(
                    "data:image/png;base64,{}",
                    general_purpose::STANDARD.encode(data)
                );
                html! {
                    <img src={url} class="preview" />
                }
            }
            ImageFormat::Jpeg => {
                let url = format!(
                    "data:image/jpeg;base64,{}",
                    general_purpose::STANDARD.encode(data)
                );
                html! {
                    <img src={url} class="preview" />
                }
            }
            _ => {
                html! {
                    <div class="preview">{"Unsupported image format"}</div>
                }
            }
        }
    }
}

pub fn human_readable_size(size: u64) -> String {
    if size < 1024 {
        format!("{} B", size)
    } else if size < 1024 * 1024 {
        format!("{:.0} KB", size as f64 / 1024.0)
    } else if size < 1024 * 1024 * 1024 {
        format!("{:.0} MB", size as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", size as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct MultiFileInputProp<Data>
where
    Data: 'static + Clone + PartialEq,
{
    pub label: String,
    pub builder: Callback<Vec<Data>>,
    pub errors: Vec<ApiError>,
    pub values: Vec<Data>,
    #[prop_or(false)]
    pub optional: bool,
    #[prop_or(false)]
    pub multiple: bool,
    #[prop_or_default]
    pub allowed_formats: Vec<GenericFileFormat>,
    #[prop_or_default]
    pub maximal_size: Option<u64>,
}

impl<Data> MultiFileInputProp<Data>
where
    Data: 'static + Clone + PartialEq,
{
    pub fn label(&self) -> String {
        self.label.clone()
    }

    pub fn normalized_label(&self) -> String {
        self.label.to_lowercase().replace(" ", "_")
    }

    pub fn human_readable_allowed_formats(&self) -> String {
        let mut formats = self
            .allowed_formats
            .iter()
            .flat_map(|f| f.extensions())
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        if formats.len() == 1 {
            formats[0].to_string()
        } else if formats.len() == 2 {
            format!("{} and {}", formats[0], formats[1])
        } else {
            let last = formats.pop().unwrap();
            let first = formats.join(", ");
            format!("{}, and {}", first, last)
        }
    }
}

pub struct MultiFileInput<Data> {
    _websocket: WorkerBridgeHandle<WebsocketWorker>,
    errors: Vec<ApiError>,
    is_valid: Option<bool>,
    validation_timeout: Option<Timeout>,
    show_drop_area: bool,
    show_drop_area_timeout: Option<Timeout>,
    hide_drop_area_timeout: Option<Timeout>,
    dragging: bool,
    _phantom: std::marker::PhantomData<Data>,
}

pub enum MultiFileInputMessage<Data> {
    Backend(WebsocketMessage),
    Validate(Result<(web_sys::File, Data), ApiError>),
    Files(web_sys::FileList),
    FilesRemoved(usize),
    SetTimeoutDropAreaVisibility(bool),
    SetDropAreaVisibility(bool),
    SetDragging(bool),
}

impl<Data> Component for MultiFileInput<Data>
where
    Data: 'static + Clone + Validate + TryFromCallback<web_sys::File> + PartialEq + HTMLPreview,
{
    type Message = MultiFileInputMessage<Data>;
    type Properties = MultiFileInputProp<Data>;

    fn create(ctx: &Context<Self>) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();

        let link = ctx.link().clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            // Handle the dragover event here
            // For example, prevent the default behavior to allow dropping elements
            event.prevent_default();
            link.send_message(MultiFileInputMessage::SetDragging(true));
        }) as Box<dyn FnMut(_)>);
        document
            .add_event_listener_with_callback("dragover", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget(); // Prevent the closure from being dropped immediately

        let link = ctx.link().clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            // Handle the dragend event here
            // For example, prevent the default behavior to allow dropping elements
            event.prevent_default();
            link.send_message(MultiFileInputMessage::SetDragging(false));
        }) as Box<dyn FnMut(_)>);
        document
            .add_event_listener_with_callback("dragend", closure.as_ref().unchecked_ref())
            .unwrap();
        document
            .add_event_listener_with_callback("dragleave", closure.as_ref().unchecked_ref())
            .unwrap();
        document
            .add_event_listener_with_callback("drop", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget(); // Prevent the closure from being dropped immediately

        Self {
            _websocket: ctx.link().bridge_worker(Callback::from({
                let link = ctx.link().clone();
                move |message: WebsocketMessage| {
                    link.send_message(MultiFileInputMessage::Backend(message));
                }
            })),
            errors: Vec::new(),
            is_valid: None,
            validation_timeout: None,
            show_drop_area: ctx.props().values.is_empty(),
            show_drop_area_timeout: None,
            hide_drop_area_timeout: None,
            dragging: false,
            _phantom: std::marker::PhantomData,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        if !self.show_drop_area
            && self.show_drop_area_timeout.is_none()
            && ctx.props().values.is_empty()
        {
            ctx.link()
                .send_message(MultiFileInputMessage::SetTimeoutDropAreaVisibility(true));
        }

        if self.show_drop_area
            && self.hide_drop_area_timeout.is_none()
            && !ctx.props().values.is_empty()
        {
            ctx.link()
                .send_message(MultiFileInputMessage::SetTimeoutDropAreaVisibility(false));
        }

        match msg {
            MultiFileInputMessage::Backend(_bm) => false,
            MultiFileInputMessage::SetDragging(dragging) => {
                if self.dragging != dragging {
                    self.dragging = dragging;
                    true
                } else {
                    false
                }
            }
            MultiFileInputMessage::Files(files) => {
                if files.length() == 0 {
                    return false;
                }

                let mut change = false;

                if !self.errors.is_empty() {
                    self.errors.clear();
                    change = true;
                }

                // If the properties require the file to be singular, we only keep the first file
                // and replace eventual previous files.

                let number_of_files = if ctx.props().multiple {
                    files.length()
                } else {
                    files.length().min(1)
                };

                let allowed_formats = &ctx.props().allowed_formats;

                for i in 0..number_of_files {
                    let file = files.get(i).unwrap();

                    if let Some(maximal_size) = ctx.props().maximal_size {
                        if file.size() as u64 > maximal_size {
                            self.errors.push(ApiError::BadRequest(vec![format!(
                                "The file {} is too large. The maximal size is {}, but the file is {}.",
                                file.name(),
                                human_readable_size(maximal_size),
                                human_readable_size(file.size() as u64)
                            )]));
                            change = true;
                            continue;
                        }
                    }

                    if !allowed_formats.is_empty() {
                        match GenericFileFormat::try_from_mime(&file.type_()) {
                            Ok(format) => {
                                if !allowed_formats.iter().any(|f| f == &format) {
                                    self.errors.push(ApiError::BadRequest(vec![format!(
                                        concat!(
                                            "The file {} is not of an allowed format. ",
                                            "The allowed formats are: {}."
                                        ),
                                        file.type_(),
                                        ctx.props().human_readable_allowed_formats()
                                    )]));
                                    change = true;
                                    continue;
                                }
                            }
                            Err(error) => {
                                self.errors.push(error);
                                change = true;
                                continue;
                            }
                        }
                    }

                    let link = ctx.link().clone();
                    if let Err(error) = Data::try_from_callback(file.clone(), move |result| {
                        link.send_message(MultiFileInputMessage::Validate(
                            result.map(|data| (file, data)),
                        ));
                    }) {
                        ctx.link()
                            .send_message(MultiFileInputMessage::Validate(Err(error)));
                    }
                }

                change
            }
            MultiFileInputMessage::Validate(data) => {
                if let Some(timeout) = self.validation_timeout.take() {
                    timeout.cancel();
                }

                match data {
                    Ok((file, data)) => {
                        self.is_valid = Some(true);
                        ctx.props().builder.emit(if ctx.props().multiple {
                            let mut files = ctx.props().values.clone();
                            files.push(data);
                            files
                        } else {
                            vec![data]
                        });
                    }
                    Err(error) => {
                        self.errors.push(error);
                        self.is_valid = Some(false);
                    }
                }

                true
            }
            MultiFileInputMessage::FilesRemoved(index) => {
                ctx.props().builder.emit({
                    let mut files = ctx.props().values.clone();
                    files.remove(index);
                    files
                });

                true
            }
            MultiFileInputMessage::SetDropAreaVisibility(visibility) => {
                if let Some(timeout) = self.hide_drop_area_timeout.take() {
                    timeout.cancel();
                }
                if let Some(timeout) = self.show_drop_area_timeout.take() {
                    timeout.cancel();
                }
                if self.show_drop_area != visibility {
                    self.show_drop_area = visibility;
                    true
                } else {
                    false
                }
            }
            MultiFileInputMessage::SetTimeoutDropAreaVisibility(visibility) => {
                if let Some(timeout) = self.hide_drop_area_timeout.take() {
                    timeout.cancel();
                }
                if let Some(timeout) = self.show_drop_area_timeout.take() {
                    timeout.cancel();
                }
                let link = ctx.link().clone();

                // If the target visibility is already the current visibility, we do not need to
                // do anything.
                if visibility == self.show_drop_area {
                    return false;
                }

                if visibility {
                    self.show_drop_area_timeout = Some(Timeout::new(200, move || {
                        link.send_message(MultiFileInputMessage::SetDropAreaVisibility(true));
                    }));
                } else {
                    self.hide_drop_area_timeout = Some(Timeout::new(200, move || {
                        link.send_message(MultiFileInputMessage::SetDropAreaVisibility(false));
                    }));
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        let classes = match self.is_valid {
            Some(true) => "input-group file input-group-valid",
            Some(false) => "input-group file input-group-invalid",
            None => "input-group file",
        };

        let uuid = uuid::Uuid::new_v4().to_string();
        // First, we handle that on click on the div, the file input is triggered and
        // the user can select files.
        let on_click = {
            let uuid = uuid.clone();
            Callback::from(move |_: MouseEvent| {
                // This click event may come from any of the children of the div, so we
                // need to make sure that we retrieve the correct input field. We can use
                // the uuid to do so, as as to minimize the risk of conflicts.
                let input = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_element_by_id(&uuid)
                    .unwrap()
                    .unchecked_into::<web_sys::HtmlInputElement>();
                input.click();
            })
        };

        // Then, we handle the input event, which is triggered when the user selects files.
        let on_input = {
            let link = ctx.link().clone();
            Callback::from(move |input_event: InputEvent| {
                input_event.prevent_default();

                let files = input_event
                    .target()
                    .unwrap()
                    .unchecked_into::<web_sys::HtmlInputElement>()
                    .files()
                    .unwrap();

                link.send_message(MultiFileInputMessage::Files(files));
            })
        };

        let on_drop = {
            let link = ctx.link().clone();
            Callback::from(move |drop_event: DragEvent| {
                drop_event.prevent_default();
                let files = drop_event.data_transfer().unwrap().files().unwrap();
                link.send_message(MultiFileInputMessage::Files(files));
            })
        };

        let on_file_delete = {
            let link = ctx.link().clone();
            Callback::from(move |index: usize| {
                link.send_message(MultiFileInputMessage::FilesRemoved(index));
            })
        };

        let no_files = ctx.props().values.is_empty();

        let ondragover = {
            let link = ctx.link().clone();
            Callback::from(move |event: DragEvent| {
                event.prevent_default();
                link.send_message(MultiFileInputMessage::SetTimeoutDropAreaVisibility(true));
            })
        };

        let ondragleave = {
            let link = ctx.link().clone();
            Callback::from(move |event: DragEvent| {
                event.prevent_default();
                link.send_message(MultiFileInputMessage::SetTimeoutDropAreaVisibility(
                    no_files,
                ));
            })
        };

        let droparea_classes = format!(
            "droparea{}{}",
            if self.dragging { " dragging" } else { "" },
            if self.hide_drop_area_timeout.is_some() {
                " hiding"
            } else {
                ""
            }
        );

        html! {
            <div class={classes} onclick={on_click} ondrop={on_drop} ondragover={ondragover} ondragleave={ondragleave.clone()} ondragend={ondragleave}>
                <label for={props.normalized_label()}>{format!("{}:", props.label())}</label>
                <input
                    type="file"
                    id={uuid}
                    class="input-control"
                    name={props.normalized_label()}
                    multiple={props.multiple}
                    // value={input_value}
                    oninput={on_input}
                    // onblur={on_blur}
                />
                {if self.show_drop_area {
                    html! {
                        <div class={droparea_classes}>
                            <div class="droparea-icon"><i class="fas fa-file-upload"></i></div>
                            <p>{"Drag & Drop files here or click to select"}</p>
                        </div>
                    }
                } else {
                    html! {
                        <FilePreview<Data>
                            hiding={self.show_drop_area_timeout.is_some()}
                            values={ctx.props().values.clone()}
                            on_delete={on_file_delete}
                        />
                    }
                }}
                <InputErrors errors={self.errors.clone()} />
            </div>
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct FilePreviewProp<Data>
where
    Data: 'static + Clone + PartialEq,
{
    pub values: Vec<Data>,
    pub on_delete: Callback<usize>,
    #[prop_or_default]
    pub hiding: bool,
}

impl<Data> FilePreviewProp<Data>
where
    Data: 'static + Clone + PartialEq,
{
    pub fn number_of_files(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn one_file(&self) -> Option<&Data> {
        if self.number_of_files() == 1 {
            self.values.first()
        } else {
            None
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Data> {
        self.values.iter()
    }
}

#[function_component(FilePreview)]
/// A component to display a preview of the files that have been selected.
///
/// # Implementative details
/// Depending on whether the provided files are images or not, the component will display
/// either the name of the file with an adequate icon, or a preview of the image. When the
/// document appear to be human-readable, the component will display a preview of the first
/// few lines of the document. Furthermore, the component will display the size of the file,
/// and the date at which it was last modified,
/// and a button to remove the file from the list of selected files.
///
/// # Properties
/// The component receives a list of web_sys::File objects, which it then displays. Since it
/// also needs to be able to communicate with the parent component, it receives a callback to
/// send messages to the parent component to delete the file from the list of selected files.
pub fn file_preview<Data>(props: &FilePreviewProp<Data>) -> Html
where
    Data: 'static + Clone + PartialEq + HTMLPreview,
{
    if props.is_empty() {
        return html! {
            <></>
        };
    }

    if let Some(data) = props.one_file() {
        // let on_delete = {
        //     let props = props.clone();
        //     Callback::from(move |_| {
        //         let props = props.clone();
        //         let timeout = Timeout::new(200, move || {
        //             props.on_delete.emit(0);
        //         });
        //         timeout.forget();
        //     })
        // };

        return data.preview();
    }

    let mut class = format!(
        "file-preview-list {}",
        if props.hiding { "hiding" } else { "" }
    );

    if props.number_of_files() == 2 {
        class += " two-files";
    }

    if props.number_of_files() == 3 {
        class += " three-files";
    }

    html! {
        <ul class={class}>
            { for props.iter().enumerate().map(|(i, file)|{
                // let props = props.clone();
                // let on_delete = Callback::from(move |_| {
                //     let props = props.clone();
                //     let timeout = Timeout::new(200, move || {
                //         props.on_delete.emit(i);
                //     });
                //     timeout.forget();
                // });

                file.preview()
                })
            }
        </ul>
    }
}

// #[derive(Clone, PartialEq, Properties)]
// pub struct FilePreviewItemProp<Data>
// where
//     Data: 'static + Clone + PartialEq,
// {
//     pub file: Data,
//     #[prop_or_default]
//     pub on_delete: Callback<()>,
//     pub large: bool,
//     #[prop_or_default]
//     pub hiding: bool,
// }

// impl<Data> FilePreviewItemProp<Data>
// where
//     Data: 'static + Clone + PartialEq,
// {
//     pub fn path(&self) -> Option<String> {
//         match &self.file {
//             FileLike::File(file) => Some(file.name()),
//             FileLike::Url(_) => None,
//         }
//     }

//     pub fn name(&self) -> Option<String> {
//         match &self.file {
//             FileLike::File(file) => Some(file.name().split('/').last().unwrap().to_string()),
//             FileLike::Url(_) => None,
//         }
//     }

//     pub fn extension(&self) -> Option<String> {
//         match &self.file {
//             FileLike::File(file) => {
//                 let path = file.name();
//                 let extension = path.split('.').last();
//                 extension.map(|s| s.to_string())
//             }
//             FileLike::Url(url) => {
//                 let extension = url.split('.').last();
//                 extension.map(|s| s.to_string())
//             }
//         }
//     }

//     pub fn size(&self) -> Option<u64> {
//         match &self.file {
//             FileLike::File(file) => Some(file.size() as u64),
//             FileLike::Url(_) => None,
//         }
//     }

//     pub fn is_image(&self) -> bool {
//         match &self.file {
//             FileLike::File(file) => file.type_().starts_with("image"),
//             FileLike::Url(url) => {
//                 url.ends_with(".png")
//                     || url.ends_with(".jpg")
//                     || url.ends_with(".jpeg")
//                     || url.ends_with(".gif")
//                     || url.ends_with(".webp")
//             }
//         }
//     }

//     pub fn is_pdf(&self) -> bool {
//         match &self.file {
//             FileLike::File(file) => file.type_().starts_with("application/pdf"),
//             FileLike::Url(url) => url.ends_with(".pdf"),
//         }
//     }

//     pub fn last_modified(&self) -> Option<u64> {
//         match &self.file {
//             FileLike::File(file) => Some(file.last_modified() as u64),
//             FileLike::Url(_) => None,
//         }
//     }

//     pub fn metadata_hash(&self) -> u64 {
//         let mut hasher = DefaultHasher::new();
//         match &self.file {
//             FileLike::File(file) => {
//                 file.name().hash(&mut hasher);
//             }
//             FileLike::Url(url) => {
//                 url.hash(&mut hasher);
//             }
//         }
//         self.size().hash(&mut hasher);
//         self.last_modified().hash(&mut hasher);
//         hasher.finish()
//     }

//     pub fn human_readable_size(&self) -> Option<String> {
//         self.size().map(human_readable_size)
//     }

//     pub fn last_modified_date(&self) -> Option<String> {
//         self.last_modified().map(|last_modified| {
//             let date = js_sys::Date::new(&JsValue::from_f64(last_modified as f64));
//             let date = date.to_locale_string("en-US", &JsValue::undefined());
//             date.as_string().unwrap()
//         })
//     }

//     /// Returns a human-readable string representing the time elapsed since the file was last modified.
//     pub fn human_readable_modified_delta(&self) -> Option<String> {
//         let date = self.last_modified()?;
//         let date = js_sys::Date::new(&JsValue::from_f64(date as f64));
//         let now = js_sys::Date::new_0();
//         let delta = now.get_time() - date.get_time();
//         let delta = (delta as f64 / 1000.0) as u64;
//         Some(if delta < 60 {
//             format!("{} seconds ago", delta)
//         } else if delta < 60 * 60 {
//             format!("{} minutes ago", delta / 60)
//         } else if delta < 60 * 60 * 24 {
//             format!("{} hours ago", delta / (60 * 60))
//         } else if delta < 60 * 60 * 24 * 30 {
//             format!("{} days ago", delta / (60 * 60 * 24))
//         } else if delta < 60 * 60 * 24 * 30 * 12 {
//             format!("{} months ago", delta / (60 * 60 * 24 * 30))
//         } else {
//             format!("{} years ago", delta / (60 * 60 * 24 * 30 * 12))
//         })
//     }

//     /// Returns the file metadata when the variant is a File.
//     pub fn file_metadata(&self) -> Option<String> {
//         match &self.file {
//             FileLike::File(_) => Some(format!(
//                 "{}, {}, from {}",
//                 self.name()?,
//                 self.human_readable_size()?,
//                 self.human_readable_modified_delta()?
//             )),
//             FileLike::Url(_) => None,
//         }
//     }
// }

// pub struct ImagePreview {}

// impl Component for ImagePreview {
//     type Message = ();
//     type Properties = FilePreviewItemProp;

//     fn create(_ctx: &Context<Self>) -> Self {
//         Self {}
//     }

//     fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
//         false
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         // We add an hash obtained from the file name and the associated informations, such
//         // as the file size and the last modified date, to the URL to make sure that the URL
//         // changes when the file changes, so that the image is reloaded when the file changes.

//         let hash = ctx.props().metadata_hash();

//         match &ctx.props().file {
//             FileLike::File(file) => {
//                 let url = web_sys::Url::create_object_url_with_blob(&file).unwrap();
//                 let url = format!("{}#{}", url, hash);
//                 html! {
//                     <img src={url} class="preview" />
//                 }
//             }
//             FileLike::Url(url) => {
//                 let url = format!("{}#{}", url, hash);
//                 html! {
//                     <img src={url} class="preview" />
//                 }
//             }
//         }
//     }
// }

// pub struct PDFPreview {}

// impl Component for PDFPreview {
//     type Message = ();
//     type Properties = FilePreviewItemProp;

//     fn create(_ctx: &Context<Self>) -> Self {
//         Self {}
//     }

//     fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
//         false
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         // We add an hash obtained from the file name and the associated informations, such
//         // as the file size and the last modified date, to the URL to make sure that the URL
//         // changes when the file changes, so that the image is reloaded when the file changes.

//         let hash = ctx.props().metadata_hash();

//         match &ctx.props().file {
//             FileLike::File(file) => {
//                 let url = web_sys::Url::create_object_url_with_blob(&file).unwrap();
//                 let url = format!("{}#{}", url, hash);
//                 html! {
//                     <iframe src={url} class="preview"></iframe>
//                 }
//             }
//             FileLike::Url(url) => {
//                 let url = format!("{}#{}", url, hash);
//                 html! {
//                     <iframe src={url} class="preview"></iframe>
//                 }
//             }
//         }
//     }
// }

// pub struct TextualFilePreview {
//     text: String,
// }

// #[derive(Clone, PartialEq, Properties)]
// pub struct TextualFilePreviewProps {
//     pub file_props: FilePreviewItemProp,
//     #[prop_or(20)]
//     pub number_of_lines: usize,
// }

// pub enum TextualFilePreviewMessage {
//     Load(String),
// }

// impl Component for TextualFilePreview {
//     type Message = TextualFilePreviewMessage;
//     type Properties = TextualFilePreviewProps;

//     fn create(ctx: &Context<Self>) -> Self {
//         let file = match ctx.props().file_props.file.clone() {
//             FileLike::File(file) => file,
//             FileLike::Url(_) => {
//                 unreachable!("TextualFilePreview should only be used with files, not URLs.")
//             }
//         };
//         let reader = web_sys::FileReader::new().unwrap();
//         let link = ctx.link().clone();
//         // We read the first few lines of the file to display a preview of the file.

//         const CHUNK_SIZE: usize = 1024; // Adjust the chunk size as needed

//         let number_of_lines = ctx.props().number_of_lines;

//         let on_load = Closure::wrap(Box::new(move |event: web_sys::ProgressEvent| {
//             let mut lines_read = 0;
//             let mut text = String::new();
//             let reader = event
//                 .target()
//                 .unwrap()
//                 .dyn_into::<web_sys::FileReader>()
//                 .unwrap();
//             let result = reader.result().unwrap();
//             let data = js_sys::Uint8Array::new(&result);

//             let mut chunk = Vec::with_capacity(CHUNK_SIZE);
//             for i in 0..data.length() {
//                 chunk.push(data.get_index(i));
//                 if chunk.len() >= CHUNK_SIZE {
//                     let chunk_text = String::from_utf8_lossy(&chunk);

//                     for line in chunk_text.lines() {
//                         text.push_str(line);
//                         text.push('\n');
//                         lines_read += 1;
//                         if lines_read >= number_of_lines {
//                             break;
//                         }
//                     }

//                     if lines_read >= number_of_lines {
//                         break;
//                     }

//                     chunk.clear();
//                 }
//             }

//             if lines_read < number_of_lines {
//                 let remaining_chunk_text = String::from_utf8_lossy(&chunk);
//                 text.push_str(&remaining_chunk_text);
//                 lines_read += remaining_chunk_text.lines().count();
//             }

//             link.send_message(TextualFilePreviewMessage::Load(text));
//         }) as Box<dyn FnMut(_)>);

//         reader.set_onload(Some(on_load.as_ref().unchecked_ref()));
//         on_load.forget();

//         reader.read_as_array_buffer(&file).unwrap();

//         Self {
//             text: String::new(),
//         }
//     }

//     fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
//         match msg {
//             TextualFilePreviewMessage::Load(text) => {
//                 self.text = text;
//                 true
//             }
//         }
//     }

//     fn view(&self, _ctx: &Context<Self>) -> Html {
//         html! {
//             <div class="file-preview-item-thumbnail">
//                 <p>{&self.text}</p>
//             </div>
//         }
//     }
// }

// #[function_component(FilePreviewItem)]
// /// A component to display a preview of a single file of a list of files.
// pub fn file_preview_item<Data>(props: &FilePreviewItemProp<Data>) -> Html
// where
//     Data: 'static + Clone + PartialEq,
// {
//     let on_delete = props.on_delete.clone();
//     let hiding = use_state(|| false);

//     let on_click = {
//         let hiding = hiding.clone();
//         Callback::from(move |click: MouseEvent| {
//             click.stop_immediate_propagation();
//             click.prevent_default();
//             hiding.set(true);
//             on_delete.emit(());
//         })
//     };

//     let thumbnail: Html = {
//         if props.is_image() {
//             html! { <ImagePreview file={props.file.clone()} large={props.large} /> }
//         } else if props.is_pdf() {
//             html! { <PDFPreview file={props.file.clone()} large={props.large} /> }
//         } else {
//             html! { <TextualFilePreview file_props={props.clone()} /> }
//         }
//     };

//     let class = format!(
//         "file-preview-item{}",
//         if props.hiding || *hiding {
//             " hiding"
//         } else {
//             ""
//         }
//     );

//     let wrapped = html! {
//         <>
//             {thumbnail}
//             {if let Some(metadata) = props.file_metadata() {
//                 html!{
//                     <>
//                     <button class="delete" onclick={on_click}><i class="fas fa-times"></i></button>
//                     <p class="metadata">{metadata}</p>
//                     </>
//                 }
//             } else {
//                 html!{
//                     <></>
//                 }
//             }}
//         </>
//     };

//     if props.large {
//         html! {
//             <div class={class} title={props.path()}>
//                 {wrapped}
//             </div>
//         }
//     } else {
//         html! {
//             <li class={class} title={props.path()}>
//                 {wrapped}
//             </li>
//         }
//     }
// }

#[derive(Clone, PartialEq, Properties)]
pub struct FileInputProp<Data>
where
    Data: 'static + Clone + PartialEq,
{
    pub label: String,
    pub builder: Callback<Option<Data>>,
    pub errors: Vec<ApiError>,
    pub value: Option<Data>,
    #[prop_or(false)]
    pub optional: bool,
    #[prop_or_default]
    pub allowed_formats: Vec<GenericFileFormat>,
    #[prop_or_default]
    pub maximal_size: Option<u64>,
}

#[function_component(FileInput)]
pub fn file_input<Data>(props: &FileInputProp<Data>) -> Html
where
    Data: 'static + Clone + PartialEq + Validate + TryFromCallback<web_sys::File> + HTMLPreview,
{
    let builder_callback = {
        let old_builder = props.builder.clone();
        Callback::from(move |mut data: Vec<Data>| {
            old_builder.emit(data.pop());
        })
    };

    html! {
        <MultiFileInput<Data>
            label={props.label.clone()}
            builder={builder_callback}
            errors={props.errors.clone()}
            optional={props.optional}
            multiple={false}
            allowed_formats={props.allowed_formats.clone()}
            values={props.value.clone().map_or_else(|| Vec::new(), |value| vec![value])}
            maximal_size={props.maximal_size}
        />
    }
}
