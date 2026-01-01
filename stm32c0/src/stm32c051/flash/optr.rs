///Register `OPTR` reader
pub type R = crate::R<OPTRrs>;
///Register `OPTR` writer
pub type W = crate::W<OPTRrs>;
/**Read protection level Other: Level 1, memories read protection active

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RDP {
    ///170: Level 0, read protection not active
    B0xAa = 170,
    ///204: Level 2, chip read protection active
    B0xCc = 204,
}
impl From<RDP> for u8 {
    #[inline(always)]
    fn from(variant: RDP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RDP {
    type Ux = u8;
}
impl crate::IsEnum for RDP {}
///Field `RDP` reader - Read protection level Other: Level 1, memories read protection active
pub type RDP_R = crate::FieldReader<RDP>;
impl RDP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<RDP> {
        match self.bits {
            170 => Some(RDP::B0xAa),
            204 => Some(RDP::B0xCc),
            _ => None,
        }
    }
    ///Level 0, read protection not active
    #[inline(always)]
    pub fn is_b_0x_aa(&self) -> bool {
        *self == RDP::B0xAa
    }
    ///Level 2, chip read protection active
    #[inline(always)]
    pub fn is_b_0x_cc(&self) -> bool {
        *self == RDP::B0xCc
    }
}
///Field `RDP` writer - Read protection level Other: Level 1, memories read protection active
pub type RDP_W<'a, REG> = crate::FieldWriter<'a, REG, 8, RDP>;
impl<'a, REG> RDP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Level 0, read protection not active
    #[inline(always)]
    pub fn b_0x_aa(self) -> &'a mut crate::W<REG> {
        self.variant(RDP::B0xAa)
    }
    ///Level 2, chip read protection active
    #[inline(always)]
    pub fn b_0x_cc(self) -> &'a mut crate::W<REG> {
        self.variant(RDP::B0xCc)
    }
}
/**Brown out reset enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOR_EN {
    ///0: Configurable brown out reset disabled, power-on reset defined by POR/PDR levels
    B0x0 = 0,
    ///1: Configurable brown out reset enabled, values of BORR_LEV and BORF_LEV taken into account
    B0x1 = 1,
}
impl From<BOR_EN> for bool {
    #[inline(always)]
    fn from(variant: BOR_EN) -> Self {
        variant as u8 != 0
    }
}
///Field `BOR_EN` reader - Brown out reset enable
pub type BOR_EN_R = crate::BitReader<BOR_EN>;
impl BOR_EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BOR_EN {
        match self.bits {
            false => BOR_EN::B0x0,
            true => BOR_EN::B0x1,
        }
    }
    ///Configurable brown out reset disabled, power-on reset defined by POR/PDR levels
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BOR_EN::B0x0
    }
    ///Configurable brown out reset enabled, values of BORR_LEV and BORF_LEV taken into account
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BOR_EN::B0x1
    }
}
///Field `BOR_EN` writer - Brown out reset enable
pub type BOR_EN_W<'a, REG> = crate::BitWriter<'a, REG, BOR_EN>;
impl<'a, REG> BOR_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Configurable brown out reset disabled, power-on reset defined by POR/PDR levels
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BOR_EN::B0x0)
    }
    ///Configurable brown out reset enabled, values of BORR_LEV and BORF_LEV taken into account
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BOR_EN::B0x1)
    }
}
/**BOR threshold at rising V<sub>DD</sub> supply Rising V<sub>DD</sub> crossings this threshold releases the reset signal.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BORR_LEV {
    ///0: BOR rising level 1 with threshold around 2.1 V
    B0x0 = 0,
    ///1: BOR rising level 2 with threshold around 2.3 V
    B0x1 = 1,
    ///2: BOR rising level 3 with threshold around 2.6 V
    B0x2 = 2,
    ///3: BOR rising level 4 with threshold around 2.9 V
    B0x3 = 3,
}
impl From<BORR_LEV> for u8 {
    #[inline(always)]
    fn from(variant: BORR_LEV) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BORR_LEV {
    type Ux = u8;
}
impl crate::IsEnum for BORR_LEV {}
///Field `BORR_LEV` reader - BOR threshold at rising V<sub>DD</sub> supply Rising V<sub>DD</sub> crossings this threshold releases the reset signal.
pub type BORR_LEV_R = crate::FieldReader<BORR_LEV>;
impl BORR_LEV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BORR_LEV {
        match self.bits {
            0 => BORR_LEV::B0x0,
            1 => BORR_LEV::B0x1,
            2 => BORR_LEV::B0x2,
            3 => BORR_LEV::B0x3,
            _ => unreachable!(),
        }
    }
    ///BOR rising level 1 with threshold around 2.1 V
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BORR_LEV::B0x0
    }
    ///BOR rising level 2 with threshold around 2.3 V
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BORR_LEV::B0x1
    }
    ///BOR rising level 3 with threshold around 2.6 V
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == BORR_LEV::B0x2
    }
    ///BOR rising level 4 with threshold around 2.9 V
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == BORR_LEV::B0x3
    }
}
///Field `BORR_LEV` writer - BOR threshold at rising V<sub>DD</sub> supply Rising V<sub>DD</sub> crossings this threshold releases the reset signal.
pub type BORR_LEV_W<'a, REG> = crate::FieldWriter<'a, REG, 2, BORR_LEV, crate::Safe>;
impl<'a, REG> BORR_LEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///BOR rising level 1 with threshold around 2.1 V
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BORR_LEV::B0x0)
    }
    ///BOR rising level 2 with threshold around 2.3 V
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BORR_LEV::B0x1)
    }
    ///BOR rising level 3 with threshold around 2.6 V
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(BORR_LEV::B0x2)
    }
    ///BOR rising level 4 with threshold around 2.9 V
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(BORR_LEV::B0x3)
    }
}
/**BOR threshold at falling V<sub>DD</sub> supply Falling V<sub>DD</sub> crossings this threshold activates the reset signal.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BORF_LEV {
    ///0: BOR falling level 1 with threshold around 2.0 V
    B0x0 = 0,
    ///1: BOR falling level 2 with threshold around 2.2 V
    B0x1 = 1,
    ///2: BOR falling level 3 with threshold around 2.5 V
    B0x2 = 2,
    ///3: BOR falling level 4 with threshold around 2.8 V
    B0x3 = 3,
}
impl From<BORF_LEV> for u8 {
    #[inline(always)]
    fn from(variant: BORF_LEV) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BORF_LEV {
    type Ux = u8;
}
impl crate::IsEnum for BORF_LEV {}
///Field `BORF_LEV` reader - BOR threshold at falling V<sub>DD</sub> supply Falling V<sub>DD</sub> crossings this threshold activates the reset signal.
pub type BORF_LEV_R = crate::FieldReader<BORF_LEV>;
impl BORF_LEV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BORF_LEV {
        match self.bits {
            0 => BORF_LEV::B0x0,
            1 => BORF_LEV::B0x1,
            2 => BORF_LEV::B0x2,
            3 => BORF_LEV::B0x3,
            _ => unreachable!(),
        }
    }
    ///BOR falling level 1 with threshold around 2.0 V
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BORF_LEV::B0x0
    }
    ///BOR falling level 2 with threshold around 2.2 V
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BORF_LEV::B0x1
    }
    ///BOR falling level 3 with threshold around 2.5 V
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == BORF_LEV::B0x2
    }
    ///BOR falling level 4 with threshold around 2.8 V
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == BORF_LEV::B0x3
    }
}
///Field `BORF_LEV` writer - BOR threshold at falling V<sub>DD</sub> supply Falling V<sub>DD</sub> crossings this threshold activates the reset signal.
pub type BORF_LEV_W<'a, REG> = crate::FieldWriter<'a, REG, 2, BORF_LEV, crate::Safe>;
impl<'a, REG> BORF_LEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///BOR falling level 1 with threshold around 2.0 V
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BORF_LEV::B0x0)
    }
    ///BOR falling level 2 with threshold around 2.2 V
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BORF_LEV::B0x1)
    }
    ///BOR falling level 3 with threshold around 2.5 V
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(BORF_LEV::B0x2)
    }
    ///BOR falling level 4 with threshold around 2.8 V
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(BORF_LEV::B0x3)
    }
}
///Field `NRST_STOP` reader - None
pub type NRST_STOP_R = crate::BitReader;
///Field `NRST_STOP` writer - None
pub type NRST_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NRST_STDBY` reader - None
pub type NRST_STDBY_R = crate::BitReader;
///Field `NRST_STDBY` writer - None
pub type NRST_STDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NRST_SHDW` reader - None
pub type NRST_SHDW_R = crate::BitReader;
///Field `NRST_SHDW` writer - None
pub type NRST_SHDW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG_SW` reader - None
pub type IWDG_SW_R = crate::BitReader;
///Field `IWDG_SW` writer - None
pub type IWDG_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Independent watchdog counter freeze in Stop mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDG_STOP {
    ///0: Independent watchdog counter is frozen in Stop mode
    B0x0 = 0,
    ///1: Independent watchdog counter is running in Stop mode
    B0x1 = 1,
}
impl From<IWDG_STOP> for bool {
    #[inline(always)]
    fn from(variant: IWDG_STOP) -> Self {
        variant as u8 != 0
    }
}
///Field `IWDG_STOP` reader - Independent watchdog counter freeze in Stop mode
pub type IWDG_STOP_R = crate::BitReader<IWDG_STOP>;
impl IWDG_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IWDG_STOP {
        match self.bits {
            false => IWDG_STOP::B0x0,
            true => IWDG_STOP::B0x1,
        }
    }
    ///Independent watchdog counter is frozen in Stop mode
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IWDG_STOP::B0x0
    }
    ///Independent watchdog counter is running in Stop mode
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IWDG_STOP::B0x1
    }
}
///Field `IWDG_STOP` writer - Independent watchdog counter freeze in Stop mode
pub type IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG, IWDG_STOP>;
impl<'a, REG> IWDG_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Independent watchdog counter is frozen in Stop mode
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IWDG_STOP::B0x0)
    }
    ///Independent watchdog counter is running in Stop mode
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IWDG_STOP::B0x1)
    }
}
///Field `IWGD_STDBY` reader - None
pub type IWGD_STDBY_R = crate::BitReader;
///Field `IWGD_STDBY` writer - None
pub type IWGD_STDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Window watchdog selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDG_SW {
    ///0: Hardware window watchdog
    B0x0 = 0,
    ///1: Software window watchdog
    B0x1 = 1,
}
impl From<WWDG_SW> for bool {
    #[inline(always)]
    fn from(variant: WWDG_SW) -> Self {
        variant as u8 != 0
    }
}
///Field `WWDG_SW` reader - Window watchdog selection
pub type WWDG_SW_R = crate::BitReader<WWDG_SW>;
impl WWDG_SW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WWDG_SW {
        match self.bits {
            false => WWDG_SW::B0x0,
            true => WWDG_SW::B0x1,
        }
    }
    ///Hardware window watchdog
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WWDG_SW::B0x0
    }
    ///Software window watchdog
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WWDG_SW::B0x1
    }
}
///Field `WWDG_SW` writer - Window watchdog selection
pub type WWDG_SW_W<'a, REG> = crate::BitWriter<'a, REG, WWDG_SW>;
impl<'a, REG> WWDG_SW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Hardware window watchdog
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WWDG_SW::B0x0)
    }
    ///Software window watchdog
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WWDG_SW::B0x1)
    }
}
/**HSE remapping enable/disable When cleared, the bit remaps the HSE clock source from PF0-OSC_IN/PF1-OSC_OUT pins to PC14-OSCX_IN/PC15-OSCX_OUT. Thus PC14-OSCX_IN/PC15-OSCX_OUT are shared by both LSE and HSE and the two clock sources cannot be use simultaneously. On packages with less than 48 pins, the remapping is always enabled (PF0-OSC_IN/PF1-OSC_OUT are not available), regardless of this bit. As all STM32C011xx packages have less than 48 pins, this bit is only applicable to STM32C031xx. Note: On 48 pins packages, when HSE_NOT_REMAPPED is reset, HSE cannot be used in bypass mode. Refer to product errata sheet for more details.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSE_NOT_REMAPPED {
    ///0: Enable
    B0x0 = 0,
    ///1: Disable
    B0x1 = 1,
}
impl From<HSE_NOT_REMAPPED> for bool {
    #[inline(always)]
    fn from(variant: HSE_NOT_REMAPPED) -> Self {
        variant as u8 != 0
    }
}
///Field `HSE_NOT_REMAPPED` reader - HSE remapping enable/disable When cleared, the bit remaps the HSE clock source from PF0-OSC_IN/PF1-OSC_OUT pins to PC14-OSCX_IN/PC15-OSCX_OUT. Thus PC14-OSCX_IN/PC15-OSCX_OUT are shared by both LSE and HSE and the two clock sources cannot be use simultaneously. On packages with less than 48 pins, the remapping is always enabled (PF0-OSC_IN/PF1-OSC_OUT are not available), regardless of this bit. As all STM32C011xx packages have less than 48 pins, this bit is only applicable to STM32C031xx. Note: On 48 pins packages, when HSE_NOT_REMAPPED is reset, HSE cannot be used in bypass mode. Refer to product errata sheet for more details.
pub type HSE_NOT_REMAPPED_R = crate::BitReader<HSE_NOT_REMAPPED>;
impl HSE_NOT_REMAPPED_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSE_NOT_REMAPPED {
        match self.bits {
            false => HSE_NOT_REMAPPED::B0x0,
            true => HSE_NOT_REMAPPED::B0x1,
        }
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSE_NOT_REMAPPED::B0x0
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSE_NOT_REMAPPED::B0x1
    }
}
///Field `HSE_NOT_REMAPPED` writer - HSE remapping enable/disable When cleared, the bit remaps the HSE clock source from PF0-OSC_IN/PF1-OSC_OUT pins to PC14-OSCX_IN/PC15-OSCX_OUT. Thus PC14-OSCX_IN/PC15-OSCX_OUT are shared by both LSE and HSE and the two clock sources cannot be use simultaneously. On packages with less than 48 pins, the remapping is always enabled (PF0-OSC_IN/PF1-OSC_OUT are not available), regardless of this bit. As all STM32C011xx packages have less than 48 pins, this bit is only applicable to STM32C031xx. Note: On 48 pins packages, when HSE_NOT_REMAPPED is reset, HSE cannot be used in bypass mode. Refer to product errata sheet for more details.
pub type HSE_NOT_REMAPPED_W<'a, REG> = crate::BitWriter<'a, REG, HSE_NOT_REMAPPED>;
impl<'a, REG> HSE_NOT_REMAPPED_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSE_NOT_REMAPPED::B0x0)
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSE_NOT_REMAPPED::B0x1)
    }
}
/**SRAM parity check control enable/disable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM_PARITY_CHECK {
    ///0: Enable
    B0x0 = 0,
    ///1: Disable
    B0x1 = 1,
}
impl From<RAM_PARITY_CHECK> for bool {
    #[inline(always)]
    fn from(variant: RAM_PARITY_CHECK) -> Self {
        variant as u8 != 0
    }
}
///Field `RAM_PARITY_CHECK` reader - SRAM parity check control enable/disable
pub type RAM_PARITY_CHECK_R = crate::BitReader<RAM_PARITY_CHECK>;
impl RAM_PARITY_CHECK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RAM_PARITY_CHECK {
        match self.bits {
            false => RAM_PARITY_CHECK::B0x0,
            true => RAM_PARITY_CHECK::B0x1,
        }
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RAM_PARITY_CHECK::B0x0
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RAM_PARITY_CHECK::B0x1
    }
}
///Field `RAM_PARITY_CHECK` writer - SRAM parity check control enable/disable
pub type RAM_PARITY_CHECK_W<'a, REG> = crate::BitWriter<'a, REG, RAM_PARITY_CHECK>;
impl<'a, REG> RAM_PARITY_CHECK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RAM_PARITY_CHECK::B0x0)
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RAM_PARITY_CHECK::B0x1)
    }
}
/**Multiple-bonding security The bit allows enabling automatic I/O configuration to prevent conflicts on I/Os connected (bonded) onto the same pin. If the software sets one of the I/Os connected to the same pin as active by configuring the SYSCFG_CFGR3 register, enabling this bit automatically forces the other I/Os in digital input mode, regardless of their software configuration. When the bit is disabled, the SYSCFG_CFGR3 register setting is ignored, all GPIOs linked to a given pin are active and can be set in the mode specified by the corresponding GPIOx_MODER register. The user software must ensure that there is no conflict between GPIOs.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECURE_MUXING_EN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<SECURE_MUXING_EN> for bool {
    #[inline(always)]
    fn from(variant: SECURE_MUXING_EN) -> Self {
        variant as u8 != 0
    }
}
///Field `SECURE_MUXING_EN` reader - Multiple-bonding security The bit allows enabling automatic I/O configuration to prevent conflicts on I/Os connected (bonded) onto the same pin. If the software sets one of the I/Os connected to the same pin as active by configuring the SYSCFG_CFGR3 register, enabling this bit automatically forces the other I/Os in digital input mode, regardless of their software configuration. When the bit is disabled, the SYSCFG_CFGR3 register setting is ignored, all GPIOs linked to a given pin are active and can be set in the mode specified by the corresponding GPIOx_MODER register. The user software must ensure that there is no conflict between GPIOs.
pub type SECURE_MUXING_EN_R = crate::BitReader<SECURE_MUXING_EN>;
impl SECURE_MUXING_EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SECURE_MUXING_EN {
        match self.bits {
            false => SECURE_MUXING_EN::B0x0,
            true => SECURE_MUXING_EN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SECURE_MUXING_EN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SECURE_MUXING_EN::B0x1
    }
}
///Field `SECURE_MUXING_EN` writer - Multiple-bonding security The bit allows enabling automatic I/O configuration to prevent conflicts on I/Os connected (bonded) onto the same pin. If the software sets one of the I/Os connected to the same pin as active by configuring the SYSCFG_CFGR3 register, enabling this bit automatically forces the other I/Os in digital input mode, regardless of their software configuration. When the bit is disabled, the SYSCFG_CFGR3 register setting is ignored, all GPIOs linked to a given pin are active and can be set in the mode specified by the corresponding GPIOx_MODER register. The user software must ensure that there is no conflict between GPIOs.
pub type SECURE_MUXING_EN_W<'a, REG> = crate::BitWriter<'a, REG, SECURE_MUXING_EN>;
impl<'a, REG> SECURE_MUXING_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SECURE_MUXING_EN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SECURE_MUXING_EN::B0x1)
    }
}
/**BOOT0 signal source selection This option bit defines the source of the BOOT0 signal.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NBOOT_SEL {
    ///0: BOOT0 pin (legacy mode)
    B0x0 = 0,
    ///1: nBOOT0 option bit
    B0x1 = 1,
}
impl From<NBOOT_SEL> for bool {
    #[inline(always)]
    fn from(variant: NBOOT_SEL) -> Self {
        variant as u8 != 0
    }
}
///Field `NBOOT_SEL` reader - BOOT0 signal source selection This option bit defines the source of the BOOT0 signal.
pub type NBOOT_SEL_R = crate::BitReader<NBOOT_SEL>;
impl NBOOT_SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NBOOT_SEL {
        match self.bits {
            false => NBOOT_SEL::B0x0,
            true => NBOOT_SEL::B0x1,
        }
    }
    ///BOOT0 pin (legacy mode)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == NBOOT_SEL::B0x0
    }
    ///nBOOT0 option bit
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == NBOOT_SEL::B0x1
    }
}
///Field `NBOOT_SEL` writer - BOOT0 signal source selection This option bit defines the source of the BOOT0 signal.
pub type NBOOT_SEL_W<'a, REG> = crate::BitWriter<'a, REG, NBOOT_SEL>;
impl<'a, REG> NBOOT_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///BOOT0 pin (legacy mode)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(NBOOT_SEL::B0x0)
    }
    ///nBOOT0 option bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(NBOOT_SEL::B0x1)
    }
}
///Field `NBOOT1` reader - Boot configuration Together with the BOOT0 pin or option bit nBOOT0 (depending on nBOOT_SEL option bit configuration), this bit selects boot mode from the Main flash memory, SRAM or the System memory. Refer to Section 3: Boot configuration.
pub type NBOOT1_R = crate::BitReader;
///Field `NBOOT1` writer - Boot configuration Together with the BOOT0 pin or option bit nBOOT0 (depending on nBOOT_SEL option bit configuration), this bit selects boot mode from the Main flash memory, SRAM or the System memory. Refer to Section 3: Boot configuration.
pub type NBOOT1_W<'a, REG> = crate::BitWriter<'a, REG>;
/**nBOOT0 option bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NBOOT0 {
    ///0: nBOOT0 = 0
    B0x0 = 0,
    ///1: nBOOT0 = 1
    B0x1 = 1,
}
impl From<NBOOT0> for bool {
    #[inline(always)]
    fn from(variant: NBOOT0) -> Self {
        variant as u8 != 0
    }
}
///Field `NBOOT0` reader - nBOOT0 option bit
pub type NBOOT0_R = crate::BitReader<NBOOT0>;
impl NBOOT0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NBOOT0 {
        match self.bits {
            false => NBOOT0::B0x0,
            true => NBOOT0::B0x1,
        }
    }
    ///nBOOT0 = 0
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == NBOOT0::B0x0
    }
    ///nBOOT0 = 1
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == NBOOT0::B0x1
    }
}
///Field `NBOOT0` writer - nBOOT0 option bit
pub type NBOOT0_W<'a, REG> = crate::BitWriter<'a, REG, NBOOT0>;
impl<'a, REG> NBOOT0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///nBOOT0 = 0
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(NBOOT0::B0x0)
    }
    ///nBOOT0 = 1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(NBOOT0::B0x1)
    }
}
/**NRST pin configuration

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NRST_MODE {
    ///1: Reset input only: a low level on the NRST pin generates system reset; internal RESET is not propagated to the NRST pin.
    B0x1 = 1,
    ///2: Standard GPIO: only internal RESET is possible
    B0x2 = 2,
    ///3: Bidirectional reset: the NRST pin is configured in reset input/output (legacy) mode
    B0x3 = 3,
}
impl From<NRST_MODE> for u8 {
    #[inline(always)]
    fn from(variant: NRST_MODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NRST_MODE {
    type Ux = u8;
}
impl crate::IsEnum for NRST_MODE {}
///Field `NRST_MODE` reader - NRST pin configuration
pub type NRST_MODE_R = crate::FieldReader<NRST_MODE>;
impl NRST_MODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<NRST_MODE> {
        match self.bits {
            1 => Some(NRST_MODE::B0x1),
            2 => Some(NRST_MODE::B0x2),
            3 => Some(NRST_MODE::B0x3),
            _ => None,
        }
    }
    ///Reset input only: a low level on the NRST pin generates system reset; internal RESET is not propagated to the NRST pin.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == NRST_MODE::B0x1
    }
    ///Standard GPIO: only internal RESET is possible
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == NRST_MODE::B0x2
    }
    ///Bidirectional reset: the NRST pin is configured in reset input/output (legacy) mode
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == NRST_MODE::B0x3
    }
}
///Field `NRST_MODE` writer - NRST pin configuration
pub type NRST_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, NRST_MODE>;
impl<'a, REG> NRST_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Reset input only: a low level on the NRST pin generates system reset; internal RESET is not propagated to the NRST pin.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(NRST_MODE::B0x1)
    }
    ///Standard GPIO: only internal RESET is possible
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(NRST_MODE::B0x2)
    }
    ///Bidirectional reset: the NRST pin is configured in reset input/output (legacy) mode
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(NRST_MODE::B0x3)
    }
}
/**Internal reset holder enable bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRHEN {
    ///0: Internal resets are propagated as simple pulse on NRST pin
    B0x0 = 0,
    ///1: Internal resets drives NRST pin low until it is seen as low level
    B0x1 = 1,
}
impl From<IRHEN> for bool {
    #[inline(always)]
    fn from(variant: IRHEN) -> Self {
        variant as u8 != 0
    }
}
///Field `IRHEN` reader - Internal reset holder enable bit
pub type IRHEN_R = crate::BitReader<IRHEN>;
impl IRHEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IRHEN {
        match self.bits {
            false => IRHEN::B0x0,
            true => IRHEN::B0x1,
        }
    }
    ///Internal resets are propagated as simple pulse on NRST pin
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IRHEN::B0x0
    }
    ///Internal resets drives NRST pin low until it is seen as low level
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IRHEN::B0x1
    }
}
///Field `IRHEN` writer - Internal reset holder enable bit
pub type IRHEN_W<'a, REG> = crate::BitWriter<'a, REG, IRHEN>;
impl<'a, REG> IRHEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Internal resets are propagated as simple pulse on NRST pin
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IRHEN::B0x0)
    }
    ///Internal resets drives NRST pin low until it is seen as low level
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IRHEN::B0x1)
    }
}
impl R {
    ///Bits 0:7 - Read protection level Other: Level 1, memories read protection active
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - Brown out reset enable
    #[inline(always)]
    pub fn bor_en(&self) -> BOR_EN_R {
        BOR_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - BOR threshold at rising V<sub>DD</sub> supply Rising V<sub>DD</sub> crossings this threshold releases the reset signal.
    #[inline(always)]
    pub fn borr_lev(&self) -> BORR_LEV_R {
        BORR_LEV_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bits 11:12 - BOR threshold at falling V<sub>DD</sub> supply Falling V<sub>DD</sub> crossings this threshold activates the reset signal.
    #[inline(always)]
    pub fn borf_lev(&self) -> BORF_LEV_R {
        BORF_LEV_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bit 13 - None
    #[inline(always)]
    pub fn nrst_stop(&self) -> NRST_STOP_R {
        NRST_STOP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - None
    #[inline(always)]
    pub fn nrst_stdby(&self) -> NRST_STDBY_R {
        NRST_STDBY_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - None
    #[inline(always)]
    pub fn nrst_shdw(&self) -> NRST_SHDW_R {
        NRST_SHDW_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - None
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Independent watchdog counter freeze in Stop mode
    #[inline(always)]
    pub fn iwdg_stop(&self) -> IWDG_STOP_R {
        IWDG_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - None
    #[inline(always)]
    pub fn iwgd_stdby(&self) -> IWGD_STDBY_R {
        IWGD_STDBY_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Window watchdog selection
    #[inline(always)]
    pub fn wwdg_sw(&self) -> WWDG_SW_R {
        WWDG_SW_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - HSE remapping enable/disable When cleared, the bit remaps the HSE clock source from PF0-OSC_IN/PF1-OSC_OUT pins to PC14-OSCX_IN/PC15-OSCX_OUT. Thus PC14-OSCX_IN/PC15-OSCX_OUT are shared by both LSE and HSE and the two clock sources cannot be use simultaneously. On packages with less than 48 pins, the remapping is always enabled (PF0-OSC_IN/PF1-OSC_OUT are not available), regardless of this bit. As all STM32C011xx packages have less than 48 pins, this bit is only applicable to STM32C031xx. Note: On 48 pins packages, when HSE_NOT_REMAPPED is reset, HSE cannot be used in bypass mode. Refer to product errata sheet for more details.
    #[inline(always)]
    pub fn hse_not_remapped(&self) -> HSE_NOT_REMAPPED_R {
        HSE_NOT_REMAPPED_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SRAM parity check control enable/disable
    #[inline(always)]
    pub fn ram_parity_check(&self) -> RAM_PARITY_CHECK_R {
        RAM_PARITY_CHECK_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Multiple-bonding security The bit allows enabling automatic I/O configuration to prevent conflicts on I/Os connected (bonded) onto the same pin. If the software sets one of the I/Os connected to the same pin as active by configuring the SYSCFG_CFGR3 register, enabling this bit automatically forces the other I/Os in digital input mode, regardless of their software configuration. When the bit is disabled, the SYSCFG_CFGR3 register setting is ignored, all GPIOs linked to a given pin are active and can be set in the mode specified by the corresponding GPIOx_MODER register. The user software must ensure that there is no conflict between GPIOs.
    #[inline(always)]
    pub fn secure_muxing_en(&self) -> SECURE_MUXING_EN_R {
        SECURE_MUXING_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - BOOT0 signal source selection This option bit defines the source of the BOOT0 signal.
    #[inline(always)]
    pub fn nboot_sel(&self) -> NBOOT_SEL_R {
        NBOOT_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Boot configuration Together with the BOOT0 pin or option bit nBOOT0 (depending on nBOOT_SEL option bit configuration), this bit selects boot mode from the Main flash memory, SRAM or the System memory. Refer to Section 3: Boot configuration.
    #[inline(always)]
    pub fn nboot1(&self) -> NBOOT1_R {
        NBOOT1_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - nBOOT0 option bit
    #[inline(always)]
    pub fn nboot0(&self) -> NBOOT0_R {
        NBOOT0_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bits 27:28 - NRST pin configuration
    #[inline(always)]
    pub fn nrst_mode(&self) -> NRST_MODE_R {
        NRST_MODE_R::new(((self.bits >> 27) & 3) as u8)
    }
    ///Bit 29 - Internal reset holder enable bit
    #[inline(always)]
    pub fn irhen(&self) -> IRHEN_R {
        IRHEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTR")
            .field("rdp", &self.rdp())
            .field("bor_en", &self.bor_en())
            .field("borr_lev", &self.borr_lev())
            .field("borf_lev", &self.borf_lev())
            .field("nrst_stop", &self.nrst_stop())
            .field("nrst_stdby", &self.nrst_stdby())
            .field("nrst_shdw", &self.nrst_shdw())
            .field("iwdg_sw", &self.iwdg_sw())
            .field("iwdg_stop", &self.iwdg_stop())
            .field("iwgd_stdby", &self.iwgd_stdby())
            .field("wwdg_sw", &self.wwdg_sw())
            .field("hse_not_remapped", &self.hse_not_remapped())
            .field("ram_parity_check", &self.ram_parity_check())
            .field("secure_muxing_en", &self.secure_muxing_en())
            .field("nboot_sel", &self.nboot_sel())
            .field("nboot1", &self.nboot1())
            .field("nboot0", &self.nboot0())
            .field("nrst_mode", &self.nrst_mode())
            .field("irhen", &self.irhen())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Read protection level Other: Level 1, memories read protection active
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W<'_, OPTRrs> {
        RDP_W::new(self, 0)
    }
    ///Bit 8 - Brown out reset enable
    #[inline(always)]
    pub fn bor_en(&mut self) -> BOR_EN_W<'_, OPTRrs> {
        BOR_EN_W::new(self, 8)
    }
    ///Bits 9:10 - BOR threshold at rising V<sub>DD</sub> supply Rising V<sub>DD</sub> crossings this threshold releases the reset signal.
    #[inline(always)]
    pub fn borr_lev(&mut self) -> BORR_LEV_W<'_, OPTRrs> {
        BORR_LEV_W::new(self, 9)
    }
    ///Bits 11:12 - BOR threshold at falling V<sub>DD</sub> supply Falling V<sub>DD</sub> crossings this threshold activates the reset signal.
    #[inline(always)]
    pub fn borf_lev(&mut self) -> BORF_LEV_W<'_, OPTRrs> {
        BORF_LEV_W::new(self, 11)
    }
    ///Bit 13 - None
    #[inline(always)]
    pub fn nrst_stop(&mut self) -> NRST_STOP_W<'_, OPTRrs> {
        NRST_STOP_W::new(self, 13)
    }
    ///Bit 14 - None
    #[inline(always)]
    pub fn nrst_stdby(&mut self) -> NRST_STDBY_W<'_, OPTRrs> {
        NRST_STDBY_W::new(self, 14)
    }
    ///Bit 15 - None
    #[inline(always)]
    pub fn nrst_shdw(&mut self) -> NRST_SHDW_W<'_, OPTRrs> {
        NRST_SHDW_W::new(self, 15)
    }
    ///Bit 16 - None
    #[inline(always)]
    pub fn iwdg_sw(&mut self) -> IWDG_SW_W<'_, OPTRrs> {
        IWDG_SW_W::new(self, 16)
    }
    ///Bit 17 - Independent watchdog counter freeze in Stop mode
    #[inline(always)]
    pub fn iwdg_stop(&mut self) -> IWDG_STOP_W<'_, OPTRrs> {
        IWDG_STOP_W::new(self, 17)
    }
    ///Bit 18 - None
    #[inline(always)]
    pub fn iwgd_stdby(&mut self) -> IWGD_STDBY_W<'_, OPTRrs> {
        IWGD_STDBY_W::new(self, 18)
    }
    ///Bit 19 - Window watchdog selection
    #[inline(always)]
    pub fn wwdg_sw(&mut self) -> WWDG_SW_W<'_, OPTRrs> {
        WWDG_SW_W::new(self, 19)
    }
    ///Bit 21 - HSE remapping enable/disable When cleared, the bit remaps the HSE clock source from PF0-OSC_IN/PF1-OSC_OUT pins to PC14-OSCX_IN/PC15-OSCX_OUT. Thus PC14-OSCX_IN/PC15-OSCX_OUT are shared by both LSE and HSE and the two clock sources cannot be use simultaneously. On packages with less than 48 pins, the remapping is always enabled (PF0-OSC_IN/PF1-OSC_OUT are not available), regardless of this bit. As all STM32C011xx packages have less than 48 pins, this bit is only applicable to STM32C031xx. Note: On 48 pins packages, when HSE_NOT_REMAPPED is reset, HSE cannot be used in bypass mode. Refer to product errata sheet for more details.
    #[inline(always)]
    pub fn hse_not_remapped(&mut self) -> HSE_NOT_REMAPPED_W<'_, OPTRrs> {
        HSE_NOT_REMAPPED_W::new(self, 21)
    }
    ///Bit 22 - SRAM parity check control enable/disable
    #[inline(always)]
    pub fn ram_parity_check(&mut self) -> RAM_PARITY_CHECK_W<'_, OPTRrs> {
        RAM_PARITY_CHECK_W::new(self, 22)
    }
    ///Bit 23 - Multiple-bonding security The bit allows enabling automatic I/O configuration to prevent conflicts on I/Os connected (bonded) onto the same pin. If the software sets one of the I/Os connected to the same pin as active by configuring the SYSCFG_CFGR3 register, enabling this bit automatically forces the other I/Os in digital input mode, regardless of their software configuration. When the bit is disabled, the SYSCFG_CFGR3 register setting is ignored, all GPIOs linked to a given pin are active and can be set in the mode specified by the corresponding GPIOx_MODER register. The user software must ensure that there is no conflict between GPIOs.
    #[inline(always)]
    pub fn secure_muxing_en(&mut self) -> SECURE_MUXING_EN_W<'_, OPTRrs> {
        SECURE_MUXING_EN_W::new(self, 23)
    }
    ///Bit 24 - BOOT0 signal source selection This option bit defines the source of the BOOT0 signal.
    #[inline(always)]
    pub fn nboot_sel(&mut self) -> NBOOT_SEL_W<'_, OPTRrs> {
        NBOOT_SEL_W::new(self, 24)
    }
    ///Bit 25 - Boot configuration Together with the BOOT0 pin or option bit nBOOT0 (depending on nBOOT_SEL option bit configuration), this bit selects boot mode from the Main flash memory, SRAM or the System memory. Refer to Section 3: Boot configuration.
    #[inline(always)]
    pub fn nboot1(&mut self) -> NBOOT1_W<'_, OPTRrs> {
        NBOOT1_W::new(self, 25)
    }
    ///Bit 26 - nBOOT0 option bit
    #[inline(always)]
    pub fn nboot0(&mut self) -> NBOOT0_W<'_, OPTRrs> {
        NBOOT0_W::new(self, 26)
    }
    ///Bits 27:28 - NRST pin configuration
    #[inline(always)]
    pub fn nrst_mode(&mut self) -> NRST_MODE_W<'_, OPTRrs> {
        NRST_MODE_W::new(self, 27)
    }
    ///Bit 29 - Internal reset holder enable bit
    #[inline(always)]
    pub fn irhen(&mut self) -> IRHEN_W<'_, OPTRrs> {
        IRHEN_W::new(self, 29)
    }
}
/**FLASH option register

You can [`read`](crate::Reg::read) this register and get [`optr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#FLASH:OPTR)*/
pub struct OPTRrs;
impl crate::RegisterSpec for OPTRrs {
    type Ux = u32;
}
///`read()` method returns [`optr::R`](R) reader structure
impl crate::Readable for OPTRrs {}
///`write(|w| ..)` method takes [`optr::W`](W) writer structure
impl crate::Writable for OPTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPTR to value 0
impl crate::Resettable for OPTRrs {}
