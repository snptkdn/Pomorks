use anyhow::Result;
use pomorks_data_manage::todo::State;
use std::{path::PathBuf, str::FromStr};
use winrt_notification::{Duration, Sound, Toast};

pub fn send_notification(state: &State) -> Result<()> {
    Toast::new(Toast::POWERSHELL_APP_ID)
        .title("Pomodoro-rs")
        .text1(&(State::get_state_name(state) + &" is finish.".to_string()))
        .sound(Some(Sound::SMS))
        // TODO!:imageが表示されない問題
        .image(&PathBuf::from_str("./image/tomato.png")?, "Tomato")
        .duration(Duration::Short)
        .show()?;

    Ok(())
}
