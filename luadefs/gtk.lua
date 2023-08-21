---@diagnostic disable:missing-return
---@diagnostic disable:lowercase-global
---@diagnostic disable:unused-local
gtk                        = {}

---@enum Orientation
gtk.Orientation            = {
    Horizontal = 0,
    Vertical = 1,
}

---@enum Align
gtk.Align                  = {
    Fill = 0,
    Start = 1,
    End = 2,
    Center = 3,
    Baseline = 4,
}

---@class Priority
local Priority             = {}

gtk.Priority               = {
    ---@type Priority
    HIGH = nil,
    ---@type Priority
    DEFAULT = nil,
    ---@type Priority
    HIGH_IDLE = nil,
    ---@type Priority
    DEFAULT_IDLE = nil,
    ---@type Priority
    LOW = nil,
}

---@enum RevealerTransitionType
gtk.RevealerTransitionType = {
    None = 0,
    Crossfade = 1,
    SlideRight = 2,
    SlideLeft = 3,
    SlideUp = 4,
    SlideDown = 5,
    SwingRight = 6,
    SwingLeft = 7,
    SwingUp = 8,
    SwingDown = 9
}

---@class MainContext
gtk.MainContext            = {
    ---@return MainContext
    default = function() end,

    ---@param self MainContext
    ---@param callback fun():nil
    spawn_local = function(self, callback) end

    ---@param self MainContext
    ---@param priority Priority
    ---@param callback fun():nil
    spawn_local_with_priority = function(self, priority, callback) end
}

---@class ApplicationFlagsCtor
---@field is_service boolean?
---@field is_launcher boolean?
---@field handles_open boolean?
---@field handles_command_line boolean?
---@field send_environment boolean?
---@field non_unique boolean?
---@field can_override_app_id boolean?
---@field allow_replacement boolean?
---@field replace boolean?

---@class ApplicationFlags
gtk.ApplicationFlags       = {
    ---@param flags ApplicationFlagsCtor
    ---@return ApplicationFlags
    new = function(flags) end
}

---@class Application
gtk.Application            = {
    ---@param id string
    ---@param flags ApplicationFlags
    ---@return Application
    new = function(id, flags) end,

    ---@param self Application
    ---@param callback fun():nil
    connect_activate = function(self, callback) end,

    ---@param self Application
    ---@param callback fun():nil
    connect_startup = function(self, callback) end,

    ---@param self Application
    ---@param callback fun():nil
    connect_shutdown = function(self, callback) end,

    ---@param self Application
    run = function(self) end
}

---@class WidgetImpl
local WidgetImpl           = {
    ---@param self WidgetImpl
    ---@return Widget
    upcast = function(self) end,

    ---@param self WidgetImpl
    ---@param visible boolean
    ---@return Widget
    set_visible = function(self, visible) end,

    ---@param self WidgetImpl
    ---@return boolean
    get_visible = function(self) end,

    ---@param self WidgetImpl
    ---@param class string
    set_css_class = function(self, class) end,

    ---@param self WidgetImpl
    ---@param classes string[]
    set_css_classes = function(self, classes) end,

    ---@param self WidgetImpl
    ---@param class string
    add_css_class = function(self, class) end,

    ---@param self WidgetImpl
    ---@param class string
    remove_css_class = function(self, class) end,

    ---@param self WidgetImpl
    ---@param align Align
    set_valign = function(self, align) end,

    ---@param self WidgetImpl
    ---@param align Align
    set_halign = function(self, align) end,

    ---@param self WidgetImpl
    ---@param width integer
    ---@param height integer
    set_size_request = function(self, width, height) end,

    ---@param self WidgetImpl
    ---@return integer
    allocated_width = function(self) end,

    ---@param self WidgetImpl
    ---@return integer
    allocated_height = function(self) end,
}

---@class Widget
local Widget               = {}

---@class ApplicationWindow : WidgetImpl
gtk.ApplicationWindow      = {
    ---@param app Application
    ---@return ApplicationWindow
    new = function(app) end,

    ---@param self ApplicationWindow
    ---@param title string?
    set_title = function(self, title) end,

    ---@param self ApplicationWindow
    ---@param child Widget?
    set_child = function(self, child) end,

    ---@param self ApplicationWindow
    close = function(self) end,

    ---@param self ApplicationWindow
    present = function(self) end
}

---@class Box : WidgetImpl
gtk.Box                    = {
    ---@param orientation Orientation
    ---@param spacing number?
    ---@return Box
    new = function(orientation, spacing) end,

    ---@param self Box
    ---@param widget Widget
    append = function(self, widget) end,

    ---@param self Box
    ---@param widget Widget
    remove = function(self, widget) end
}
---@class Grid : WidgetImpl
gtk.Grid                   = {
    ---@return Grid
    new = function() end,

    ---@param self Grid
    ---@param widget Widget
    ---@param column integer
    ---@param row integer
    ---@param width integer
    ---@param height integer
    attach = function(self, widget, column, row, width, height) end,

    ---@param self Grid
    ---@param widget Widget
    remove = function(self, widget) end
}

