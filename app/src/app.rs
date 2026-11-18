use crate::auth::{
    create_family::{CreateFamilyEvent, CreateFamilyView},
    family_setup::{FamilySetupEvent, FamilySetupView},
    join_family::{JoinFamilyEvent, JoinFamilyView},
    login::{LoginEvent, LoginView},
    register::{RegisterEvent, RegisterView},
    terms::{TermsEvent, TermsView},
};
use crate::brand::*;
use crate::dashboard::{DashboardEvent, DashboardView};
use crate::i18n::Language;
use crate::navigation::{self, AuthScreen, SavedNavigation};
use gpui_kit::*;
use rusqlite::Connection;

pub struct UmuryangoApp {
    theme: ThemeMode,
    language: Language,
    screen: AuthScreen,
    member_name: Option<String>,
    connection: Connection,
    login_view: Entity<LoginView>,
    register_view: Entity<RegisterView>,
    terms_view: Entity<TermsView>,
    family_setup_view: Entity<FamilySetupView>,
    create_family_view: Entity<CreateFamilyView>,
    join_family_view: Entity<JoinFamilyView>,
    dashboard_view: Entity<DashboardView>,
    _login_subscription: Subscription,
    _register_subscription: Subscription,
    _terms_subscription: Subscription,
    _family_setup_subscription: Subscription,
    _create_family_subscription: Subscription,
    _join_family_subscription: Subscription,
    _dashboard_subscription: Subscription,
}

impl UmuryangoApp {
    pub fn new(connection: Connection, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let theme = ThemeMode::Dark;
        let language = Language::default();
        crate::auth::client::set_current_language(language.locale_code());

        let saved = navigation::load(&connection);
        if saved.token.is_some() {
            crate::auth::client::set_current_token(saved.token.clone());
        }

        let login_view = cx.new(|cx| LoginView::new(window, theme, language, cx));
        let register_view = cx.new(|cx| RegisterView::new(window, theme, language, cx));
        let terms_view = cx.new(|cx| TermsView::new(theme, language, cx));
        let family_setup_view = cx.new(|cx| FamilySetupView::new(theme, language, cx));
        let create_family_view = cx.new(|cx| CreateFamilyView::new(window, theme, language, cx));
        let join_family_view = cx.new(|cx| JoinFamilyView::new(window, theme, language, cx));
        let dashboard_view = cx.new(|cx| DashboardView::new(window, theme, language, cx));

        if saved.screen.requires_session() {
            let name = saved.member_name.clone().unwrap_or_default();
            family_setup_view.update(cx, |view, cx| {
                view.set_member_name(name.clone(), cx);
            });
            dashboard_view.update(cx, |view, cx| {
                view.set_member_name(name, cx);
            });
        }

        let login_subscription = cx.subscribe(
            &login_view,
            |this, _view, event: &LoginEvent, cx| match event {
                LoginEvent::ThemeChanged(theme) => {
                    this.theme = *theme;
                    cx.notify();
                }
                LoginEvent::LanguageChanged(language) => {
                    this.language = *language;
                    crate::auth::client::set_current_language(language.locale_code());
                    cx.notify();
                }
                LoginEvent::RegisterRequested => this.set_screen(AuthScreen::Register, cx),
                LoginEvent::SignedIn { display_name, .. } => {
                    let name = display_name.clone();
                    this.member_name = Some(name.clone());
                    this.dashboard_view.update(cx, |view, cx| {
                        view.set_member_name(name, cx);
                    });
                    this.set_screen(AuthScreen::Dashboard, cx);
                }
            },
        );

        let register_subscription = cx.subscribe(
            &register_view,
            |this, _view, event: &RegisterEvent, cx| match event {
                RegisterEvent::ThemeChanged(theme) => {
                    this.theme = *theme;
                    cx.notify();
                }
                RegisterEvent::LanguageChanged(language) => {
                    this.language = *language;
                    crate::auth::client::set_current_language(language.locale_code());
                    cx.notify();
                }
                RegisterEvent::LoginRequested => this.set_screen(AuthScreen::Login, cx),
                RegisterEvent::TermsRequested => this.set_screen(AuthScreen::Terms, cx),
                RegisterEvent::Registered { display_name, .. } => {
                    let name = display_name.clone();
                    this.member_name = Some(name.clone());
                    this.family_setup_view.update(cx, |view, cx| {
                        view.set_member_name(name, cx);
                    });
                    this.set_screen(AuthScreen::FamilySetup, cx);
                }
            },
        );

        let terms_subscription = cx.subscribe(
            &terms_view,
            |this, _view, event: &TermsEvent, cx| match event {
                TermsEvent::ThemeChanged(theme) => {
                    this.theme = *theme;
                    cx.notify();
                }
                TermsEvent::LanguageChanged(language) => {
                    this.language = *language;
                    crate::auth::client::set_current_language(language.locale_code());
                    cx.notify();
                }
                TermsEvent::Accepted => {
                    this.register_view.update(cx, |view, cx| {
                        view.accept_terms(cx);
                    });
                    this.set_screen(AuthScreen::Register, cx);
                }
                TermsEvent::BackRequested => this.set_screen(AuthScreen::Register, cx),
            },
        );

        let family_setup_subscription = cx.subscribe(
            &family_setup_view,
            |this, _view, event: &FamilySetupEvent, cx| match event {
                FamilySetupEvent::ThemeChanged(theme) => {
                    this.theme = *theme;
                    cx.notify();
                }
                FamilySetupEvent::LanguageChanged(language) => {
                    this.language = *language;
                    crate::auth::client::set_current_language(language.locale_code());
                    cx.notify();
                }
                FamilySetupEvent::CreateFamilyRequested => {
                    this.set_screen(AuthScreen::CreateFamily, cx)
                }
                FamilySetupEvent::JoinFamilyRequested => {
                    this.set_screen(AuthScreen::JoinFamily, cx)
                }
                FamilySetupEvent::SkipRequested => {
                    this.set_screen(AuthScreen::Dashboard, cx);
                }
                FamilySetupEvent::FamilyJoined { .. } => {
                    this.set_screen(AuthScreen::Dashboard, cx);
                }
                FamilySetupEvent::LoginRequested => this.set_screen(AuthScreen::Login, cx),
                FamilySetupEvent::ChoiceSelected(_) => {
                    cx.notify();
                }
            },
        );

        let create_family_subscription = cx.subscribe(
            &create_family_view,
            |this, _view, event: &CreateFamilyEvent, cx| match event {
                CreateFamilyEvent::ThemeChanged(theme) => {
                    this.theme = *theme;
                    cx.notify();
                }
                CreateFamilyEvent::LanguageChanged(language) => {
                    this.language = *language;
                    crate::auth::client::set_current_language(language.locale_code());
                    cx.notify();
                }
                CreateFamilyEvent::BackRequested => this.set_screen(AuthScreen::FamilySetup, cx),
                CreateFamilyEvent::FamilyCreated { .. } => {
                    this.set_screen(AuthScreen::Dashboard, cx);
                }
            },
        );

        let join_family_subscription = cx.subscribe(
            &join_family_view,
            |this, _view, event: &JoinFamilyEvent, cx| match event {
                JoinFamilyEvent::ThemeChanged(theme) => {
                    this.theme = *theme;
                    cx.notify();
                }
                JoinFamilyEvent::LanguageChanged(language) => {
                    this.language = *language;
                    crate::auth::client::set_current_language(language.locale_code());
                    cx.notify();
                }
                JoinFamilyEvent::BackRequested => this.set_screen(AuthScreen::FamilySetup, cx),
                JoinFamilyEvent::FamilyJoined { .. } => {
                    this.set_screen(AuthScreen::Dashboard, cx);
                }
            },
        );

        let dashboard_subscription = cx.subscribe(
            &dashboard_view,
            |this, _view, event: &DashboardEvent, cx| match event {
                DashboardEvent::ThemeChanged(theme) => {
                    this.theme = *theme;
                    cx.notify();
                }
                DashboardEvent::LanguageChanged(language) => {
                    this.language = *language;
                    crate::auth::client::set_current_language(language.locale_code());
                    cx.notify();
                }
                DashboardEvent::SignOutRequested => {
                    this.member_name = None;
                    crate::auth::client::set_current_token(None);
                    this.set_screen(AuthScreen::Login, cx);
                }
                DashboardEvent::RecordContributionRequested
                | DashboardEvent::AddExpenseRequested
                | DashboardEvent::NewProjectRequested
                | DashboardEvent::SyncRequested => {
                    cx.notify();
                }
            },
        );

        Self {
            theme,
            language,
            screen: saved.screen,
            member_name: saved.member_name,
            connection,
            login_view,
            register_view,
            terms_view,
            family_setup_view,
            create_family_view,
            join_family_view,
            dashboard_view,
            _login_subscription: login_subscription,
            _register_subscription: register_subscription,
            _terms_subscription: terms_subscription,
            _family_setup_subscription: family_setup_subscription,
            _create_family_subscription: create_family_subscription,
            _join_family_subscription: join_family_subscription,
            _dashboard_subscription: dashboard_subscription,
        }
    }

