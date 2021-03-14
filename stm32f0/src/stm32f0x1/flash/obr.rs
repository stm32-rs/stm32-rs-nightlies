#[doc = "Reader of register OBR"]
pub type R = crate::R<u32, super::OBR>;
#[doc = "Option byte error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPTERR_A {
    #[doc = "1: The loaded option byte and its complement do not match"]
    OPTIONBYTEERROR = 1,
}
impl From<OPTERR_A> for bool {
    #[inline(always)]
    fn from(variant: OPTERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OPTERR`"]
pub type OPTERR_R = crate::R<bool, OPTERR_A>;
impl OPTERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, OPTERR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(OPTERR_A::OPTIONBYTEERROR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OPTIONBYTEERROR`"]
    #[inline(always)]
    pub fn is_option_byte_error(&self) -> bool {
        *self == OPTERR_A::OPTIONBYTEERROR
    }
}
#[doc = "Read protection level status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RDPRT_A {
    #[doc = "0: Level 0"]
    LEVEL0 = 0,
    #[doc = "1: Level 1"]
    LEVEL1 = 1,
    #[doc = "3: Level 2"]
    LEVEL2 = 3,
}
impl From<RDPRT_A> for u8 {
    #[inline(always)]
    fn from(variant: RDPRT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RDPRT`"]
pub type RDPRT_R = crate::R<u8, RDPRT_A>;
impl RDPRT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RDPRT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RDPRT_A::LEVEL0),
            1 => Val(RDPRT_A::LEVEL1),
            3 => Val(RDPRT_A::LEVEL2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL0`"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == RDPRT_A::LEVEL0
    }
    #[doc = "Checks if the value of the field is `LEVEL1`"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == RDPRT_A::LEVEL1
    }
    #[doc = "Checks if the value of the field is `LEVEL2`"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == RDPRT_A::LEVEL2
    }
}
#[doc = "WDG_SW\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDG_SW_A {
    #[doc = "0: Hardware watchdog"]
    HARDWARE = 0,
    #[doc = "1: Software watchdog"]
    SOFTWARE = 1,
}
impl From<WDG_SW_A> for bool {
    #[inline(always)]
    fn from(variant: WDG_SW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WDG_SW`"]
pub type WDG_SW_R = crate::R<bool, WDG_SW_A>;
impl WDG_SW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDG_SW_A {
        match self.bits {
            false => WDG_SW_A::HARDWARE,
            true => WDG_SW_A::SOFTWARE,
        }
    }
    #[doc = "Checks if the value of the field is `HARDWARE`"]
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == WDG_SW_A::HARDWARE
    }
    #[doc = "Checks if the value of the field is `SOFTWARE`"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == WDG_SW_A::SOFTWARE
    }
}
#[doc = "nRST_STOP\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NRST_STOP_A {
    #[doc = "0: Reset generated when entering Stop mode"]
    RESET = 0,
    #[doc = "1: No reset generated"]
    NORESET = 1,
}
impl From<NRST_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: NRST_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `nRST_STOP`"]
pub type NRST_STOP_R = crate::R<bool, NRST_STOP_A>;
impl NRST_STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NRST_STOP_A {
        match self.bits {
            false => NRST_STOP_A::RESET,
            true => NRST_STOP_A::NORESET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == NRST_STOP_A::RESET
    }
    #[doc = "Checks if the value of the field is `NORESET`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == NRST_STOP_A::NORESET
    }
}
#[doc = "nRST_STDBY\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NRST_STDBY_A {
    #[doc = "0: Reset generated when entering Standby mode"]
    RESET = 0,
    #[doc = "1: No reset generated"]
    NORESET = 1,
}
impl From<NRST_STDBY_A> for bool {
    #[inline(always)]
    fn from(variant: NRST_STDBY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `nRST_STDBY`"]
pub type NRST_STDBY_R = crate::R<bool, NRST_STDBY_A>;
impl NRST_STDBY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NRST_STDBY_A {
        match self.bits {
            false => NRST_STDBY_A::RESET,
            true => NRST_STDBY_A::NORESET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == NRST_STDBY_A::RESET
    }
    #[doc = "Checks if the value of the field is `NORESET`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == NRST_STDBY_A::NORESET
    }
}
#[doc = "nBOOT0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NBOOT0_A {
    #[doc = "0: When BOOT_SEL is cleared, select the device boot mode"]
    DISABLED = 0,
    #[doc = "1: When BOOT_SEL is cleared, select the device boot mode"]
    ENABLED = 1,
}
impl From<NBOOT0_A> for bool {
    #[inline(always)]
    fn from(variant: NBOOT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `nBOOT0`"]
pub type NBOOT0_R = crate::R<bool, NBOOT0_A>;
impl NBOOT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NBOOT0_A {
        match self.bits {
            false => NBOOT0_A::DISABLED,
            true => NBOOT0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NBOOT0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NBOOT0_A::ENABLED
    }
}
#[doc = "BOOT1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NBOOT1_A {
    #[doc = "0: Together with BOOT0, select the device boot mode"]
    DISABLED = 0,
    #[doc = "1: Together with BOOT0, select the device boot mode"]
    ENABLED = 1,
}
impl From<NBOOT1_A> for bool {
    #[inline(always)]
    fn from(variant: NBOOT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `nBOOT1`"]
pub type NBOOT1_R = crate::R<bool, NBOOT1_A>;
impl NBOOT1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NBOOT1_A {
        match self.bits {
            false => NBOOT1_A::DISABLED,
            true => NBOOT1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NBOOT1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NBOOT1_A::ENABLED
    }
}
#[doc = "VDDA_MONITOR\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDA_MONITOR_A {
    #[doc = "0: VDDA power supply supervisor disabled"]
    DISABLED = 0,
    #[doc = "1: VDDA power supply supervisor enabled"]
    ENABLED = 1,
}
impl From<VDDA_MONITOR_A> for bool {
    #[inline(always)]
    fn from(variant: VDDA_MONITOR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VDDA_MONITOR`"]
pub type VDDA_MONITOR_R = crate::R<bool, VDDA_MONITOR_A>;
impl VDDA_MONITOR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDA_MONITOR_A {
        match self.bits {
            false => VDDA_MONITOR_A::DISABLED,
            true => VDDA_MONITOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VDDA_MONITOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VDDA_MONITOR_A::ENABLED
    }
}
#[doc = "RAM_PARITY_CHECK\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_PARITY_CHECK_A {
    #[doc = "1: RAM parity check disabled"]
    DISABLED = 1,
    #[doc = "0: RAM parity check enabled"]
    ENABLED = 0,
}
impl From<RAM_PARITY_CHECK_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_PARITY_CHECK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RAM_PARITY_CHECK`"]
pub type RAM_PARITY_CHECK_R = crate::R<bool, RAM_PARITY_CHECK_A>;
impl RAM_PARITY_CHECK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_PARITY_CHECK_A {
        match self.bits {
            true => RAM_PARITY_CHECK_A::DISABLED,
            false => RAM_PARITY_CHECK_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RAM_PARITY_CHECK_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RAM_PARITY_CHECK_A::ENABLED
    }
}
#[doc = "BOOT_SEL\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOOT_SEL_A {
    #[doc = "0: BOOT0 signal is defined by nBOOT0 option bit"]
    NBOOT0 = 0,
    #[doc = "1: BOOT0 signal is defined by BOOT0 pin value (legacy mode)"]
    BOOT0 = 1,
}
impl From<BOOT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: BOOT_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BOOT_SEL`"]
pub type BOOT_SEL_R = crate::R<bool, BOOT_SEL_A>;
impl BOOT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOT_SEL_A {
        match self.bits {
            false => BOOT_SEL_A::NBOOT0,
            true => BOOT_SEL_A::BOOT0,
        }
    }
    #[doc = "Checks if the value of the field is `NBOOT0`"]
    #[inline(always)]
    pub fn is_n_boot0(&self) -> bool {
        *self == BOOT_SEL_A::NBOOT0
    }
    #[doc = "Checks if the value of the field is `BOOT0`"]
    #[inline(always)]
    pub fn is_boot0(&self) -> bool {
        *self == BOOT_SEL_A::BOOT0
    }
}
#[doc = "Reader of field `Data0`"]
pub type DATA0_R = crate::R<u8, u8>;
#[doc = "Reader of field `Data1`"]
pub type DATA1_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Option byte error"]
    #[inline(always)]
    pub fn opterr(&self) -> OPTERR_R {
        OPTERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Read protection level status"]
    #[inline(always)]
    pub fn rdprt(&self) -> RDPRT_R {
        RDPRT_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 8 - WDG_SW"]
    #[inline(always)]
    pub fn wdg_sw(&self) -> WDG_SW_R {
        WDG_SW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - nRST_STOP"]
    #[inline(always)]
    pub fn n_rst_stop(&self) -> NRST_STOP_R {
        NRST_STOP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - nRST_STDBY"]
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> NRST_STDBY_R {
        NRST_STDBY_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - nBOOT0"]
    #[inline(always)]
    pub fn n_boot0(&self) -> NBOOT0_R {
        NBOOT0_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - BOOT1"]
    #[inline(always)]
    pub fn n_boot1(&self) -> NBOOT1_R {
        NBOOT1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - VDDA_MONITOR"]
    #[inline(always)]
    pub fn vdda_monitor(&self) -> VDDA_MONITOR_R {
        VDDA_MONITOR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RAM_PARITY_CHECK"]
    #[inline(always)]
    pub fn ram_parity_check(&self) -> RAM_PARITY_CHECK_R {
        RAM_PARITY_CHECK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - BOOT_SEL"]
    #[inline(always)]
    pub fn boot_sel(&self) -> BOOT_SEL_R {
        BOOT_SEL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Data0"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data1"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
