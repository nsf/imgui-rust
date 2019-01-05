fn main() {
    cc::Build::new()
        .cpp(true)
        .file("third-party/cimgui.cpp")
        .file("third-party/imgui/imgui.cpp")
        .file("third-party/imgui/imgui_demo.cpp")
        .file("third-party/imgui/imgui_draw.cpp")
        .file("third-party/imgui/imgui_widgets.cpp")
        .compile("libcimgui.a")
}