---@class CenterBox : WidgetImpl
gtk.CenterBox              = {
    ---@return CenterBox
    new = function() end,

    ---@param self CenterBox
    ---@param widget? Widget
    set_start_widget = function(self, widget) end,

    ---@param self CenterBox
    ---@param widget? Widget
    set_center_widget = function(self, widget) end,

    ---@param self CenterBox
    ---@param widget? Widget
    set_end_widget = function(self, widget) end
}

---@class Button : WidgetImpl
gtk.Button                 = {
    ---@return Button
    new = function() end,

    ---@param label string
    ---@return Button
    with_label = function(label) end,

    ---@param self Button
    ---@param callback fun():nil
    connect_clicked = function(self, callback) end,

    ---@param self Button
    ---@param label string
    set_label = function(self, label) end
}

---@class CheckButton : WidgetImpl
gtk.CheckButton            = {
    ---@return CheckButton
    new              = function() end,

    ---@param label string
    ---@return CheckButton
    with_label       = function(label) end,

    ---@param self CheckButton
    ---@param callback fun():nil
    connect_toggled  = function(self, callback) end,

    ---@param self CheckButton
    ---@param setting boolean
    set_active       = function(self, setting) end,

    ---@param self CheckButton
    ---@param child Widget?
    set_child        = function(self, child) end,

    ---@param self CheckButton
    ---@param group CheckButton?
    set_group        = function(self, group) end,

    ---@param self CheckButton
    ---@param inconsistent boolean
    set_inconsistent = function(self, inconsistent) end,

    ---@param self CheckButton
    ---@param label string?
    set_label        = function(self, label) end,
}

---@class Label : WidgetImpl
gtk.Label                  = {
    ---@param str? string
    ---@return Label
    new = function(str) end,

    ---@param self Label
    ---@param str string
    set_label = function(self, str) end,

    ---@param self Label
    ---@param markup string
    set_markup = function(self, markup) end
}

---@class Image : WidgetImpl
gtk.Image                  = {
    ---@return Image
    new = function() end,

    ---@param path string
    ---@return Image
    from_file = function(path) end,

    ---@param icon_name string
    ---@return Image
    from_icon_name = function(icon_name) end,

    ---@param self Image
    ---@param pixel_size integer
    set_pixel_size = function(self, pixel_size) end,

    ---@param self Image
    ---@param path string
    set_from_file = function(self, path) end,

    ---@param self Image
    ---@param icon_name string
    set_from_icon_name = function(self, icon_name) end,
}

---@class Revealer : WidgetImpl
gtk.Revealer               = {
    ---@return Revealer
    new = function() end,

    ---@param self Revealer
    ---@param child Widget
    set_child = function(self, child) end,

    ---@param self Revealer
    ---@param reveal_child boolean
    set_reveal_child = function(self, reveal_child) end,

    ---@param self Revealer
    ---@param duration integer
    set_transition_duration = function(self, duration) end,

    ---@param self Revealer
    ---@param transition RevealerTransitionType
    set_transition_type = function(self, transition) end,
}

---@class CssProvider
gtk.CssProvider            = {
    ---@return CssProvider
    new = function() end,

    ---@param self CssProvider
    ---@param data string
    load_from_data = function(self, data) end,

    ---@param self CssProvider
    ---@param path string
    load_from_file = function(self, path) end,
}

---@param provider CssProvider
function gtk.style_context_add_provider(provider) end

gtk.layer_shell = {}

---@enum Layer
gtk.layer_shell.Layer = {
    Background = 0,
    Bottom = 1,
    Top = 2,
    Overlay = 3
}

---@enum Edge
gtk.layer_shell.Edge = {
    Left = 0,
    Right = 1,
    Top = 2,
    Bottom = 3
}

---@param window ApplicationWindow
function gtk.layer_shell.init_for_window(window) end

---@param window ApplicationWindow
---@param layer Layer
function gtk.layer_shell.set_layer(window, layer) end

---@param window ApplicationWindow
function gtk.layer_shell.auto_exclusive_zone_enable(window) end

---@param window ApplicationWindow
---@param exclusive_zone integer
function gtk.layer_shell.set_exclusive_zone(window, exclusive_zone) end

---@param window ApplicationWindow
---@param edge Edge
---@param margin_size integer
function gtk.layer_shell.set_margin(window, edge, margin_size) end

---@param window ApplicationWindow
---@param edge Edge
---@param anchor_to_edge boolean
function gtk.layer_shell.set_anchor(window, edge, anchor_to_edge) end
