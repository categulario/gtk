// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gtk_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Buildable;
use Container;
use DirectionType;
use MenuDirectionType;
use MenuItem;
use Widget;

glib_wrapper! {
    pub struct MenuShell(Object<gtk_sys::GtkMenuShell, gtk_sys::GtkMenuShellClass, MenuShellClass>) @extends Container, Widget, @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_menu_shell_get_type(),
    }
}

pub const NONE_MENU_SHELL: Option<&MenuShell> = None;

pub trait MenuShellExt: 'static {
    fn activate_item<P: IsA<Widget>>(&self, menu_item: &P, force_deactivate: bool);

    fn append<P: IsA<MenuItem>>(&self, child: &P);

    fn bind_model<P: IsA<gio::MenuModel>>(
        &self,
        model: Option<&P>,
        action_namespace: Option<&str>,
        with_separators: bool,
    );

    fn cancel(&self);

    fn deactivate(&self);

    fn deselect(&self);

    fn get_parent_shell(&self) -> Option<Widget>;

    fn get_selected_item(&self) -> Option<Widget>;

    fn get_take_focus(&self) -> bool;

    fn insert<P: IsA<Widget>>(&self, child: &P, position: i32);

    fn prepend<P: IsA<Widget>>(&self, child: &P);

    fn select_first(&self, search_sensitive: bool);

    fn select_item<P: IsA<Widget>>(&self, menu_item: &P);

    fn set_take_focus(&self, take_focus: bool);

    fn connect_activate_current<F: Fn(&Self, bool) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_activate_current(&self, force_hide: bool);

    fn connect_cancel<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_cancel(&self);

    fn connect_cycle_focus<F: Fn(&Self, DirectionType) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_cycle_focus(&self, direction: DirectionType);

    fn connect_deactivate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_insert<F: Fn(&Self, &Widget, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_move_current<F: Fn(&Self, MenuDirectionType) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn emit_move_current(&self, direction: MenuDirectionType);

    fn connect_move_selected<F: Fn(&Self, i32) -> glib::signal::Inhibit + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_selection_done<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_take_focus_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<MenuShell>> MenuShellExt for O {
    fn activate_item<P: IsA<Widget>>(&self, menu_item: &P, force_deactivate: bool) {
        unsafe {
            gtk_sys::gtk_menu_shell_activate_item(
                self.as_ref().to_glib_none().0,
                menu_item.as_ref().to_glib_none().0,
                force_deactivate.to_glib(),
            );
        }
    }

    fn append<P: IsA<MenuItem>>(&self, child: &P) {
        unsafe {
            gtk_sys::gtk_menu_shell_append(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    fn bind_model<P: IsA<gio::MenuModel>>(
        &self,
        model: Option<&P>,
        action_namespace: Option<&str>,
        with_separators: bool,
    ) {
        unsafe {
            gtk_sys::gtk_menu_shell_bind_model(
                self.as_ref().to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
                action_namespace.to_glib_none().0,
                with_separators.to_glib(),
            );
        }
    }

    fn cancel(&self) {
        unsafe {
            gtk_sys::gtk_menu_shell_cancel(self.as_ref().to_glib_none().0);
        }
    }

    fn deactivate(&self) {
        unsafe {
            gtk_sys::gtk_menu_shell_deactivate(self.as_ref().to_glib_none().0);
        }
    }

    fn deselect(&self) {
        unsafe {
            gtk_sys::gtk_menu_shell_deselect(self.as_ref().to_glib_none().0);
        }
    }

    fn get_parent_shell(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_menu_shell_get_parent_shell(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_selected_item(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_menu_shell_get_selected_item(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_take_focus(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_menu_shell_get_take_focus(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn insert<P: IsA<Widget>>(&self, child: &P, position: i32) {
        unsafe {
            gtk_sys::gtk_menu_shell_insert(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                position,
            );
        }
    }

    fn prepend<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            gtk_sys::gtk_menu_shell_prepend(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    fn select_first(&self, search_sensitive: bool) {
        unsafe {
            gtk_sys::gtk_menu_shell_select_first(
                self.as_ref().to_glib_none().0,
                search_sensitive.to_glib(),
            );
        }
    }

    fn select_item<P: IsA<Widget>>(&self, menu_item: &P) {
        unsafe {
            gtk_sys::gtk_menu_shell_select_item(
                self.as_ref().to_glib_none().0,
                menu_item.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_take_focus(&self, take_focus: bool) {
        unsafe {
            gtk_sys::gtk_menu_shell_set_take_focus(
                self.as_ref().to_glib_none().0,
                take_focus.to_glib(),
            );
        }
    }

    fn connect_activate_current<F: Fn(&Self, bool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_current_trampoline<P, F: Fn(&P, bool) + 'static>(
            this: *mut gtk_sys::GtkMenuShell,
            force_hide: glib_sys::gboolean,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MenuShell>,
        {
            let f: &F = &*(f as *const F);
            f(
                &MenuShell::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(force_hide),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate-current\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    activate_current_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_activate_current(&self, force_hide: bool) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut gobject_sys::GObject)
                .emit("activate-current", &[&force_hide])
                .unwrap()
        };
    }

    fn connect_cancel<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn cancel_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkMenuShell,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MenuShell>,
        {
            let f: &F = &*(f as *const F);
            f(&MenuShell::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cancel\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    cancel_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_cancel(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut gobject_sys::GObject)
                .emit("cancel", &[])
                .unwrap()
        };
    }

    fn connect_cycle_focus<F: Fn(&Self, DirectionType) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn cycle_focus_trampoline<P, F: Fn(&P, DirectionType) + 'static>(
            this: *mut gtk_sys::GtkMenuShell,
            direction: gtk_sys::GtkDirectionType,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MenuShell>,
        {
            let f: &F = &*(f as *const F);
            f(
                &MenuShell::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(direction),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cycle-focus\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    cycle_focus_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_cycle_focus(&self, direction: DirectionType) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut gobject_sys::GObject)
                .emit("cycle-focus", &[&direction])
                .unwrap()
        };
    }

    fn connect_deactivate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn deactivate_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkMenuShell,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MenuShell>,
        {
            let f: &F = &*(f as *const F);
            f(&MenuShell::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"deactivate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    deactivate_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_insert<F: Fn(&Self, &Widget, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn insert_trampoline<P, F: Fn(&P, &Widget, i32) + 'static>(
            this: *mut gtk_sys::GtkMenuShell,
            child: *mut gtk_sys::GtkWidget,
            position: libc::c_int,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MenuShell>,
        {
            let f: &F = &*(f as *const F);
            f(
                &MenuShell::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(child),
                position,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"insert\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    insert_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_move_current<F: Fn(&Self, MenuDirectionType) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn move_current_trampoline<P, F: Fn(&P, MenuDirectionType) + 'static>(
            this: *mut gtk_sys::GtkMenuShell,
            direction: gtk_sys::GtkMenuDirectionType,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MenuShell>,
        {
            let f: &F = &*(f as *const F);
            f(
                &MenuShell::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(direction),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"move-current\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    move_current_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_move_current(&self, direction: MenuDirectionType) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut gobject_sys::GObject)
                .emit("move-current", &[&direction])
                .unwrap()
        };
    }

    fn connect_move_selected<F: Fn(&Self, i32) -> glib::signal::Inhibit + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn move_selected_trampoline<
            P,
            F: Fn(&P, i32) -> glib::signal::Inhibit + 'static,
        >(
            this: *mut gtk_sys::GtkMenuShell,
            distance: libc::c_int,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean
        where
            P: IsA<MenuShell>,
        {
            let f: &F = &*(f as *const F);
            f(
                &MenuShell::from_glib_borrow(this).unsafe_cast_ref(),
                distance,
            )
            .to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"move-selected\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    move_selected_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_selection_done<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn selection_done_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkMenuShell,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MenuShell>,
        {
            let f: &F = &*(f as *const F);
            f(&MenuShell::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"selection-done\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    selection_done_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_take_focus_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_take_focus_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkMenuShell,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MenuShell>,
        {
            let f: &F = &*(f as *const F);
            f(&MenuShell::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::take-focus\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_take_focus_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for MenuShell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MenuShell")
    }
}
