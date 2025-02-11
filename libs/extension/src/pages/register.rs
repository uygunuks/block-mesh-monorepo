use crate::components::credentials::CredentialsForm;
use crate::utils::auth::register;
use crate::utils::log::{log_error, log_warn};
use crate::utils::state::{AppState, AppStatus};
use block_mesh_common::interface::RegisterForm;
use leptos::{logging::log, *};

#[component]
pub fn Register(#[prop(into)] on_success: Callback<()>) -> impl IntoView {
    let (register_error, set_register_error) = create_signal(None::<String>);
    let (wait_for_response, set_wait_for_response) = create_signal(false);
    let state = use_context::<AppState>().unwrap();

    let register_action = create_action(move |params: &Vec<String>| {
        let email = params[0].to_string();
        let password = params[1].to_string();
        let password_confirm = params[2].to_string();
        let credentials = RegisterForm {
            email,
            password,
            password_confirm,
        };
        log!("Try to register new account for {}", credentials.email);
        async move {
            if credentials.password != credentials.password_confirm {
                log_warn!("Passwords do not match");
                set_register_error.update(|e| *e = Some("Passwords do not match".to_string()));
                return;
            }
            set_wait_for_response.update(|w| *w = true);
            let result = register(&state.blockmesh_url.get_untracked(), &credentials).await;
            set_wait_for_response.update(|w| *w = false);
            match result {
                Ok(_) => {
                    set_register_error.update(|e| *e = None);
                    state
                        .status
                        .update(|v| *v = AppStatus::WaitingEmailVerification);
                    on_success.call(());
                }
                Err(err) => {
                    log_error!(
                        "Unable to register new account for {}: {err}",
                        credentials.email
                    );
                    set_register_error.update(|e| *e = Some(err.to_string()));
                }
            }
        }
    });

    let disabled = Signal::derive(move || wait_for_response.get());

    view! {
        <CredentialsForm
            title="Register"
            action_label="Register"
            action=register_action
            error=register_error.into()
            disabled=disabled
            register=true
        />
    }
}
