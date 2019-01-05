#!/bin/bash
node generate.js raw | rustfmt > imgui-raw/imguiraw.rs
node generate.js safe | rustfmt > imgui/auto.rs
