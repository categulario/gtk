// This file was generated by gir (6bcd52a) from gir-files (1069259)
// DO NOT EDIT

use IconSet;
use ImageType;
use Misc;
use Widget;
#[cfg(feature = "v3_10")]
use cairo;
use ffi;
use gdk_pixbuf;
use gio;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Image(Object<ffi::GtkImage>): Misc, Widget;

    match fn {
        get_type => || ffi::gtk_image_get_type(),
    }
}

impl Image {
    pub fn new() -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new()).downcast_unchecked()
        }
    }

    pub fn new_from_animation<P: IsA<gdk_pixbuf::PixbufAnimation>>(animation: &P) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_animation(animation.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_from_file<P: AsRef<std::path::Path>>(filename: P) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_file(filename.as_ref().to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_from_gicon<P: IsA<gio::Icon>>(icon: &P, size: i32) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_gicon(icon.to_glib_none().0, size)).downcast_unchecked()
        }
    }

    pub fn new_from_icon_name(icon_name: &str, size: i32) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_icon_name(icon_name.to_glib_none().0, size)).downcast_unchecked()
        }
    }

    pub fn new_from_icon_set(icon_set: &IconSet, size: i32) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_icon_set(icon_set.to_glib_none().0, size)).downcast_unchecked()
        }
    }

    pub fn new_from_pixbuf<'a, P: Into<Option<&'a gdk_pixbuf::Pixbuf>>>(pixbuf: P) -> Image {
        assert_initialized_main_thread!();
        let pixbuf = pixbuf.into();
        let pixbuf = pixbuf.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_pixbuf(pixbuf.0)).downcast_unchecked()
        }
    }

    pub fn new_from_resource(resource_path: &str) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_resource(resource_path.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_from_stock(stock_id: &str, size: i32) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_stock(stock_id.to_glib_none().0, size)).downcast_unchecked()
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn new_from_surface<'a, P: Into<Option<&'a cairo::Surface>>>(surface: P) -> Image {
        assert_initialized_main_thread!();
        let surface = surface.into();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_surface(mut_override(surface.to_glib_none().0))).downcast_unchecked()
        }
    }
}

impl Default for Image {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ImageExt {
    fn clear(&self);

    fn get_animation(&self) -> Option<gdk_pixbuf::PixbufAnimation>;

    fn get_gicon(&self) -> (gio::Icon, i32);

    fn get_icon_name(&self) -> (String, i32);

    fn get_icon_set(&self) -> (IconSet, i32);

    fn get_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf>;

    fn get_pixel_size(&self) -> i32;

    fn get_storage_type(&self) -> ImageType;

    fn set_from_animation<P: IsA<gdk_pixbuf::PixbufAnimation>>(&self, animation: &P);

    fn set_from_file<P: AsRef<std::path::Path>>(&self, filename: P);

    fn set_from_gicon<P: IsA<gio::Icon>>(&self, icon: &P, size: i32);

    fn set_from_icon_name(&self, icon_name: &str, size: i32);

    fn set_from_icon_set(&self, icon_set: &IconSet, size: i32);

    fn set_from_pixbuf<'a, P: Into<Option<&'a gdk_pixbuf::Pixbuf>>>(&self, pixbuf: P);

    fn set_from_resource<'a, P: Into<Option<&'a str>>>(&self, resource_path: P);

    fn set_from_stock(&self, stock_id: &str, size: i32);

    #[cfg(feature = "v3_10")]
    fn set_from_surface(&self, surface: &cairo::Surface);

    fn set_pixel_size(&self, pixel_size: i32);

    fn get_property_file(&self) -> Option<String>;

    fn set_property_file(&self, file: Option<&str>);

    fn set_property_gicon<P: IsA<gio::Icon> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, gicon: Option<&P>);

    fn set_property_icon_name(&self, icon_name: Option<&str>);

    fn get_property_icon_size(&self) -> i32;

    fn set_property_icon_size(&self, icon_size: i32);

