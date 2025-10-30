use crate::data::{get_question_and_answer, CATEGORIES};
#[cfg(feature = "ssr")]
use crate::schema::answered_questions::dsl::*;
#[cfg(feature = "ssr")]
use diesel::prelude::*;
#[cfg(feature = "ssr")]
use diesel::r2d2::{ConnectionManager, Pool};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use leptos_router::path;
use leptos_sweetalert::*;
#[cfg(feature = "hydrate")]
use web_sys::window;

#[derive(Debug, Clone, thiserror::Error, serde::Serialize, serde::Deserialize)]
pub enum AppError {
    #[error("Database error: {0}")]
    DbError(String),
    #[error("HTTP error: {0}")]
    HttpError(String),
    #[error("Authorization error: {0}")]
    AuthError(String),
    #[error("Server framework error: {0}")]
    ServerFnError(#[from] ServerFnErrorErr),
}

impl leptos::server_fn::error::FromServerFnError for AppError {
    type Encoder = leptos::server_fn::codec::JsonEncoding;

    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        AppError::ServerFnError(value)
    }
}

#[cfg(feature = "ssr")]
pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

/// Fetches all answeerd (category, question) pairs from the database.
#[server(GetAnswered, "/api")]
pub async fn get_answered() -> Result<Vec<(usize, usize)>, AppError> {
    let pool: DbPool = expect_context();
    let mut conn = pool.get().map_err(|e| AppError::DbError(e.to_string()))?;
    let results: Vec<(i32, i32)> = answered_questions
        .select((category, question))
        .load(&mut conn)
        .map_err(|e| AppError::DbError(e.to_string()))?;
    Ok(results
        .into_iter()
        .map(|(c, q)| (c as usize, q as usize))
        .collect())
}

/// Marks a question as answered.
#[server(MarkAnswered, "/api")]
pub async fn mark_answered(cat: usize, ques: usize) -> Result<(), AppError> {
    let pool: DbPool = expect_context();
    let mut conn = pool.get().map_err(|e| AppError::DbError(e.to_string()))?;
    diesel::insert_or_ignore_into(answered_questions)
        .values((category.eq(cat as i32), question.eq(ques as i32)))
        .execute(&mut conn)
        .map_err(|e| AppError::DbError(e.to_string()))?;
    leptos::logging::log!("Marked {}:{} as answered", cat, ques);
    Ok(())
}

