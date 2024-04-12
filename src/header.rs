use crate::{MainWindow, Mode, NewsCard};
use reqwest::Error;
use slint::{self, Global, ModelRc, VecModel, Weak};

pub struct Header {
    pub articles: Vec<NewsCard>,
}

impl Header {
    fn new() -> Self {
        Self {
            articles: Vec::new(),
        }
    }
}

///Функция для парсинга сайта habr.com, берет первые 20 свежих новостей
pub async fn parsing_site(mywindow_weak: &Weak<MainWindow>) -> Result<(), Error> {
    let mut header = Header::new();
    let url = "https://habr.com/ru/feed/"; //с помощью библиотеки reqwest
    let response = reqwest::get(url).await?; //создаем асинхронный запрос к сайту
    let html_content = response.text().await?; //конвертируем в текст
    mywindow_weak
        .upgrade_in_event_loop(move |mywindow| {
            Mode::get(&mywindow).set_error(false);
        })
        .unwrap();
    //с помощью библиотеки scraper вытаскиваем необходимые элементы с html документа
    //все статьи помечены тегом <article>, в этом теге находим заголовок,
    //короткое описание, дату создания и адрес страницы
    let document = scraper::Html::parse_document(&html_content);
    let html_product_selector = scraper::Selector::parse("article.tm-articles-list__item").unwrap();
    let html_products = document.select(&html_product_selector);
    for html_product in html_products {
        let name = html_product
            .select(&scraper::Selector::parse("h2.tm-title.tm-title_h2").unwrap())
            .next()
            .map(|h2| h2.text().collect::<String>());
        let url = html_product
            .select(&scraper::Selector::parse("a.tm-title__link").unwrap())
            .next()
            .and_then(|a| a.value().attr("href"))
            .map(str::to_owned);
        let descr = html_product
            .select(&scraper::Selector::parse("div.article-formatted-body.article-formatted-body.article-formatted-body_version-2").unwrap())
            .next()
            .map(|desc| desc.text().collect::<String>());
        //когда найдены все элементы тега <article>, создаем карточку и
        //помещаем в вектор, пока не переберем все статьи
        let article = NewsCard {
            title: if let Some(name) = name {
                format!("▶ {name}").into()
            } else {
                "Post".into()
            },
            descr: if let Some(des) = descr {
                des.into()
            } else {
                "...".into()
            },
            url: if let Some(url) = url {
                format!("https://habr.com{url}").into()
            } else {
                "".into()
            },
        };
        header.articles.push(article);
    }
    mywindow_weak
        .upgrade_in_event_loop(move |mywindow| {
            //чтобы поместить вектор в массив скрипта .slint, создаем из вектора
            //вектор моделей, затем - модель с подсчетом ссылок
            mywindow.set_articles(ModelRc::new(VecModel::from(header.articles)));
            Mode::get(&mywindow).set_anim(false); //отключаем анимацию ожидания
        })
        .unwrap();
    Ok(())
}
