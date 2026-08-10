use super::Project;
use leptos::prelude::*;

pub const PROJECT: Project = Project {
    name: "Хлебкрафт",
    slug: "hlebcraft",
    desctiption: "Приватный Ванильный Minecraft Сервер",
    icon: "assets/images/hlebcraft_icon.png",
    view_link: None,
    read_component: || HlebcraftRead().into_any().into_view(),
};

#[component]
pub fn HlebcraftRead() -> impl IntoView {
    view! {
        <p>
            "Хлебкрафт был одним из самых масштабных проектов, в которых я участвовал.
            Всё начиналось как обычный локальный сервер среди друзей - сначала на Aternos,
            потом был арендован отдельный хостинг. Игроков было немного, максимум человек 15,
            и особой известности сервер не получил - играли, по сути, только свои."
        </p>

        <p>
            "Первый сезон продлился примерно полгода - пока наш главный технический администратор
            не пропал на целый год. Когда он наконец вернулся, мы решили запустить второй сезон.
            На этот раз я присоединился уже как второй технический администратор.
            Сервер перестал быть чисто локальным: мы начали его рекламировать, и постепенно
            стали приходить новые люди. Второй сезон, как и первый, продержался около полугода.
            В какой-то момент всем стало скучно, интерес угас, и сервер отключили."
        </p>

        <div class="section-grid">
            <img src="/assets/images/hlebcraft_screenshot_1.webp" />
            <img src="/assets/images/hlebcraft_screenshot_2.webp" />
            <img src="/assets/images/hlebcraft_screenshot_3.webp" />
            <img src="/assets/images/hlebcraft_screenshot_4.webp" />
        </div>

        <p>
            "Спустя год мы взялись за третий сезон - уже гораздо серьёзнее, чем раньше.
            Я разработал для проекта сайт с системой продажи проходок и автоматической регистрацией
            на сервере после оплаты. Завели отдельный Discord-сервер, делали рекламные ролики для TikTok -
            в общем, подошли к делу как к настоящему проекту."
        </p>

        <img src="/assets/images/hlebcraft_website.webp" />

        <div class="section-grid">
            <img src="/assets/images/hlebcraft_screenshot_6.webp" />
            <img src="/assets/images/hlebcraft_screenshot_7.webp" />
            <img src="/assets/images/hlebcraft_screenshot_5.webp" />
            <img src="/assets/images/hlebcraft_screenshot_8.webp" />
        </div>

        <p>
            "К сожалению, третий сезон завершился довольно быстро - из-за разногласий внутри
            команды и финансовых трудностей. Появится ли когда-нибудь четвёртый сезон Хлебкрафта,
            я не знаю. Но в любом случае я рад, что всё это случилось. Это мой самый любимый проект,
            я ценю каждый этап, пройденный с командой и участие игроков в жизни этого сервера."
        </p>

        <img src="/assets/images/hlebcraft_end.png" />
    }
}
