use crate::{BarGlobal, Main, OperateEnum};
use i_slint_backend_winit::WinitWindowAccessor;
use slint::{
    ComponentHandle, LogicalPosition, LogicalSize, Weak, WindowPosition::Logical, WindowSize,
};


pub fn system_window(weak: Weak<Main>) {
    let win = weak.unwrap();
    let win_weak = weak.clone().unwrap();

    //关闭程序
    win.global::<BarGlobal>()
        .on_window_operate(move |operate| match operate {
            OperateEnum::Minimize => {
                win_weak
                    .window()
                    .with_winit_window(|winit_window: &winit::window::Window| {
                        winit_window.set_minimized(true);
                    });
            }
            OperateEnum::Maximize => {
                win_weak
                    .window()
                    .with_winit_window(|winit_window: &winit::window::Window| {
                        if winit_window.is_maximized() {
                            winit_window.set_maximized(false);
                        } else {
                            winit_window.set_maximized(true);
                        }
                    });
            }
            OperateEnum::Close => {
                win_weak
                    .window()
                    .with_winit_window(|winit_window: &winit::window::Window| {
                        winit_window.set_visible(false);
                    });
            }
        });

    //移动窗体
    let win_weak = weak.clone().unwrap();
    win.global::<BarGlobal>().on_move(move |x: f32, y: f32| {
        let logical_pos = win_weak
            .window()
            .position()
            .to_logical(win_weak.window().scale_factor());
        win_weak.window().set_position(Logical(LogicalPosition {
            x: logical_pos.x + x,
            y: logical_pos.y + y,
        }));
    });

    let win_weak_copy = weak.clone().unwrap();
    weak.unwrap().global::<BarGlobal>().on_size_win(
        move |x: f32, y: f32, movex: f32, movey: f32, mx: f32, my: f32| {
            if mx <= 8. && my <= 8. {
                win_weak_copy
                    .window()
                    .set_size(WindowSize::Logical(LogicalSize::new(x + movex, y + movey)));
            }
        },
    );
}
