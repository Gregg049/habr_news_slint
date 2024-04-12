#![windows_subsystem = "windows"]
use habr_news_slint::{parsing_site, MainWindow, Mode};
use slint::{self, ComponentHandle, Global, PhysicalPosition};

#[tokio::main]
async fn main() {
    let mywindow = MainWindow::new().unwrap();
    let position = slint::WindowPosition::Physical(PhysicalPosition { x: 800, y: 50 });
    mywindow.window().set_position(position);

    //создаем слабую ссылку, согласно документации Slint
    let mywindow_weak = mywindow.as_weak();
    //Создаем новую задачу, пока загружается контент выводится анимация
    slint::spawn_local(async move {
        if parsing_site(&mywindow_weak).await.is_err() {
            mywindow_weak
                .upgrade_in_event_loop(move |mywindow| {
                    Mode::get(&mywindow).set_anim(false); //выкл анимацию ожидания
                    Mode::get(&mywindow).set_error(true); //вкл анимацию ошибки
                })
                .unwrap();
        };
    })
    .unwrap();
    let mywindow_weak = mywindow.as_weak();
    //Функция обновления содержимого окна
    mywindow.on_update(move || {
        let mywindow_weak = mywindow_weak.clone();
        slint::invoke_from_event_loop(move || {
            slint::spawn_local(async move {
                //для запуска асинхронного кода
                let mywindow = mywindow_weak.unwrap();
                mywindow.set_opa(0.0); //устанавливаем окно полностью прозрачным
                Mode::get(&mywindow).set_error(false); // выкл анимацию ошибки
                Mode::get(&mywindow).set_anim(true); //запускаем анимацию ожидания
                if parsing_site(&mywindow_weak).await.is_err() {
                    Mode::get(&mywindow).set_anim(false); //выкл анимацию ожидания
                    Mode::get(&mywindow).set_error(true); //вкл анимацию ошибки
                } else {
                    mywindow.set_opa(1.0)
                }; //делаем окно снова видимым
            })
            .unwrap();
        })
        .unwrap();
    });
    //Функция для запуска браузера по ссылке
    mywindow.on_handle_url(move |url| {
        if webbrowser::open(url.as_str()).is_err() {
            println!("Not found browser Chrom");
        };
    });
    //Закрытие и выход из приложения
    let mywindow_weak = mywindow.as_weak();
    mywindow.on_close(move || {
        let mywindow = mywindow_weak.unwrap();
        let _ = mywindow.hide();
    });
    mywindow.run().unwrap();
}
