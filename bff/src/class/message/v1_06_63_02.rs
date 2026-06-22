use crate::helpers::Message;
use crate::source::message::{SourceMessage, SourceMessageKind};

pub fn source_message_v1_06_63_02_pc(message: &Message) -> SourceMessage {
    let Some(command) = message_kind_v1_06_63_02_pc(message.message_id) else {
        return raw_source_message(message);
    };

    SourceMessage::Command {
        command,
        args: message_args_v1_06_63_02_pc(message, command),
    }
}

pub fn raw_source_message(message: &Message) -> SourceMessage {
    SourceMessage::Raw {
        message_id: message.message_id,
        u32_param: message.u32_param,
        flag_param: message.flag_param,
        float_param: message.float_param,
        name_param: message.name_param,
    }
}

fn message_args_v1_06_63_02_pc(message: &Message, command: SourceMessageKind) -> Vec<String> {
    match command {
        SourceMessageKind::MsgAnimPlayDialog => vec![
            message.u32_param.to_string(),
            message.float_param.to_string(),
        ],
        SourceMessageKind::MsgAnimLoadDialog => vec![message.u32_param.to_string()],
        SourceMessageKind::MsgAnimPlaySound => vec![
            message.u32_param.to_string(),
            message.flag_param.to_string(),
            message.float_param.to_string(),
        ],
        SourceMessageKind::MsgAnimStopSound
        | SourceMessageKind::MsgAnimHideNode
        | SourceMessageKind::MsgAnimUnhideNode
        | SourceMessageKind::MsgAnimSetAnim
        | SourceMessageKind::MsgAnimStartMaterialAnim
        | SourceMessageKind::MsgAnimStopMaterialAnim
        | SourceMessageKind::MsgAnimPauseMaterialAnim => vec![message.u32_param.to_string()],
        SourceMessageKind::MsgRtcRetrieveNode => {
            vec![message.u32_param.to_string(), message.name_param.to_string()]
        }
        SourceMessageKind::MsgRtcOrientNodes
        | SourceMessageKind::MsgRtcSetCameraGame
        | SourceMessageKind::MsgRtcSetCameraChase
        | SourceMessageKind::MsgRtcSetCameraTravelling
        | SourceMessageKind::MsgRtcSetCameraForce
        | SourceMessageKind::MsgRtcSetCameraFocus
        | SourceMessageKind::MsgRtcSetCameraDialog => {
            vec![message.u32_param.to_string(), message.name_param.to_string()]
        }
        SourceMessageKind::MsgRtcHideNode | SourceMessageKind::MsgRtcUnhideNode => {
            vec![message.name_param.to_string()]
        }
        SourceMessageKind::MsgRtcControlStrip
        | SourceMessageKind::MsgRtcSetCameraForceTravelling
        | SourceMessageKind::MsgRtcCameraCut
        | SourceMessageKind::MsgRtcPlayAnim
        | SourceMessageKind::MsgRtcStartCountdown
        | SourceMessageKind::MsgRtcDoFade
        | SourceMessageKind::MsgRtcWait
        | SourceMessageKind::MsgRtcSetText
        | SourceMessageKind::MsgRtcWaitAnim
        | SourceMessageKind::MsgRtcStartAnimframe => vec![message.u32_param.to_string()],
        SourceMessageKind::MsgRtcCameraSmooth | SourceMessageKind::MsgRtcStartRtcFx => {
            vec![message.u32_param.to_string(), message.float_param.to_string()]
        }
        SourceMessageKind::MsgRtcGoto => {
            vec![message.name_param.to_string(), message.u32_param.to_string()]
        }
        SourceMessageKind::MsgRtcSetDof
        | SourceMessageKind::MsgRtcChangeSmoothAnim
        | SourceMessageKind::MsgRtcPlaySound => vec![message.float_param.to_string()],
        SourceMessageKind::MsgRtcPlayDialog => vec![
            message.name_param.to_string(),
            message.u32_param.to_string(),
            message.float_param.to_string(),
        ],
        SourceMessageKind::MsgRtcPosPerso | SourceMessageKind::MsgRtcPosVehicle => {
            vec![message.name_param.to_string(), message.u32_param.to_string()]
        }
        _ => Vec::new(),
    }
}