// Resets all state by deleting answered questions.
#[server(ResetGame, "/api")]
pub async fn reset_game() -> Result<(), AppError> {
    let pool: DbPool = expect_context();
    let mut conn = pool.get().map_err(|e| AppError::DbError(e.to_string()))?;
    diesel::delete(answered_questions)
        .execute(&mut conn)
        .map_err(|e| AppError::DbError(e.to_string()))?;
    leptos::logging::log!("Reset successful!");
    Ok(())
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <link rel="preconnect" href="https://fonts.googleapis.com" />
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
                <link
                    href="https://fonts.googleapis.com/css2?family=Bangers&display=swap"
                    rel="stylesheet"
                />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/hp-trivia.css" />

        // sets the document title
        <Title text="Harry Potter Trivia" />

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("/") view=GridPage />
                    <Route path=path!("/question/:category/:question") view=QuestionPage />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn GridPage() -> impl IntoView {
    let answered_resource =
        LocalResource::new(|| async { get_answered().await.ok().unwrap_or_default() });
    let gryffindor_score = RwSignal::new(0i32);
    let hufflepuff_score = RwSignal::new(0i32);
    let ravenclaw_score = RwSignal::new(0i32);
    let slytherin_score = RwSignal::new(0i32);

    let confirm_and_reset = move |_| {
        Swal::fire(SwalOptions {
            title: "WARNING",
            text: "This action cannot be undone. Are you sure you want to reset the game?",
            icon: SwalIcon::WARNING,
            confirm_button_text: "Yes",
            deny_button_text: "No",
            show_confirm_button: true,
            show_deny_button: true,
            show_cancel_button: true,
            pre_confirm: || {
                spawn_local(async move {
                    let _ = reset_game().await;
                });
                // Reload page to refetch answered_resource and update grid.
                #[cfg(feature = "hydrate")]
                {
                    if let Some(w) = window() {
                        let _ = w.location().reload();
                    }
                }
                Swal::close(Some(SwalResult::confirmed()));
            },
            ..SwalOptions::default()
        });
    };

    let is_answered = move |col: usize, row: usize, answered: &[(usize, usize)]| {
        answered.iter().any(|&(c, q)| c == col && q == row)
    };

    view! {
        <div class="main-container">
            <div class="left-container">
                <div class="house-section">
                    <div class="house-header-text">"Gryffindor"</div>
                    <div class="score-controls">
                        <div class="button-stack">
                            <button class="score-btn" on:click=move |_| gryffindor_score.update(|s| *s -= 5)>"-5"</button>
                            <button class="score-btn" on:click=move |_| gryffindor_score.update(|s| *s -= 10)>"-10"</button>
                        </div>
                        <div class="score-display">
                            <span class="score-text">{move || gryffindor_score.get().to_string()}</span>
                        </div>
                        <div class="button-stack">
                            <button class="score-btn" on:click=move |_| gryffindor_score.update(|s| *s += 5)>"+5"</button>
                            <button class="score-btn" on:click=move |_| gryffindor_score.update(|s| *s += 10)>"+10"</button>
                        </div>
                    </div>
                </div>
                <div class="house-section">
                    <div class="house-header-text">"Hufflepuff"</div>
                    <div class="score-controls">
                        <div class="button-stack">
                            <button class="score-btn" on:click=move |_| hufflepuff_score.update(|s| *s -= 5)>"-5"</button>
                            <button class="score-btn" on:click=move |_| hufflepuff_score.update(|s| *s -= 10)>"-10"</button>
                        </div>
                        <div class="score-display">
                            <span class="score-text">{move || hufflepuff_score.get().to_string()}</span>
                        </div>
                        <div class="button-stack">
                            <button class="score-btn" on:click=move |_| hufflepuff_score.update(|s| *s += 5)>"+5"</button>
                            <button class="score-btn" on:click=move |_| hufflepuff_score.update(|s| *s += 10)>"+10"</button>
                        </div>
                    </div>
                </div>
            </div>
        <div class="grid-wrapper">
            <div class="grid-container">
                <button class="reset-btn" on:click=confirm_and_reset>
                    "Reset Game"
                </button>

                <table class="jeopardy-grid">
                    <thead>
                        <tr>
                            {CATEGORIES
                                .iter()
                                .enumerate()
                                .map(|(i, c)| {
                                    let col_class = format!("category-header category-{}", i);
                                    view! { <th class=col_class>{c.0}</th> }
                                })
                                .collect_view()}
                        </tr>
                    </thead>
                    <tbody>
                        {(0..6)
                            .map(|row| {
                                view! {
                                    <tr class="point-row">
                                        {(0..5)
                                            .map(|col| {
                                                let points = (row + 1) * 10;
                                                let cell_class = move || {
                                                    let maybe_answered = answered_resource.read().clone();
                                                    maybe_answered
                                                        .map_or_else(
                                                            || format!("cell category-{}", col),
                                                            |answered| {
                                                                if is_answered(col, row, &answered) {
                                                                    "cell answered".to_string()
                                                                } else {
                                                                    format!("cell category-{}", col)
                                                                }
                                                            },
                                                        )
                                                };
                                                view! {
                                                    <td class=cell_class>
                                                        <a
                                                            class="cell-link"
                                                            href=format!("/question/{}/{}", col, row)
                                                        >
                                                            {points}
                                                        </a>
                                                    </td>
                                                }
                                            })
                                            .collect_view()}
                                    </tr>
                                }
                            })
                            .collect_view()}
                    </tbody>
                </table>
            </div>
        </div>
            <div class="right-container">
                <div class="house-section">
                    <div class="house-header-text">"Ravenclaw"</div>
                    <div class="score-controls">
                        <div class="button-stack">
                            <button class="score-btn" on:click=move |_| ravenclaw_score.update(|s| *s -= 5)>"-5"</button>
                            <button class="score-btn" on:click=move |_| ravenclaw_score.update(|s| *s -= 10)>"-10"</button>
                        </div>
                        <div class="score-display">
                            <span class="score-text">{move || ravenclaw_score.get().to_string()}</span>
                        </div>
                        <div class="button-stack">
                            <button class="score-btn" on:click=move |_| ravenclaw_score.update(|s| *s += 5)>"+5"</button>
                            <button class="score-btn" on:click=move |_| ravenclaw_score.update(|s| *s += 10)>"+10"</button>
                        </div>
                    </div>
                </div>
                <div class="house-section">
                    <div class="house-header-text">"Slytherin"</div>
                    <div class="score-controls">
                        <div class="button-stack">
                            <button class="score-btn" on:click=move |_| slytherin_score.update(|s| *s -= 5)>"-5"</button>
                            <button class="score-btn" on:click=move |_| slytherin_score.update(|s| *s -= 10)>"-10"</button>
                        </div>
                        <div class="score-display">
                            <span class="score-text">{move || slytherin_score.get().to_string()}</span>
                        </div>
                        <div class="button-stack">
                            <button class="score-btn" on:click=move |_| slytherin_score.update(|s| *s += 5)>"+5"</button>
                            <button class="score-btn" on:click=move |_| slytherin_score.update(|s| *s += 10)>"+10"</button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[derive(leptos::Params, PartialEq, Clone)]
struct QuestionParams {
    category: Option<usize>,
    question: Option<usize>,
}

#[component]
fn QuestionPage() -> impl IntoView {
    let params = use_params::<QuestionParams>();

    let category_idx = Memo::new(move |_| {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|p| p.category)
            .unwrap_or(0usize)
    });

    let question_idx = Memo::new(move |_| {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|p| p.question)
            .unwrap_or(0usize)
    });

    let (q_text, a_text) =
        get_question_and_answer(category_idx.get_untracked(), question_idx.get_untracked());

    let revealed = RwSignal::new(false);

    Effect::new(move |_| {
        let c = category_idx.get_untracked();
        let q = question_idx.get_untracked();
        leptos::logging::log!("Effect mounted: About to mark answered for {}:{}", c, q,);
        spawn_local(async move {
            match mark_answered(c, q).await {
                Ok(_) => {
                    leptos::logging::log!("Marked {}:{} complete", c, q)
                }
                Err(e) => leptos::logging::error!(
                    "Failed to mark {}:{} complete: {}",
                    c,
                    q,
                    e.to_string()
                ),
            }
        });
    });

    view! {
        <div class="question-container">
            <h2 class="question-text">{q_text}</h2>
            <Show
                when=move || revealed.get()
                fallback=|| view! { <p class="reveal-prompt">"Click to reveal..."</p> }
            >
                <div class="answer-section">
                    <strong class="answer-text">"Answer: " {a_text}</strong>
                </div>
            </Show>
            <button class="reveal-btn" on:click=move |_| revealed.set(true)>
                "Answer"
            </button>
            <a class="back-link" href="/">
                "Back"
            </a>
        </div>
    }
}
