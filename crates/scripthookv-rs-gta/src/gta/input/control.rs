use num_enum::IntoPrimitive;

use crate::natives::pad;

use super::InputContext;

#[derive(IntoPrimitive, Copy, Clone)]
#[repr(i32)]
pub enum Control {
  NextCamera,
  LookLr,
  LookUd,
  LookUpOnly,
  LookDownOnly,
  LookLeftOnly,
  LookRightOnly,
  CinematicSlowmo,
  ScriptedFlyUd,
  ScriptedFlyLr,
  ScriptedFlyZup,
  ScriptedFlyZdown,
  WeaponWheelUd,
  WeaponWheelLr,
  WeaponWheelNext,
  WeaponWheelPrev,
  SelectNextWeapon,
  SelectPrevWeapon,
  SkipCutscene,
  CharacterWheel,
  MultiplayerInfo,
  Sprint,
  Jump,
  Enter,
  Attack,
  Aim,
  LookBehind,
  Phone,
  SpecialAbility,
  SpecialAbilitySecondary,
  MoveLr,
  MoveUd,
  MoveUpOnly,
  MoveDownOnly,
  MoveLeftOnly,
  MoveRightOnly,
  Duck,
  SelectWeapon,
  Pickup,
  SniperZoom,
  SniperZoomInOnly,
  SniperZoomOutOnly,
  SniperZoomInSecondary,
  SniperZoomOutSecondary,
  Cover,
  Reload,
  Talk,
  Detonate,
  HudSpecial,
  Arrest,
  AccurateAim,
  Context,
  ContextSecondary,
  WeaponSpecial,
  WeaponSpecialTwo,
  Dive,
  DropWeapon,
  DropAmmo,
  ThrowGrenade,
  VehMoveLr,
  VehMoveUd,
  VehMoveUpOnly,
  VehMoveDownOnly,
  VehMoveLeftOnly,
  VehMoveRightOnly,
  VehSpecial,
  VehGunLr,
  VehGunUd,
  VehAim,
  VehAttack,
  VehAttack2,
  VehAccelerate,
  VehBrake,
  VehDuck,
  VehHeadlight,
  VehExit,
  VehHandbrake,
  VehHotwireLeft,
  VehHotwireRight,
  VehLookBehind,
  VehCinCam,
  VehNextRadio,
  VehPrevRadio,
  VehNextRadioTrack,
  VehPrevRadioTrack,
  VehRadioWheel,
  VehHorn,
  VehFlyThrottleUp,
  VehFlyThrottleDown,
  VehFlyYawLeft,
  VehFlyYawRight,
  VehPassengerAim,
  VehPassengerAttack,
  VehSpecialAbilityFranklin,
  VehStuntUd,
  VehCinematicUd,
  VehCinematicUpOnly,
  VehCinematicDownOnly,
  VehCinematicLr,
  VehSelectNextWeapon,
  VehSelectPrevWeapon,
  VehRoof,
  VehJump,
  VehGrapplingHook,
  VehShuffle,
  VehDropProjectile,
  VehMouseControlOverride,
  VehFlyRollLr,
  VehFlyRollLeftOnly,
  VehFlyRollRightOnly,
  VehFlyPitchUd,
  VehFlyPitchUpOnly,
  VehFlyPitchDownOnly,
  VehFlyUndercarriage,
  VehFlyAttack,
  VehFlySelectNextWeapon,
  VehFlySelectPrevWeapon,
  VehFlySelectTargetLeft,
  VehFlySelectTargetRight,
  VehFlyVerticalFlightMode,
  VehFlyDuck,
  VehFlyAttackCamera,
  VehFlyMouseControlOverride,
  VehSubTurnLr,
  VehSubTurnLeftOnly,
  VehSubTurnRightOnly,
  VehSubPitchUd,
  VehSubPitchUpOnly,
  VehSubPitchDownOnly,
  VehSubThrottleUp,
  VehSubThrottleDown,
  VehSubAscend,
  VehSubDescend,
  VehSubTurnHardLeft,
  VehSubTurnHardRight,
  VehSubMouseControlOverride,
  VehPushbikePedal,
  VehPushbikeSprint,
  VehPushbikeFrontBrake,
  VehPushbikeRearBrake,
  MeleeAttackLight,
  MeleeAttackHeavy,
  MeleeAttackAlternate,
  MeleeBlock,
  ParachuteDeploy,
  ParachuteDetach,
  ParachuteTurnLr,
  ParachuteTurnLeftOnly,
  ParachuteTurnRightOnly,
  ParachutePitchUd,
  ParachutePitchUpOnly,
  ParachutePitchDownOnly,
  ParachuteBrakeLeft,
  ParachuteBrakeRight,
  ParachuteSmoke,
  ParachutePrecisionLanding,
  Map,
  SelectWeaponUnarmed,
  SelectWeaponMelee,
  SelectWeaponHandgun,
  SelectWeaponShotgun,
  SelectWeaponSmg,
  SelectWeaponAutoRifle,
  SelectWeaponSniper,
  SelectWeaponHeavy,
  SelectWeaponSpecial,
  SelectCharacterMichael,
  SelectCharacterFranklin,
  SelectCharacterTrevor,
  SelectCharacterMultiplayer,
  SaveReplayClip,
  SpecialAbilityPc,
  CellphoneUp,
  CellphoneDown,
  CellphoneLeft,
  CellphoneRight,
  CellphoneSelect,
  CellphoneCancel,
  CellphoneOption,
  CellphoneExtraOption,
  CellphoneScrollForward,
  CellphoneScrollBackward,
  CellphoneCameraFocusLock,
  CellphoneCameraGrid,
  CellphoneCameraSelfie,
  CellphoneCameraDof,
  CellphoneCameraExpression,
  FrontendDown,
  FrontendUp,
  FrontendLeft,
  FrontendRight,
  FrontendRdown,
  FrontendRup,
  FrontendRleft,
  FrontendRright,
  FrontendAxisX,
  FrontendAxisY,
  FrontendRightAxisX,
  FrontendRightAxisY,
  FrontendPause,
  FrontendPauseAlternate,
  FrontendAccept,
  FrontendCancel,
  FrontendX,
  FrontendY,
  FrontendLb,
  FrontendRb,
  FrontendLt,
  FrontendRt,
  FrontendLs,
  FrontendRs,
  FrontendLeaderboard,
  FrontendSocialClub,
  FrontendSocialClubSecondary,
  FrontendDelete,
  FrontendEndscreenAccept,
  FrontendEndscreenExpand,
  FrontendSelect,
  ScriptLeftAxisX,
  ScriptLeftAxisY,
  ScriptRightAxisX,
  ScriptRightAxisY,
  ScriptRup,
  ScriptRdown,
  ScriptRleft,
  ScriptRright,
  ScriptLb,
  ScriptRb,
  ScriptLt,
  ScriptRt,
  ScriptLs,
  ScriptRs,
  ScriptPadUp,
  ScriptPadDown,
  ScriptPadLeft,
  ScriptPadRight,
  ScriptSelect,
  CursorAccept,
  CursorCancel,
  CursorX,
  CursorY,
  CursorScrollUp,
  CursorScrollDown,
  EnterCheatCode,
  InteractionMenu,
  MpTextChatAll,
  MpTextChatTeam,
  MpTextChatFriends,
  MpTextChatCrew,
  PushToTalk,
  CreatorLs,
  CreatorRs,
  CreatorLt,
  CreatorRt,
  CreatorMenuToggle,
  CreatorAccept,
  CreatorDelete,
  Attack2,
  RappelJump,
  RappelLongJump,
  RappelSmashWindow,
  PrevWeapon,
  NextWeapon,
  MeleeAttack1,
  MeleeAttack2,
  Whistle,
  MoveLeft,
  MoveRight,
  MoveUp,
  MoveDown,
  LookLeft,
  LookRight,
  LookUp,
  LookDown,
  SniperZoomIn,
  SniperZoomOut,
  SniperZoomInAlternate,
  SniperZoomOutAlternate,
  VehMoveLeft,
  VehMoveRight,
  VehMoveUp,
  VehMoveDown,
  VehGunLeft,
  VehGunRight,
  VehGunUp,
  VehGunDown,
  VehLookLeft,
  VehLookRight,
  ReplayStartStopRecording,
  ReplayStartStopRecordingSecon,
  ScaledLookLr,
  ScaledLookUd,
  ScaledLookUpOnly,
  ScaledLookDownOnly,
  ScaledLookLeftOnly,
  ScaledLookRightOnly,
  ReplayMarkerDelete,
  ReplayClipDelete,
  ReplayPause,
  ReplayRewind,
  ReplayFfwd,
  ReplayNewmarker,
  ReplayRecord,
  ReplayScreenshot,
  ReplayHidehud,
  ReplayStartpoint,
  ReplayEndpoint,
  ReplayAdvance,
  ReplayBack,
  ReplayTools,
  ReplayRestart,
  ReplayShowhotkey,
  ReplayCyclemarkerleft,
  ReplayCyclemarkerright,
  ReplayFovincrease,
  ReplayFovdecrease,
  ReplayCameraup,
  ReplayCameradown,
  ReplaySave,
  ReplayToggletime,
  ReplayToggletips,
  ReplayPreview,
  ReplayToggleTimeline,
  ReplayTimelinePickupClip,
  ReplayTimelineDuplicateClip,
  ReplayTimelinePlaceClip,
  ReplayCtrl,
  ReplayTimelineSave,
  ReplayPreviewAudio,
  VehDriveLook,
  VehDriveLook2,
  VehFlyAttack2,
  RadioWheelUd,
  RadioWheelLr,
  VehSlowmoUd,
  VehSlowmoUpOnly,
  VehSlowmoDownOnly,
  VehHydraulicsControlToggle,
  VehHydraulicsControlLeft,
  VehHydraulicsControlRight,
  VehHydraulicsControlUp,
  VehHydraulicsControlDown,
  VehHydraulicsControlLr,
  VehHydraulicsControlUd,
  SwitchVisor,
  VehMeleeHold,
  VehMeleeLeft,
  VehMeleeRight,
  MapPoi,
  ReplaySnapmaticPhoto,
  VehCarJump,
  VehRocketBoost,
  VehFlyBoost,
  VehParachute,
  VehBikeWings,
  VehFlyBombBay,
  VehFlyCounter,
  VehTransform,
  QuadLocoReverse
}