pub fn message_kind_v1_06_63_02_pc(message_id: u32) -> Option<SourceMessageKind> {
    Some(match message_id {
        0 => SourceMessageKind::MsgNoMessage,
        1 => SourceMessageKind::MsgAnimNoteTrackFirst,
        2 => SourceMessageKind::MsgSkelCollide,
        3 => SourceMessageKind::MsgAnimLoop,
        4 => SourceMessageKind::MsgAnimNosmooth,
        5 => SourceMessageKind::MsgAnimSmoothratio,
        6 => SourceMessageKind::MsgAnimNodeCollide,
        7 => SourceMessageKind::MsgAnimNodeUncollide,
        8 => SourceMessageKind::MsgAnimNodeGetorient,
        9 => SourceMessageKind::MsgAnimOrient,
        10 => SourceMessageKind::MsgAnimNoorient,
        11 => SourceMessageKind::MsgAnimAction,
        12 => SourceMessageKind::MsgAnimLoopStart,
        13 => SourceMessageKind::MsgAnimLoopEnd,
        14 => SourceMessageKind::MsgAnimNodynamic,
        15 => SourceMessageKind::MsgAnimDynamic,
        16 => SourceMessageKind::MsgAnimMaterialColor,
        17 => SourceMessageKind::MsgAnimActivateAnimatedObject,
        18 => SourceMessageKind::MsgAnimDeactivateAnimatedObject,
        19 => SourceMessageKind::MsgAnimHide,
        20 => SourceMessageKind::MsgAnimUnhide,
        21 => SourceMessageKind::MsgAnimReset,
        22 => SourceMessageKind::MsgAnimReplay,
        23 => SourceMessageKind::MsgAnimPause,
        24 => SourceMessageKind::MsgAnimResume,
        25 => SourceMessageKind::MsgAnimPlayDialog,
        26 => SourceMessageKind::MsgAnimStopDialog,
        27 => SourceMessageKind::MsgAnimLoadDialog,
        28 => SourceMessageKind::MsgAnimUnknownSkelCollide,
        29 => SourceMessageKind::MsgAnimHitSkelCollide,
        30 => SourceMessageKind::MsgAnimParticlesEfx,
        31 => SourceMessageKind::MsgAnimParticlesEfxPause,
        32 => SourceMessageKind::MsgAnimParticlesEfxReset,
        33 => SourceMessageKind::MsgAnimPlaySound,
        34 => SourceMessageKind::MsgAnimStopSound,
        35 => SourceMessageKind::MsgAnimHideNode,
        36 => SourceMessageKind::MsgAnimUnhideNode,
        37 => SourceMessageKind::MsgAnimSetAnim,
        38 => SourceMessageKind::MsgAnimStartMaterialAnim,
        39 => SourceMessageKind::MsgAnimStopMaterialAnim,
        40 => SourceMessageKind::MsgAnimPauseMaterialAnim,
        41 => SourceMessageKind::MsgRtcPicture,
        42 => SourceMessageKind::MsgRscNoteTrackEnd,
        43 => SourceMessageKind::MsgRtcStarted,
        44 => SourceMessageKind::MsgRtcEnded,
        45 => SourceMessageKind::MsgRtcChangecamera,
        46 => SourceMessageKind::MsgRtcNoPicture,
        47 => SourceMessageKind::MsgRtcMark,
        48 => SourceMessageKind::MsgRtcLetterBoxStart,
        49 => SourceMessageKind::MsgRtcLetterBoxEnd,
        50 => SourceMessageKind::MsgRtcLetterBoxColor,
        51 => SourceMessageKind::MsgAnimNoteTrackEnd,
        52 => SourceMessageKind::MsgGotoCurlength,
        53 => SourceMessageKind::MsgGoto2ndVertex,
        54 => SourceMessageKind::MsgGotoEnd,
        55 => SourceMessageKind::MsgVolumeIn,
        56 => SourceMessageKind::MsgVolumeOut,
        57 => SourceMessageKind::MsgVolumeStay,
        58 => SourceMessageKind::MsgVolumeAnimReset,
        59 => SourceMessageKind::MsgAnimStopallsound,
        60 => SourceMessageKind::MsgAnimNodeStopalldialog,
        61 => SourceMessageKind::MsgCarryBegin,
        62 => SourceMessageKind::MsgCarryEnd,
        63 => SourceMessageKind::MsgHeldBegin,
        64 => SourceMessageKind::MsgHeldEnd,
        65 => SourceMessageKind::MsgThrownBegin,
        66 => SourceMessageKind::MsgThrownEnd,
        67 => SourceMessageKind::MsgTargetHit,
        68 => SourceMessageKind::MsgTargetMiss,
        69 => SourceMessageKind::MsgLodCollide,
        70 => SourceMessageKind::MsgDecorCollide,
        71 => SourceMessageKind::MsgThrownLodCollide,
        72 => SourceMessageKind::MsgThrownDecorCollide,
        73 => SourceMessageKind::MsgRestoreCollide,
        74 => SourceMessageKind::MsgSequencedCollide,
        75 => SourceMessageKind::MsgSkinActBegin,
        76 => SourceMessageKind::MsgSkinActEnd,
        77 => SourceMessageKind::MsgBumper,
        78 => SourceMessageKind::MsgCameraCapturePatchStart,
        79 => SourceMessageKind::MsgCameraCaptureFrameStart,
        80 => SourceMessageKind::MsgCameraCaptureTextureStart,
        81 => SourceMessageKind::MsgCameraCaptureEnd,
        82 => SourceMessageKind::MsgCameraCaptureParam1,
        83 => SourceMessageKind::MsgCameraCaptureParam2,
        84 => SourceMessageKind::MsgCameraCaptureParam3,
        85 => SourceMessageKind::MsgCameraResetFocus,
        86 => SourceMessageKind::MsgStreamEnd,
        87 => SourceMessageKind::MsgStreamErrorInit,
        88 => SourceMessageKind::MsgStreamError,
        89 => SourceMessageKind::MsgStreamErrorTrayopen,
        90 => SourceMessageKind::MsgStreamResumed,
        91 => SourceMessageKind::MsgStreamResumePossible,
        92 => SourceMessageKind::MsgStreamAbort,
        93 => SourceMessageKind::MsgStreamAborted,
        94 => SourceMessageKind::MsgGameActivated,
        95 => SourceMessageKind::MsgGameDeactivated,
        96 => SourceMessageKind::MsgGameReseted,
        97 => SourceMessageKind::MsgAnimFramelink,
        98 => SourceMessageKind::MsgAnimFrameunlink,
        99 => SourceMessageKind::MsgAvirer,
        100 => SourceMessageKind::MsgOccluderOn,
        101 => SourceMessageKind::MsgOccluderOff,
        102 => SourceMessageKind::MsgStencilOn,
        103 => SourceMessageKind::MsgStencilOff,
        104 => SourceMessageKind::MsgEngineEnd,
        105 => SourceMessageKind::MsgCollisionPlayerIn,
        106 => SourceMessageKind::MsgCollisionPlayerOut,
        107 => SourceMessageKind::MsgEndCheckPlayer,
        108 => SourceMessageKind::MsgStartCheckRespawn,
        109 => SourceMessageKind::MsgCheckPlayer,
        110 => SourceMessageKind::MsgRespawn,
        111 => SourceMessageKind::MsgResetBaseAgent,
        112 => SourceMessageKind::MsgRtcScriptInterpCommand,
        113 => SourceMessageKind::MsgRtcScriptCheckSkipRtc,
        114 => SourceMessageKind::MsgRtcScriptCheckSkipText,
        115 => SourceMessageKind::MsgMenuActivateGame,
        117 => SourceMessageKind::MsgRtcScriptCheckEndGoto,
        119 => SourceMessageKind::MsgNoteTrackSample,
        120 => SourceMessageKind::MsgToLevel,
        121 => SourceMessageKind::MsgChangeLevel,
        122 => SourceMessageKind::MsgToMenu,
        125 => SourceMessageKind::MsgStartMenu,
        126 => SourceMessageKind::MsgRtcRetrieveNode,
        127 => SourceMessageKind::MsgRtcOrientNodes,
        128 => SourceMessageKind::MsgRtcHideNode,
        129 => SourceMessageKind::MsgRtcUnhideNode,
        130 => SourceMessageKind::MsgRtcControlStrip,
        131 => SourceMessageKind::MsgRtcSetCameraGame,
        132 => SourceMessageKind::MsgRtcSetCameraChase,
        133 => SourceMessageKind::MsgRtcSetCameraTravelling,
        134 => SourceMessageKind::MsgRtcSetCameraForce,
        135 => SourceMessageKind::MsgRtcSetCameraForceTravelling,
        136 => SourceMessageKind::MsgRtcSetCameraFocus,
        137 => SourceMessageKind::MsgRtcSetCameraDialog,
        138 => SourceMessageKind::MsgRtcCameraCut,
        139 => SourceMessageKind::MsgRtcCameraSmooth,
        140 => SourceMessageKind::MsgRtcPlayAnim,
        141 => SourceMessageKind::MsgRtcGoto,
        142 => SourceMessageKind::MsgRtcSkip,
        143 => SourceMessageKind::MsgRtcClearText,
        144 => SourceMessageKind::MsgRtcStartCountdown,
        145 => SourceMessageKind::MsgRtcSetDof,
        146 => SourceMessageKind::MsgRtcEndOfRtc,
        147 => SourceMessageKind::MsgRtcDisableSkip,
        148 => SourceMessageKind::MsgRtcDoFade,
        149 => SourceMessageKind::MsgRtcWait,
        150 => SourceMessageKind::MsgRtcSetText,
        153 => SourceMessageKind::MsgRtcWaitAnim,
        154 => SourceMessageKind::MsgRtcPlayDialog,
        155 => SourceMessageKind::MsgRtcPosPerso,
        156 => SourceMessageKind::MsgRtcPosVehicle,
        157 => SourceMessageKind::MsgRtcWaitGoto,
        158 => SourceMessageKind::MsgRtcStartAnimframe,
        159 => SourceMessageKind::MsgRtcStartRtcFx,
        160 => SourceMessageKind::MsgRtcChangeSmoothAnim,
        161 => SourceMessageKind::MsgRtcPlaySound,
        163 => SourceMessageKind::MsgRtcUnhideAllCreatures,
        164 => SourceMessageKind::MsgRtcUnkSl,
        165 => SourceMessageKind::MsgEnterTeleport,
        166 => SourceMessageKind::MsgEnterTeleportAlternative,
        167 => SourceMessageKind::MsgExecuteTeleport,
        169 => SourceMessageKind::MsgWarpColorCheck,
        170 => SourceMessageKind::MsgWarpColorCheck2,
        171 => SourceMessageKind::MsgWarpColorDontCheck,
        174 => SourceMessageKind::MsgBlockFader,
        182 => SourceMessageKind::MsgFxTraceBStart,
        183 => SourceMessageKind::MsgFxTraceBEnd,
        184 => SourceMessageKind::MsgFxTraceEStart,
        185 => SourceMessageKind::MsgFxTraceEEnd,
        191 => SourceMessageKind::MsgBreak,
        192 => SourceMessageKind::MsgGenerateObject,
        193 => SourceMessageKind::MsgExecuteTeleportEnd,
        196 => SourceMessageKind::MsgFootOnGroundBone,
        197 => SourceMessageKind::MsgFootprintOnGround,
        199 => SourceMessageKind::MsgAnimChange,
        201 => SourceMessageKind::MsgDeactivateSceneOmnis,
        202 => SourceMessageKind::MsgActivateSceneOmnis,
        204 => SourceMessageKind::MsgActionInEventVolume,
        206 => SourceMessageKind::MsgExplosionEventVolume,
        207 => SourceMessageKind::MsgImpactable,
        208 => SourceMessageKind::MsgLockPos,
        209 => SourceMessageKind::MsgUnlockPos,
        210 => SourceMessageKind::MsgContextualSound1,
        211 => SourceMessageKind::MsgContextualSound2,
        212 => SourceMessageKind::MsgSoundClothe,
        213 => SourceMessageKind::MsgHit,
        214 => SourceMessageKind::MsgUnhit,
        215 => SourceMessageKind::MsgEvent,
        228 => SourceMessageKind::MsgStartMission,
        230 => SourceMessageKind::MsgCheckPlayerDistMission,
        232 => SourceMessageKind::MsgCheckRespawnSpecific,
        233 => SourceMessageKind::MsgStartBlind,
        234 => SourceMessageKind::MsgFinishBlind,
        235 => SourceMessageKind::MsgOnGround,
        236 => SourceMessageKind::MsgItCarryingRespawnTeleport,
        _ => return None,
    })
}