    /// Switches screens and remembers the choice so the next launch reopens it.
    fn set_screen(&mut self, screen: AuthScreen, cx: &mut Context<Self>) {
        self.screen = screen;
        let state = SavedNavigation {
            screen,
            token: crate::auth::client::get_current_token(),
            member_name: self.member_name.clone(),
        };
        if let Err(err) = navigation::save(&self.connection, &state) {
            eprintln!("failed to save last screen: {err}");
        }
        cx.notify();
    }
}

impl Render for UmuryangoApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        let language = self.language;
        crate::auth::client::set_current_language(language.locale_code());

        let active_view: AnyElement = match self.screen {
            AuthScreen::Login => {
                self.login_view.update(cx, |view, cx| {
                    view.sync_preferences(theme, language, window, cx);
                });
                self.login_view.clone().into_any_element()
            }
            AuthScreen::Register => {
                self.register_view.update(cx, |view, cx| {
                    view.sync_preferences(theme, language, window, cx);
                });
                self.register_view.clone().into_any_element()
            }
            AuthScreen::Terms => {
                self.terms_view.update(cx, |view, cx| {
                    view.sync_preferences(theme, language, cx);
                });
                self.terms_view.clone().into_any_element()
            }
            AuthScreen::FamilySetup => {
                self.family_setup_view.update(cx, |view, cx| {
                    view.sync_preferences(theme, language, cx);
                });
                self.family_setup_view.clone().into_any_element()
            }
            AuthScreen::CreateFamily => {
                self.create_family_view.update(cx, |view, cx| {
                    view.sync_preferences(theme, language, window, cx);
                });
                self.create_family_view.clone().into_any_element()
            }
            AuthScreen::JoinFamily => {
                self.join_family_view.update(cx, |view, cx| {
                    view.sync_preferences(theme, language, cx);
                });
                self.join_family_view.clone().into_any_element()
            }
            AuthScreen::Dashboard => {
                self.dashboard_view.update(cx, |view, cx| {
                    view.sync_preferences(theme, language, window, cx);
                });
                self.dashboard_view.clone().into_any_element()
            }
        };

        div()
            .size_full()
            .bg(colors(self.theme).bg)
            .child(active_view)
    }
}