impl Control {
  #[inline]
  pub fn disable_all(context: InputContext) {
    unsafe { pad::disable_all_control_actions(context.into()) }
  }

  #[inline]
  #[must_use]
  pub fn is_using_keyboard(context: InputContext) -> bool {
    unsafe { pad::is_using_keyboard_and_mouse(context.into()) }
  }

  #[inline]
  #[must_use]
  pub fn is_using_controller(context: InputContext) -> bool {
    !Self::is_using_keyboard(context)
  }

  #[inline]
  #[must_use]
  pub fn is_enabled(self, context: InputContext) -> bool {
    unsafe { pad::is_control_enabled(context.into(), self.into()) }
  }

  #[inline]
  pub fn disable(self, context: InputContext) {
    unsafe { pad::disable_control_action(context.into(), self.into(), true) }
  }

  #[inline]
  pub fn enable(self, context: InputContext) {
    unsafe { pad::enable_control_action(context.into(), self.into(), true) }
  }

  #[inline]
  #[must_use]
  pub fn is_pressed(self, context: InputContext, disabled: bool) -> bool {
    if disabled {
      unsafe { pad::is_disabled_control_pressed(context.into(), self.into()) }
    } else {
      unsafe { pad::is_control_pressed(context.into(), self.into()) }
    }
  }

