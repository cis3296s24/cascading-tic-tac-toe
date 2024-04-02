use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioPlugin, AudioSource, AudioChannel};
pub struct GameAudioPlugin;

pub struct AudioState{
    bgm_handle: Handle<AudioSource>,
    combat_handle: Handle<AudioSource>,
    hit_handle : Handle<AudioSource>,
    reward_handle : Handle<AudioSource>,

    bgm_channel: AudioChannel,
    combat_channel: AudioChannel,
    sfx_channel: AudioChannel,
    volume: f32,
}

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
        .add_startup_system_to_stage(StartupStage::PreStartuo,load_audio);
        .add_startup_system(start_bgm_music);
    }
}

fn start_bgm_music(audio: Res<Audio>, audio_state: Res<AudioState>){
    audio.play_looped_in_channel(
        audio_state.bgm_handle.clone(),
        &audio_state.bgm_channel
    );
}

fn load_audio(mut commands: Commands, audio: Res<Audio>, asset: Res<AssetServer>){
    let bgm_handle = assets.load("sounds/Crush8-Bit.ogg");
    let combat_handle = assets.load("");
    let hit_handle = assets.load("");
    let reward_handle = assets.load("");

    let bgm_channel: AudioChannel = new("bgm".to_string()),
    let combat_channel: AudioChannel = new("combat".tostring()),
    let sfx_channel: AudioChannel = new("sfx".tostring()),
    let volume = 0.5,


    audio.set_volume_in_channel(volume,&bgm_channel);
    audio.set_volume_in_channel(volume,&combat_channel);
    audio.set_volume_in_channel(volume,&sfx_channel);


    commands.insert_resource(AudioState{
        bgm_handle: bgm_handle,
        combat_handle: combat_handle,
        hit_handle: hit_handle,
        reward_handle: reward_handle,
        bgm_channel,
        combat_channel,
        sfx_channel,
        volume,
    });
   
}
