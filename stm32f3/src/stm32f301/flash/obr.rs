#[doc = "Register `OBR` reader"]
pub type R = crate::R<OBRrs>;
#[doc = "Option byte error\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTERR {
    #[doc = "1: The loaded option byte and its complement do not match"]
    OptionByteError = 1,
}
impl From<OPTERR> for bool {
    #[inline(always)]
    fn from(variant: OPTERR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPTERR` reader - Option byte error"]
pub type OPTERR_R = crate::BitReader<OPTERR>;
impl OPTERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OPTERR> {
        match self.bits {
            true => Some(OPTERR::OptionByteError),
            _ => None,
        }
    }
    #[doc = "The loaded option byte and its complement do not match"]
    #[inline(always)]
    pub fn is_option_byte_error(&self) -> bool {
        *self == OPTERR::OptionByteError
    }
}
#[doc = "Read protection Level status\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RDPRT {
    #[doc = "0: Level 0"]
    Level0 = 0,
    #[doc = "1: Level 1"]
    Level1 = 1,
    #[doc = "3: Level 2"]
    Level2 = 3,
}
impl From<RDPRT> for u8 {
    #[inline(always)]
    fn from(variant: RDPRT) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RDPRT {
    type Ux = u8;
}
#[doc = "Field `RDPRT` reader - Read protection Level status"]
pub type RDPRT_R = crate::FieldReader<RDPRT>;
impl RDPRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RDPRT> {
        match self.bits {
            0 => Some(RDPRT::Level0),
            1 => Some(RDPRT::Level1),
            3 => Some(RDPRT::Level2),
            _ => None,
        }
    }
    #[doc = "Level 0"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == RDPRT::Level0
    }
    #[doc = "Level 1"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == RDPRT::Level1
    }
    #[doc = "Level 2"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == RDPRT::Level2
    }
}
#[doc = "WDG_SW\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDG_SW {
    #[doc = "0: Hardware watchdog"]
    Hardware = 0,
    #[doc = "1: Software watchdog"]
    Software = 1,
}
impl From<WDG_SW> for bool {
    #[inline(always)]
    fn from(variant: WDG_SW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDG_SW` reader - WDG_SW"]
pub type WDG_SW_R = crate::BitReader<WDG_SW>;
impl WDG_SW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WDG_SW {
        match self.bits {
            false => WDG_SW::Hardware,
            true => WDG_SW::Software,
        }
    }
    #[doc = "Hardware watchdog"]
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == WDG_SW::Hardware
    }
    #[doc = "Software watchdog"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == WDG_SW::Software
    }
}
#[doc = "nRST_STOP\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_RST_STOP {
    #[doc = "0: Reset generated when entering Stop mode"]
    Reset = 0,
    #[doc = "1: No reset generated"]
    NoReset = 1,
}
impl From<N_RST_STOP> for bool {
    #[inline(always)]
    fn from(variant: N_RST_STOP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `nRST_STOP` reader - nRST_STOP"]
pub type N_RST_STOP_R = crate::BitReader<N_RST_STOP>;
impl N_RST_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> N_RST_STOP {
        match self.bits {
            false => N_RST_STOP::Reset,
            true => N_RST_STOP::NoReset,
        }
    }
    #[doc = "Reset generated when entering Stop mode"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == N_RST_STOP::Reset
    }
    #[doc = "No reset generated"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == N_RST_STOP::NoReset
    }
}
#[doc = "nRST_STDBY\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_RST_STDBY {
    #[doc = "0: Reset generated when entering Standby mode"]
    Reset = 0,
    #[doc = "1: No reset generated"]
    NoReset = 1,
}
impl From<N_RST_STDBY> for bool {
    #[inline(always)]
    fn from(variant: N_RST_STDBY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `nRST_STDBY` reader - nRST_STDBY"]
pub type N_RST_STDBY_R = crate::BitReader<N_RST_STDBY>;
impl N_RST_STDBY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> N_RST_STDBY {
        match self.bits {
            false => N_RST_STDBY::Reset,
            true => N_RST_STDBY::NoReset,
        }
    }
    #[doc = "Reset generated when entering Standby mode"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == N_RST_STDBY::Reset
    }
    #[doc = "No reset generated"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == N_RST_STDBY::NoReset
    }
}
#[doc = "BOOT1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_BOOT1 {
    #[doc = "0: Together with BOOT0, select the device boot mode"]
    Disabled = 0,
    #[doc = "1: Together with BOOT0, select the device boot mode"]
    Enabled = 1,
}
impl From<N_BOOT1> for bool {
    #[inline(always)]
    fn from(variant: N_BOOT1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `nBOOT1` reader - BOOT1"]
pub type N_BOOT1_R = crate::BitReader<N_BOOT1>;
impl N_BOOT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> N_BOOT1 {
        match self.bits {
            false => N_BOOT1::Disabled,
            true => N_BOOT1::Enabled,
        }
    }
    #[doc = "Together with BOOT0, select the device boot mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == N_BOOT1::Disabled
    }
    #[doc = "Together with BOOT0, select the device boot mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == N_BOOT1::Enabled
    }
}
#[doc = "VDDA_MONITOR\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDDA_MONITOR {
    #[doc = "0: VDDA power supply supervisor disabled"]
    Disabled = 0,
    #[doc = "1: VDDA power supply supervisor enabled"]
    Enabled = 1,
}
impl From<VDDA_MONITOR> for bool {
    #[inline(always)]
    fn from(variant: VDDA_MONITOR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDDA_MONITOR` reader - VDDA_MONITOR"]
pub type VDDA_MONITOR_R = crate::BitReader<VDDA_MONITOR>;
impl VDDA_MONITOR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VDDA_MONITOR {
        match self.bits {
            false => VDDA_MONITOR::Disabled,
            true => VDDA_MONITOR::Enabled,
        }
    }
    #[doc = "VDDA power supply supervisor disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VDDA_MONITOR::Disabled
    }
    #[doc = "VDDA power supply supervisor enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VDDA_MONITOR::Enabled
    }
}
#[doc = "Field `Data0` reader - Data0"]
pub type DATA0_R = crate::FieldReader;
#[doc = "Field `Data1` reader - Data1"]
pub type DATA1_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Option byte error"]
    #[inline(always)]
    pub fn opterr(&self) -> OPTERR_R {
        OPTERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Read protection Level status"]
    #[inline(always)]
    pub fn rdprt(&self) -> RDPRT_R {
        RDPRT_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 8 - WDG_SW"]
    #[inline(always)]
    pub fn wdg_sw(&self) -> WDG_SW_R {
        WDG_SW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - nRST_STOP"]
    #[inline(always)]
    pub fn n_rst_stop(&self) -> N_RST_STOP_R {
        N_RST_STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - nRST_STDBY"]
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> N_RST_STDBY_R {
        N_RST_STDBY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - BOOT1"]
    #[inline(always)]
    pub fn n_boot1(&self) -> N_BOOT1_R {
        N_BOOT1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - VDDA_MONITOR"]
    #[inline(always)]
    pub fn vdda_monitor(&self) -> VDDA_MONITOR_R {
        VDDA_MONITOR_R::new(((self.bits >> 13) & 1) != 0)
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
#[doc = "Option byte register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OBRrs;
impl crate::RegisterSpec for OBRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`obr::R`](R) reader structure"]
impl crate::Readable for OBRrs {}
#[doc = "`reset()` method sets OBR to value 0xffff_ff0f"]
impl crate::Resettable for OBRrs {
    const RESET_VALUE: u32 = 0xffff_ff0f;
}