  #[inline]
  #[must_use]
  pub fn is_just_pressed(self, context: InputContext, disabled: bool) -> bool {
    if disabled {
      unsafe { pad::is_disabled_control_just_pressed(context.into(), self.into()) }
    } else {
      unsafe { pad::is_control_just_pressed(context.into(), self.into()) }
    }
  }

  #[inline]
  #[must_use]
  pub fn is_released(self, context: InputContext, disabled: bool) -> bool {
    if disabled {
      unsafe { pad::is_disabled_control_released(context.into(), self.into()) }
    } else {
      unsafe { pad::is_control_released(context.into(), self.into()) }
    }
  }

  #[inline]
  #[must_use]
  pub fn is_just_released(self, context: InputContext, disabled: bool) -> bool {
    if disabled {
      unsafe { pad::is_disabled_control_just_released(context.into(), self.into()) }
    } else {
      unsafe { pad::is_control_just_released(context.into(), self.into()) }
    }
  }

  #[inline]
  #[must_use]
  pub fn get_value(self, context: InputContext) -> i32 {
    unsafe { pad::get_control_value(context.into(), self.into()) }
  }

  #[inline]
  #[must_use]
  pub fn get_normal(self, context: InputContext, disabled: bool) -> f32 {
    if disabled {
      unsafe { pad::get_disabled_control_normal(context.into(), self.into()) }
    } else {
      unsafe { pad::get_control_normal(context.into(), self.into()) }
    }
  }

  #[inline]
  #[must_use]
  pub fn get_unbound_normal(self, context: InputContext, disabled: bool) -> f32 {
    if disabled {
      unsafe { pad::get_disabled_control_unbound_normal(context.into(), self.into()) }
    } else {
      unsafe { pad::get_control_unbound_normal(context.into(), self.into()) }
    }
  }
}
