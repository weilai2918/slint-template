use slint::{Weak, ComponentHandle,WindowPosition::Logical,LogicalPosition, WindowSize, LogicalSize};
use crate::{Main,BarGlobal};
use i_slint_backend_winit::WinitWindowAccessor;

pub fn system_window(weak: Weak<Main>){
    let win = weak.unwrap();
    let win_weak = weak.clone().unwrap();
    
    //关闭程序
    win.global::<BarGlobal>().on_close(move ||{
        let _ = win_weak.window().hide();
    });

    //移动窗体
    let win_weak = weak.clone().unwrap();
    win.global::<BarGlobal>().on_move(move |x:f32,y:f32|{
        let logical_pos = win_weak.window().position().to_logical(win_weak.window().scale_factor());
        win_weak.window().set_position(Logical(LogicalPosition{
            x:logical_pos.x + x,
            y:logical_pos.y + y
        }));
    });

    let win_weak_copy = weak.clone().unwrap();
    weak.unwrap().global::<BarGlobal>().on_min_win(move ||{
        win_weak_copy.window().with_winit_window(|winit_window: &winit::window::Window| {
            winit_window.set_minimized(true);
        });
    });
    let win_weak_copy = weak.clone().unwrap();
    weak.unwrap().global::<BarGlobal>().on_max_win(move ||{
        win_weak_copy.window().with_winit_window(|winit_window: &winit::window::Window| {
            if winit_window.is_maximized() {
                winit_window.set_maximized(false);
            }else{
                winit_window.set_maximized(true);
            }
            
        });
    });

    let win_weak_copy = weak.clone().unwrap();
    weak.unwrap().global::<BarGlobal>().on_size_win(move |x:f32,y:f32,movex:f32,movey:f32,mx:f32,my:f32|{
        if mx <= 8. && my <= 8. {
            win_weak_copy.window().set_size(WindowSize::Logical(LogicalSize::new(x + movex,y + movey)));
        }
    })
}