    fn set_property_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>);

    fn get_property_pixbuf_animation(&self) -> Option<gdk_pixbuf::PixbufAnimation>;

    fn set_property_pixbuf_animation<P: IsA<gdk_pixbuf::PixbufAnimation> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, pixbuf_animation: Option<&P>);

    #[cfg(feature = "v3_8")]
    fn get_property_resource(&self) -> Option<String>;

    #[cfg(feature = "v3_8")]
    fn set_property_resource(&self, resource: Option<&str>);

    fn get_property_stock(&self) -> Option<String>;

    fn set_property_stock(&self, stock: Option<&str>);

    fn get_property_use_fallback(&self) -> bool;

    fn set_property_use_fallback(&self, use_fallback: bool);

    fn connect_property_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_pixbuf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_pixbuf_animation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_pixel_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    #[cfg(feature = "v3_8")]
    fn connect_property_resource_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_stock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_storage_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_use_fallback_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<Image> + IsA<glib::object::Object>> ImageExt for O {
    fn clear(&self) {
        unsafe {
            ffi::gtk_image_clear(self.to_glib_none().0);
        }
    }

    fn get_animation(&self) -> Option<gdk_pixbuf::PixbufAnimation> {
        unsafe {
            from_glib_none(ffi::gtk_image_get_animation(self.to_glib_none().0))
        }
    }

    fn get_gicon(&self) -> (gio::Icon, i32) {
        unsafe {
            let mut gicon = ptr::null_mut();
            let mut size = mem::uninitialized();
            ffi::gtk_image_get_gicon(self.to_glib_none().0, &mut gicon, &mut size);
            (from_glib_none(gicon), size)
        }
    }

    fn get_icon_name(&self) -> (String, i32) {
        unsafe {
            let mut icon_name = ptr::null();
            let mut size = mem::uninitialized();
            ffi::gtk_image_get_icon_name(self.to_glib_none().0, &mut icon_name, &mut size);
            (from_glib_none(icon_name), size)
        }
    }

    fn get_icon_set(&self) -> (IconSet, i32) {
        unsafe {
            let mut icon_set = ptr::null_mut();
            let mut size = mem::uninitialized();
            ffi::gtk_image_get_icon_set(self.to_glib_none().0, &mut icon_set, &mut size);
            (from_glib_none(icon_set), size)
        }
    }

    fn get_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_none(ffi::gtk_image_get_pixbuf(self.to_glib_none().0))
        }
    }

    fn get_pixel_size(&self) -> i32 {
        unsafe {
            ffi::gtk_image_get_pixel_size(self.to_glib_none().0)
        }
    }

    fn get_storage_type(&self) -> ImageType {
        unsafe {
            from_glib(ffi::gtk_image_get_storage_type(self.to_glib_none().0))
        }
    }

    fn set_from_animation<P: IsA<gdk_pixbuf::PixbufAnimation>>(&self, animation: &P) {
        unsafe {
            ffi::gtk_image_set_from_animation(self.to_glib_none().0, animation.to_glib_none().0);
        }
    }

    fn set_from_file<P: AsRef<std::path::Path>>(&self, filename: P) {
        unsafe {
            ffi::gtk_image_set_from_file(self.to_glib_none().0, filename.as_ref().to_glib_none().0);
        }
    }

    fn set_from_gicon<P: IsA<gio::Icon>>(&self, icon: &P, size: i32) {
        unsafe {
            ffi::gtk_image_set_from_gicon(self.to_glib_none().0, icon.to_glib_none().0, size);
        }
    }

    fn set_from_icon_name(&self, icon_name: &str, size: i32) {
        unsafe {
            ffi::gtk_image_set_from_icon_name(self.to_glib_none().0, icon_name.to_glib_none().0, size);
        }
    }

    fn set_from_icon_set(&self, icon_set: &IconSet, size: i32) {
        unsafe {
            ffi::gtk_image_set_from_icon_set(self.to_glib_none().0, icon_set.to_glib_none().0, size);
        }
    }

    fn set_from_pixbuf<'a, P: Into<Option<&'a gdk_pixbuf::Pixbuf>>>(&self, pixbuf: P) {
        let pixbuf = pixbuf.into();
        let pixbuf = pixbuf.to_glib_none();
        unsafe {
            ffi::gtk_image_set_from_pixbuf(self.to_glib_none().0, pixbuf.0);
        }
    }

    fn set_from_resource<'a, P: Into<Option<&'a str>>>(&self, resource_path: P) {
        let resource_path = resource_path.into();
        let resource_path = resource_path.to_glib_none();
        unsafe {
            ffi::gtk_image_set_from_resource(self.to_glib_none().0, resource_path.0);
        }
    }

    fn set_from_stock(&self, stock_id: &str, size: i32) {
        unsafe {
            ffi::gtk_image_set_from_stock(self.to_glib_none().0, stock_id.to_glib_none().0, size);
        }
    }

    #[cfg(feature = "v3_10")]
    fn set_from_surface(&self, surface: &cairo::Surface) {
        unsafe {
            ffi::gtk_image_set_from_surface(self.to_glib_none().0, mut_override(surface.to_glib_none().0));
        }
    }

    fn set_pixel_size(&self, pixel_size: i32) {
        unsafe {
            ffi::gtk_image_set_pixel_size(self.to_glib_none().0, pixel_size);
        }
    }

    fn get_property_file(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "file".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_file(&self, file: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "file".to_glib_none().0, Value::from(file).to_glib_none().0);
        }
    }

    fn set_property_gicon<P: IsA<gio::Icon> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, gicon: Option<&P>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "gicon".to_glib_none().0, Value::from(gicon).to_glib_none().0);
        }
    }

    fn set_property_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "icon-name".to_glib_none().0, Value::from(icon_name).to_glib_none().0);
        }
    }

    fn get_property_icon_size(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "icon-size".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_icon_size(&self, icon_size: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "icon-size".to_glib_none().0, Value::from(&icon_size).to_glib_none().0);
        }
    }

    fn set_property_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "pixbuf".to_glib_none().0, Value::from(pixbuf).to_glib_none().0);
        }
    }

    fn get_property_pixbuf_animation(&self) -> Option<gdk_pixbuf::PixbufAnimation> {
        let mut value = Value::from(None::<&gdk_pixbuf::PixbufAnimation>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "pixbuf-animation".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_pixbuf_animation<P: IsA<gdk_pixbuf::PixbufAnimation> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, pixbuf_animation: Option<&P>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "pixbuf-animation".to_glib_none().0, Value::from(pixbuf_animation).to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_8")]
    fn get_property_resource(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "resource".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    #[cfg(feature = "v3_8")]
    fn set_property_resource(&self, resource: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "resource".to_glib_none().0, Value::from(resource).to_glib_none().0);
        }
    }

    fn get_property_stock(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "stock".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_stock(&self, stock: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "stock".to_glib_none().0, Value::from(stock).to_glib_none().0);
        }
    }

    fn get_property_use_fallback(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "use-fallback".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_use_fallback(&self, use_fallback: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "use-fallback".to_glib_none().0, Value::from(&use_fallback).to_glib_none().0);
        }
    }

    fn connect_property_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::file",
                transmute(notify_file_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::gicon",
                transmute(notify_gicon_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::icon-name",
                transmute(notify_icon_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::icon-size",
                transmute(notify_icon_size_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_pixbuf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::pixbuf",
                transmute(notify_pixbuf_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_pixbuf_animation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::pixbuf-animation",
                transmute(notify_pixbuf_animation_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_pixel_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::pixel-size",
                transmute(notify_pixel_size_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_8")]
    fn connect_property_resource_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::resource",
                transmute(notify_resource_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_stock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::stock",
                transmute(notify_stock_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_storage_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::storage-type",
                transmute(notify_storage_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_use_fallback_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::use-fallback",
                transmute(notify_use_fallback_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_file_trampoline<P>(this: *mut ffi::GtkImage, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Image> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Image::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_gicon_trampoline<P>(this: *mut ffi::GtkImage, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Image> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Image::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_icon_name_trampoline<P>(this: *mut ffi::GtkImage, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Image> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Image::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_icon_size_trampoline<P>(this: *mut ffi::GtkImage, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Image> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Image::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_pixbuf_trampoline<P>(this: *mut ffi::GtkImage, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Image> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Image::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_pixbuf_animation_trampoline<P>(this: *mut ffi::GtkImage, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Image> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Image::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_pixel_size_trampoline<P>(this: *mut ffi::GtkImage, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Image> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Image::from_glib_none(this).downcast_unchecked())
}

#[cfg(feature = "v3_8")]
unsafe extern "C" fn notify_resource_trampoline<P>(this: *mut ffi::GtkImage, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Image> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Image::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_stock_trampoline<P>(this: *mut ffi::GtkImage, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Image> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Image::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_storage_type_trampoline<P>(this: *mut ffi::GtkImage, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Image> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Image::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_use_fallback_trampoline<P>(this: *mut ffi::GtkImage, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Image> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Image::from_glib_none(this).downcast_unchecked())
}
