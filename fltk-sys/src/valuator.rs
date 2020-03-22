/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Widget {
    _unused: [u8; 0],
}
pub type Fl_Callback = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut Fl_Widget, arg2: *mut ::std::os::raw::c_void),
>;
extern "C" {
    pub fn Fl_Widget_callback_with_captures(
        arg1: *mut Fl_Widget,
        cb: Fl_Callback,
        arg2: *mut ::std::os::raw::c_void,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Slider {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Slider_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Slider;
}
extern "C" {
    pub fn Fl_Slider_x(arg1: *mut Fl_Slider) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Slider_y(arg1: *mut Fl_Slider) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Slider_width(arg1: *mut Fl_Slider) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Slider_height(arg1: *mut Fl_Slider) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Slider_label(arg1: *mut Fl_Slider) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Slider_set_label(arg1: *mut Fl_Slider, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Slider_redraw(arg1: *mut Fl_Slider);
}
extern "C" {
    pub fn Fl_Slider_show(arg1: *mut Fl_Slider);
}
extern "C" {
    pub fn Fl_Slider_hide(arg1: *mut Fl_Slider);
}
extern "C" {
    pub fn Fl_Slider_activate(arg1: *mut Fl_Slider);
}
extern "C" {
    pub fn Fl_Slider_deactivate(arg1: *mut Fl_Slider);
}
extern "C" {
    pub fn Fl_Slider_redraw_label(arg1: *mut Fl_Slider);
}
extern "C" {
    pub fn Fl_Slider_resize(
        arg1: *mut Fl_Slider,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Slider_tooltip(arg1: *mut Fl_Slider) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Slider_set_tooltip(arg1: *mut Fl_Slider, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Slider_get_type(arg1: *mut Fl_Slider) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Slider_set_type(arg1: *mut Fl_Slider, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Slider_color(arg1: *mut Fl_Slider) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Slider_set_color(arg1: *mut Fl_Slider, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Slider_label_color(arg1: *mut Fl_Slider) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Slider_set_label_color(arg1: *mut Fl_Slider, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Slider_label_font(arg1: *mut Fl_Slider) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Slider_set_label_font(arg1: *mut Fl_Slider, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Slider_label_size(arg1: *mut Fl_Slider) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Slider_set_label_size(arg1: *mut Fl_Slider, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Slider_label_type(arg1: *mut Fl_Slider) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Slider_set_label_type(arg1: *mut Fl_Slider, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Slider_box(arg1: *mut Fl_Slider) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Slider_set_box(arg1: *mut Fl_Slider, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Slider_changed(arg1: *mut Fl_Slider) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Slider_set_changed(arg1: *mut Fl_Slider);
}
extern "C" {
    pub fn Fl_Slider_clear_changed(arg1: *mut Fl_Slider);
}
extern "C" {
    pub fn Fl_Slider_align(arg1: *mut Fl_Slider) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Slider_set_align(arg1: *mut Fl_Slider, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Slider_delete(arg1: *mut Fl_Slider);
}
extern "C" {
    pub fn Fl_Slider_set_image(arg1: *mut Fl_Slider, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Slider_handle(
        arg1: *mut Fl_Slider,
        event: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Slider_set_bounds(arg1: *mut Fl_Slider, a: f64, b: f64);
}
extern "C" {
    pub fn Fl_Slider_minimum(arg1: *mut Fl_Slider) -> f64;
}
extern "C" {
    pub fn Fl_Slider_set_minimum(arg1: *mut Fl_Slider, a: f64);
}
extern "C" {
    pub fn Fl_Slider_maximum(arg1: *mut Fl_Slider) -> f64;
}
extern "C" {
    pub fn Fl_Slider_set_maximum(arg1: *mut Fl_Slider, a: f64);
}
extern "C" {
    pub fn Fl_Slider_set_range(arg1: *mut Fl_Slider, a: f64, b: f64);
}
extern "C" {
    pub fn Fl_Slider_set_step(arg1: *mut Fl_Slider, a: f64, b: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Slider_step(arg1: *mut Fl_Slider) -> f64;
}
extern "C" {
    pub fn Fl_Slider_set_precision(arg1: *mut Fl_Slider, digits: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Slider_value(arg1: *mut Fl_Slider) -> f64;
}
extern "C" {
    pub fn Fl_Slider_set_value(arg1: *mut Fl_Slider, arg2: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Slider_format(
        arg1: *mut Fl_Slider,
        arg2: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Slider_round(arg1: *mut Fl_Slider, arg2: f64) -> f64;
}
extern "C" {
    pub fn Fl_Slider_clamp(arg1: *mut Fl_Slider, arg2: f64) -> f64;
}
extern "C" {
    pub fn Fl_Slider_increment(arg1: *mut Fl_Slider, arg2: f64, arg3: ::std::os::raw::c_int)
        -> f64;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Counter {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Counter_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Counter;
}
extern "C" {
    pub fn Fl_Counter_x(arg1: *mut Fl_Counter) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Counter_y(arg1: *mut Fl_Counter) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Counter_width(arg1: *mut Fl_Counter) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Counter_height(arg1: *mut Fl_Counter) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Counter_label(arg1: *mut Fl_Counter) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Counter_set_label(arg1: *mut Fl_Counter, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Counter_redraw(arg1: *mut Fl_Counter);
}
extern "C" {
    pub fn Fl_Counter_show(arg1: *mut Fl_Counter);
}
extern "C" {
    pub fn Fl_Counter_hide(arg1: *mut Fl_Counter);
}
extern "C" {
    pub fn Fl_Counter_activate(arg1: *mut Fl_Counter);
}
extern "C" {
    pub fn Fl_Counter_deactivate(arg1: *mut Fl_Counter);
}
extern "C" {
    pub fn Fl_Counter_redraw_label(arg1: *mut Fl_Counter);
}
extern "C" {
    pub fn Fl_Counter_resize(
        arg1: *mut Fl_Counter,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Counter_tooltip(arg1: *mut Fl_Counter) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Counter_set_tooltip(arg1: *mut Fl_Counter, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Counter_get_type(arg1: *mut Fl_Counter) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Counter_set_type(arg1: *mut Fl_Counter, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Counter_color(arg1: *mut Fl_Counter) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Counter_set_color(arg1: *mut Fl_Counter, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Counter_label_color(arg1: *mut Fl_Counter) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Counter_set_label_color(arg1: *mut Fl_Counter, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Counter_label_font(arg1: *mut Fl_Counter) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Counter_set_label_font(arg1: *mut Fl_Counter, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Counter_label_size(arg1: *mut Fl_Counter) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Counter_set_label_size(arg1: *mut Fl_Counter, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Counter_label_type(arg1: *mut Fl_Counter) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Counter_set_label_type(arg1: *mut Fl_Counter, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Counter_box(arg1: *mut Fl_Counter) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Counter_set_box(arg1: *mut Fl_Counter, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Counter_changed(arg1: *mut Fl_Counter) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Counter_set_changed(arg1: *mut Fl_Counter);
}
extern "C" {
    pub fn Fl_Counter_clear_changed(arg1: *mut Fl_Counter);
}
extern "C" {
    pub fn Fl_Counter_align(arg1: *mut Fl_Counter) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Counter_set_align(arg1: *mut Fl_Counter, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Counter_delete(arg1: *mut Fl_Counter);
}
extern "C" {
    pub fn Fl_Counter_set_image(arg1: *mut Fl_Counter, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Counter_handle(
        arg1: *mut Fl_Counter,
        event: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Counter_set_bounds(arg1: *mut Fl_Counter, a: f64, b: f64);
}
extern "C" {
    pub fn Fl_Counter_minimum(arg1: *mut Fl_Counter) -> f64;
}
extern "C" {
    pub fn Fl_Counter_set_minimum(arg1: *mut Fl_Counter, a: f64);
}
extern "C" {
    pub fn Fl_Counter_maximum(arg1: *mut Fl_Counter) -> f64;
}
extern "C" {
    pub fn Fl_Counter_set_maximum(arg1: *mut Fl_Counter, a: f64);
}
extern "C" {
    pub fn Fl_Counter_set_range(arg1: *mut Fl_Counter, a: f64, b: f64);
}
extern "C" {
    pub fn Fl_Counter_set_step(arg1: *mut Fl_Counter, a: f64, b: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Counter_step(arg1: *mut Fl_Counter) -> f64;
}
extern "C" {
    pub fn Fl_Counter_set_precision(arg1: *mut Fl_Counter, digits: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Counter_value(arg1: *mut Fl_Counter) -> f64;
}
extern "C" {
    pub fn Fl_Counter_set_value(arg1: *mut Fl_Counter, arg2: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Counter_format(
        arg1: *mut Fl_Counter,
        arg2: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Counter_round(arg1: *mut Fl_Counter, arg2: f64) -> f64;
}
extern "C" {
    pub fn Fl_Counter_clamp(arg1: *mut Fl_Counter, arg2: f64) -> f64;
}
extern "C" {
    pub fn Fl_Counter_increment(
        arg1: *mut Fl_Counter,
        arg2: f64,
        arg3: ::std::os::raw::c_int,
    ) -> f64;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Dial {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Dial_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Dial;
}
extern "C" {
    pub fn Fl_Dial_x(arg1: *mut Fl_Dial) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Dial_y(arg1: *mut Fl_Dial) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Dial_width(arg1: *mut Fl_Dial) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Dial_height(arg1: *mut Fl_Dial) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Dial_label(arg1: *mut Fl_Dial) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Dial_set_label(arg1: *mut Fl_Dial, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Dial_redraw(arg1: *mut Fl_Dial);
}
extern "C" {
    pub fn Fl_Dial_show(arg1: *mut Fl_Dial);
}
extern "C" {
    pub fn Fl_Dial_hide(arg1: *mut Fl_Dial);
}
extern "C" {
    pub fn Fl_Dial_activate(arg1: *mut Fl_Dial);
}
extern "C" {
    pub fn Fl_Dial_deactivate(arg1: *mut Fl_Dial);
}
extern "C" {
    pub fn Fl_Dial_redraw_label(arg1: *mut Fl_Dial);
}
extern "C" {
    pub fn Fl_Dial_resize(
        arg1: *mut Fl_Dial,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Dial_tooltip(arg1: *mut Fl_Dial) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Dial_set_tooltip(arg1: *mut Fl_Dial, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Dial_get_type(arg1: *mut Fl_Dial) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Dial_set_type(arg1: *mut Fl_Dial, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Dial_color(arg1: *mut Fl_Dial) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Dial_set_color(arg1: *mut Fl_Dial, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Dial_label_color(arg1: *mut Fl_Dial) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Dial_set_label_color(arg1: *mut Fl_Dial, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Dial_label_font(arg1: *mut Fl_Dial) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Dial_set_label_font(arg1: *mut Fl_Dial, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Dial_label_size(arg1: *mut Fl_Dial) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Dial_set_label_size(arg1: *mut Fl_Dial, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Dial_label_type(arg1: *mut Fl_Dial) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Dial_set_label_type(arg1: *mut Fl_Dial, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Dial_box(arg1: *mut Fl_Dial) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Dial_set_box(arg1: *mut Fl_Dial, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Dial_changed(arg1: *mut Fl_Dial) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Dial_set_changed(arg1: *mut Fl_Dial);
}
extern "C" {
    pub fn Fl_Dial_clear_changed(arg1: *mut Fl_Dial);
}
extern "C" {
    pub fn Fl_Dial_align(arg1: *mut Fl_Dial) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Dial_set_align(arg1: *mut Fl_Dial, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Dial_delete(arg1: *mut Fl_Dial);
}
extern "C" {
    pub fn Fl_Dial_set_image(arg1: *mut Fl_Dial, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Dial_handle(
        arg1: *mut Fl_Dial,
        event: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Dial_set_bounds(arg1: *mut Fl_Dial, a: f64, b: f64);
}
extern "C" {
    pub fn Fl_Dial_minimum(arg1: *mut Fl_Dial) -> f64;
}
extern "C" {
    pub fn Fl_Dial_set_minimum(arg1: *mut Fl_Dial, a: f64);
}
extern "C" {
    pub fn Fl_Dial_maximum(arg1: *mut Fl_Dial) -> f64;
}
extern "C" {
    pub fn Fl_Dial_set_maximum(arg1: *mut Fl_Dial, a: f64);
}
extern "C" {
    pub fn Fl_Dial_set_range(arg1: *mut Fl_Dial, a: f64, b: f64);
}
extern "C" {
    pub fn Fl_Dial_set_step(arg1: *mut Fl_Dial, a: f64, b: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Dial_step(arg1: *mut Fl_Dial) -> f64;
}
extern "C" {
    pub fn Fl_Dial_set_precision(arg1: *mut Fl_Dial, digits: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Dial_value(arg1: *mut Fl_Dial) -> f64;
}
extern "C" {
    pub fn Fl_Dial_set_value(arg1: *mut Fl_Dial, arg2: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Dial_format(
        arg1: *mut Fl_Dial,
        arg2: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Dial_round(arg1: *mut Fl_Dial, arg2: f64) -> f64;
}
extern "C" {
    pub fn Fl_Dial_clamp(arg1: *mut Fl_Dial, arg2: f64) -> f64;
}
extern "C" {
    pub fn Fl_Dial_increment(arg1: *mut Fl_Dial, arg2: f64, arg3: ::std::os::raw::c_int) -> f64;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Roller {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Roller_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Roller;
}
extern "C" {
    pub fn Fl_Roller_x(arg1: *mut Fl_Roller) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Roller_y(arg1: *mut Fl_Roller) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Roller_width(arg1: *mut Fl_Roller) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Roller_height(arg1: *mut Fl_Roller) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Roller_label(arg1: *mut Fl_Roller) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Roller_set_label(arg1: *mut Fl_Roller, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Roller_redraw(arg1: *mut Fl_Roller);
}
extern "C" {
    pub fn Fl_Roller_show(arg1: *mut Fl_Roller);
}
extern "C" {
    pub fn Fl_Roller_hide(arg1: *mut Fl_Roller);
}
extern "C" {
    pub fn Fl_Roller_activate(arg1: *mut Fl_Roller);
}
extern "C" {
    pub fn Fl_Roller_deactivate(arg1: *mut Fl_Roller);
}
extern "C" {
    pub fn Fl_Roller_redraw_label(arg1: *mut Fl_Roller);
}
extern "C" {
    pub fn Fl_Roller_resize(
        arg1: *mut Fl_Roller,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Roller_tooltip(arg1: *mut Fl_Roller) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Roller_set_tooltip(arg1: *mut Fl_Roller, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Roller_get_type(arg1: *mut Fl_Roller) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Roller_set_type(arg1: *mut Fl_Roller, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Roller_color(arg1: *mut Fl_Roller) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Roller_set_color(arg1: *mut Fl_Roller, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Roller_label_color(arg1: *mut Fl_Roller) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Roller_set_label_color(arg1: *mut Fl_Roller, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Roller_label_font(arg1: *mut Fl_Roller) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Roller_set_label_font(arg1: *mut Fl_Roller, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Roller_label_size(arg1: *mut Fl_Roller) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Roller_set_label_size(arg1: *mut Fl_Roller, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Roller_label_type(arg1: *mut Fl_Roller) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Roller_set_label_type(arg1: *mut Fl_Roller, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Roller_box(arg1: *mut Fl_Roller) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Roller_set_box(arg1: *mut Fl_Roller, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Roller_changed(arg1: *mut Fl_Roller) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Roller_set_changed(arg1: *mut Fl_Roller);
}
extern "C" {
    pub fn Fl_Roller_clear_changed(arg1: *mut Fl_Roller);
}
extern "C" {
    pub fn Fl_Roller_align(arg1: *mut Fl_Roller) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Roller_set_align(arg1: *mut Fl_Roller, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Roller_delete(arg1: *mut Fl_Roller);
}
extern "C" {
    pub fn Fl_Roller_set_image(arg1: *mut Fl_Roller, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Roller_handle(
        arg1: *mut Fl_Roller,
        event: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Roller_set_bounds(arg1: *mut Fl_Roller, a: f64, b: f64);
}
extern "C" {
    pub fn Fl_Roller_minimum(arg1: *mut Fl_Roller) -> f64;
}
extern "C" {
    pub fn Fl_Roller_set_minimum(arg1: *mut Fl_Roller, a: f64);
}
extern "C" {
    pub fn Fl_Roller_maximum(arg1: *mut Fl_Roller) -> f64;
}
extern "C" {
    pub fn Fl_Roller_set_maximum(arg1: *mut Fl_Roller, a: f64);
}
extern "C" {
    pub fn Fl_Roller_set_range(arg1: *mut Fl_Roller, a: f64, b: f64);
}
extern "C" {
    pub fn Fl_Roller_set_step(arg1: *mut Fl_Roller, a: f64, b: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Roller_step(arg1: *mut Fl_Roller) -> f64;
}
extern "C" {
    pub fn Fl_Roller_set_precision(arg1: *mut Fl_Roller, digits: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Roller_value(arg1: *mut Fl_Roller) -> f64;
}
extern "C" {
    pub fn Fl_Roller_set_value(arg1: *mut Fl_Roller, arg2: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Roller_format(
        arg1: *mut Fl_Roller,
        arg2: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Roller_round(arg1: *mut Fl_Roller, arg2: f64) -> f64;
}
extern "C" {
    pub fn Fl_Roller_clamp(arg1: *mut Fl_Roller, arg2: f64) -> f64;
}
extern "C" {
    pub fn Fl_Roller_increment(arg1: *mut Fl_Roller, arg2: f64, arg3: ::std::os::raw::c_int)
        -> f64;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Scrollbar {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Scrollbar_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Scrollbar;
}
extern "C" {
    pub fn Fl_Scrollbar_x(arg1: *mut Fl_Scrollbar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scrollbar_y(arg1: *mut Fl_Scrollbar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scrollbar_width(arg1: *mut Fl_Scrollbar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scrollbar_height(arg1: *mut Fl_Scrollbar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scrollbar_label(arg1: *mut Fl_Scrollbar) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Scrollbar_set_label(arg1: *mut Fl_Scrollbar, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Scrollbar_redraw(arg1: *mut Fl_Scrollbar);
}
extern "C" {
    pub fn Fl_Scrollbar_show(arg1: *mut Fl_Scrollbar);
}
extern "C" {
    pub fn Fl_Scrollbar_hide(arg1: *mut Fl_Scrollbar);
}
extern "C" {
    pub fn Fl_Scrollbar_activate(arg1: *mut Fl_Scrollbar);
}
extern "C" {
    pub fn Fl_Scrollbar_deactivate(arg1: *mut Fl_Scrollbar);
}
extern "C" {
    pub fn Fl_Scrollbar_redraw_label(arg1: *mut Fl_Scrollbar);
}
extern "C" {
    pub fn Fl_Scrollbar_resize(
        arg1: *mut Fl_Scrollbar,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Scrollbar_tooltip(arg1: *mut Fl_Scrollbar) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Scrollbar_set_tooltip(arg1: *mut Fl_Scrollbar, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Scrollbar_get_type(arg1: *mut Fl_Scrollbar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scrollbar_set_type(arg1: *mut Fl_Scrollbar, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Scrollbar_color(arg1: *mut Fl_Scrollbar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scrollbar_set_color(arg1: *mut Fl_Scrollbar, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Scrollbar_label_color(arg1: *mut Fl_Scrollbar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scrollbar_set_label_color(arg1: *mut Fl_Scrollbar, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Scrollbar_label_font(arg1: *mut Fl_Scrollbar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scrollbar_set_label_font(arg1: *mut Fl_Scrollbar, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Scrollbar_label_size(arg1: *mut Fl_Scrollbar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scrollbar_set_label_size(arg1: *mut Fl_Scrollbar, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Scrollbar_label_type(arg1: *mut Fl_Scrollbar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scrollbar_set_label_type(arg1: *mut Fl_Scrollbar, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Scrollbar_box(arg1: *mut Fl_Scrollbar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scrollbar_set_box(arg1: *mut Fl_Scrollbar, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Scrollbar_changed(arg1: *mut Fl_Scrollbar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scrollbar_set_changed(arg1: *mut Fl_Scrollbar);
}
extern "C" {
    pub fn Fl_Scrollbar_clear_changed(arg1: *mut Fl_Scrollbar);
}
extern "C" {
    pub fn Fl_Scrollbar_align(arg1: *mut Fl_Scrollbar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scrollbar_set_align(arg1: *mut Fl_Scrollbar, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Scrollbar_delete(arg1: *mut Fl_Scrollbar);
}
extern "C" {
    pub fn Fl_Scrollbar_set_image(arg1: *mut Fl_Scrollbar, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Scrollbar_handle(
        arg1: *mut Fl_Scrollbar,
        event: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scrollbar_set_bounds(arg1: *mut Fl_Scrollbar, a: f64, b: f64);
}
extern "C" {
    pub fn Fl_Scrollbar_minimum(arg1: *mut Fl_Scrollbar) -> f64;
}
extern "C" {
    pub fn Fl_Scrollbar_set_minimum(arg1: *mut Fl_Scrollbar, a: f64);
}
extern "C" {
    pub fn Fl_Scrollbar_maximum(arg1: *mut Fl_Scrollbar) -> f64;
}
extern "C" {
    pub fn Fl_Scrollbar_set_maximum(arg1: *mut Fl_Scrollbar, a: f64);
}
extern "C" {
    pub fn Fl_Scrollbar_set_range(arg1: *mut Fl_Scrollbar, a: f64, b: f64);
}
extern "C" {
    pub fn Fl_Scrollbar_set_step(arg1: *mut Fl_Scrollbar, a: f64, b: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Scrollbar_step(arg1: *mut Fl_Scrollbar) -> f64;
}
extern "C" {
    pub fn Fl_Scrollbar_set_precision(arg1: *mut Fl_Scrollbar, digits: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Scrollbar_value(arg1: *mut Fl_Scrollbar) -> f64;
}
extern "C" {
    pub fn Fl_Scrollbar_set_value(arg1: *mut Fl_Scrollbar, arg2: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scrollbar_format(
        arg1: *mut Fl_Scrollbar,
        arg2: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scrollbar_round(arg1: *mut Fl_Scrollbar, arg2: f64) -> f64;
}
extern "C" {
    pub fn Fl_Scrollbar_clamp(arg1: *mut Fl_Scrollbar, arg2: f64) -> f64;
}
extern "C" {
    pub fn Fl_Scrollbar_increment(
        arg1: *mut Fl_Scrollbar,
        arg2: f64,
        arg3: ::std::os::raw::c_int,
    ) -> f64;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Value_Slider {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Value_Slider_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Value_Slider;
}
extern "C" {
    pub fn Fl_Value_Slider_x(arg1: *mut Fl_Value_Slider) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Value_Slider_y(arg1: *mut Fl_Value_Slider) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Value_Slider_width(arg1: *mut Fl_Value_Slider) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Value_Slider_height(arg1: *mut Fl_Value_Slider) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Value_Slider_label(arg1: *mut Fl_Value_Slider) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Value_Slider_set_label(
        arg1: *mut Fl_Value_Slider,
        title: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Value_Slider_redraw(arg1: *mut Fl_Value_Slider);
}
extern "C" {
    pub fn Fl_Value_Slider_show(arg1: *mut Fl_Value_Slider);
}
extern "C" {
    pub fn Fl_Value_Slider_hide(arg1: *mut Fl_Value_Slider);
}
extern "C" {
    pub fn Fl_Value_Slider_activate(arg1: *mut Fl_Value_Slider);
}
extern "C" {
    pub fn Fl_Value_Slider_deactivate(arg1: *mut Fl_Value_Slider);
}
extern "C" {
    pub fn Fl_Value_Slider_redraw_label(arg1: *mut Fl_Value_Slider);
}
extern "C" {
    pub fn Fl_Value_Slider_resize(
        arg1: *mut Fl_Value_Slider,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Value_Slider_tooltip(arg1: *mut Fl_Value_Slider) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Value_Slider_set_tooltip(
        arg1: *mut Fl_Value_Slider,
        txt: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Value_Slider_get_type(arg1: *mut Fl_Value_Slider) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Value_Slider_set_type(arg1: *mut Fl_Value_Slider, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Value_Slider_color(arg1: *mut Fl_Value_Slider) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Value_Slider_set_color(arg1: *mut Fl_Value_Slider, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Value_Slider_label_color(arg1: *mut Fl_Value_Slider) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Value_Slider_set_label_color(
        arg1: *mut Fl_Value_Slider,
        color: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Value_Slider_label_font(arg1: *mut Fl_Value_Slider) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Value_Slider_set_label_font(arg1: *mut Fl_Value_Slider, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Value_Slider_label_size(arg1: *mut Fl_Value_Slider) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Value_Slider_set_label_size(arg1: *mut Fl_Value_Slider, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Value_Slider_label_type(arg1: *mut Fl_Value_Slider) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Value_Slider_set_label_type(arg1: *mut Fl_Value_Slider, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Value_Slider_box(arg1: *mut Fl_Value_Slider) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Value_Slider_set_box(arg1: *mut Fl_Value_Slider, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Value_Slider_changed(arg1: *mut Fl_Value_Slider) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Value_Slider_set_changed(arg1: *mut Fl_Value_Slider);
}
extern "C" {
    pub fn Fl_Value_Slider_clear_changed(arg1: *mut Fl_Value_Slider);
}
extern "C" {
    pub fn Fl_Value_Slider_align(arg1: *mut Fl_Value_Slider) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Value_Slider_set_align(arg1: *mut Fl_Value_Slider, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Value_Slider_delete(arg1: *mut Fl_Value_Slider);
}
extern "C" {
    pub fn Fl_Value_Slider_set_image(arg1: *mut Fl_Value_Slider, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Value_Slider_handle(
        arg1: *mut Fl_Value_Slider,
        event: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Value_Slider_set_bounds(arg1: *mut Fl_Value_Slider, a: f64, b: f64);
}
extern "C" {
    pub fn Fl_Value_Slider_minimum(arg1: *mut Fl_Value_Slider) -> f64;
}
extern "C" {
    pub fn Fl_Value_Slider_set_minimum(arg1: *mut Fl_Value_Slider, a: f64);
}
extern "C" {
    pub fn Fl_Value_Slider_maximum(arg1: *mut Fl_Value_Slider) -> f64;
}
extern "C" {
    pub fn Fl_Value_Slider_set_maximum(arg1: *mut Fl_Value_Slider, a: f64);
}
extern "C" {
    pub fn Fl_Value_Slider_set_range(arg1: *mut Fl_Value_Slider, a: f64, b: f64);
}
extern "C" {
    pub fn Fl_Value_Slider_set_step(arg1: *mut Fl_Value_Slider, a: f64, b: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Value_Slider_step(arg1: *mut Fl_Value_Slider) -> f64;
}
extern "C" {
    pub fn Fl_Value_Slider_set_precision(arg1: *mut Fl_Value_Slider, digits: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Value_Slider_value(arg1: *mut Fl_Value_Slider) -> f64;
}
extern "C" {
    pub fn Fl_Value_Slider_set_value(
        arg1: *mut Fl_Value_Slider,
        arg2: f64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Value_Slider_format(
        arg1: *mut Fl_Value_Slider,
        arg2: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Value_Slider_round(arg1: *mut Fl_Value_Slider, arg2: f64) -> f64;
}
extern "C" {
    pub fn Fl_Value_Slider_clamp(arg1: *mut Fl_Value_Slider, arg2: f64) -> f64;
}
extern "C" {
    pub fn Fl_Value_Slider_increment(
        arg1: *mut Fl_Value_Slider,
        arg2: f64,
        arg3: ::std::os::raw::c_int,
    ) -> f64;
}
