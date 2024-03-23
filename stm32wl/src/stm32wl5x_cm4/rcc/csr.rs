#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSRrs>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CSRrs>;
#[doc = "LSI oscillator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSION {
    #[doc = "0: LSI oscillator off"]
    Off = 0,
    #[doc = "1: LSI oscillator on"]
    On = 1,
}
impl From<LSION> for bool {
    #[inline(always)]
    fn from(variant: LSION) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSION` reader - LSI oscillator enable"]
pub type LSION_R = crate::BitReader<LSION>;
impl LSION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSION {
        match self.bits {
            false => LSION::Off,
            true => LSION::On,
        }
    }
    #[doc = "LSI oscillator off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LSION::Off
    }
    #[doc = "LSI oscillator on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == LSION::On
    }
}
#[doc = "Field `LSION` writer - LSI oscillator enable"]
pub type LSION_W<'a, REG> = crate::BitWriter<'a, REG, LSION>;
impl<'a, REG> LSION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSI oscillator off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(LSION::Off)
    }
    #[doc = "LSI oscillator on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(LSION::On)
    }
}
#[doc = "LSI oscillator ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDY {
    #[doc = "0: LSI oscillator not ready"]
    NotReady = 0,
    #[doc = "1: LSI oscillator ready"]
    Ready = 1,
}
impl From<LSIRDY> for bool {
    #[inline(always)]
    fn from(variant: LSIRDY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDY` reader - LSI oscillator ready"]
pub type LSIRDY_R = crate::BitReader<LSIRDY>;
impl LSIRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSIRDY {
        match self.bits {
            false => LSIRDY::NotReady,
            true => LSIRDY::Ready,
        }
    }
    #[doc = "LSI oscillator not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LSIRDY::NotReady
    }
    #[doc = "LSI oscillator ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LSIRDY::Ready
    }
}
#[doc = "LSI frequency prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIPRE {
    #[doc = "0: LSI clock not divided"]
    Div1 = 0,
    #[doc = "1: LSI clock divided by 128"]
    Div128 = 1,
}
impl From<LSIPRE> for bool {
    #[inline(always)]
    fn from(variant: LSIPRE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIPRE` reader - LSI frequency prescaler"]
pub type LSIPRE_R = crate::BitReader<LSIPRE>;
impl LSIPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSIPRE {
        match self.bits {
            false => LSIPRE::Div1,
            true => LSIPRE::Div128,
        }
    }
    #[doc = "LSI clock not divided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == LSIPRE::Div1
    }
    #[doc = "LSI clock divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == LSIPRE::Div128
    }
}
#[doc = "Field `LSIPRE` writer - LSI frequency prescaler"]
pub type LSIPRE_W<'a, REG> = crate::BitWriter<'a, REG, LSIPRE>;
impl<'a, REG> LSIPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSI clock not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(LSIPRE::Div1)
    }
    #[doc = "LSI clock divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(LSIPRE::Div128)
    }
}
#[doc = "MSI clock ranges\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSISRANGE {
    #[doc = "4: Range 4 around 1 MHz"]
    F1mhz = 4,
    #[doc = "5: Range 5 around 2 MHz"]
    F2mhz = 5,
    #[doc = "6: Range 6 around 4 MHz (reset value)"]
    F4mhz = 6,
    #[doc = "7: Range 7 around 8 MHz"]
    F8mhz = 7,
}
impl From<MSISRANGE> for u8 {
    #[inline(always)]
    fn from(variant: MSISRANGE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MSISRANGE {
    type Ux = u8;
}
#[doc = "Field `MSISRANGE` reader - MSI clock ranges"]
pub type MSISRANGE_R = crate::FieldReader<MSISRANGE>;
impl MSISRANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MSISRANGE> {
        match self.bits {
            4 => Some(MSISRANGE::F1mhz),
            5 => Some(MSISRANGE::F2mhz),
            6 => Some(MSISRANGE::F4mhz),
            7 => Some(MSISRANGE::F8mhz),
            _ => None,
        }
    }
    #[doc = "Range 4 around 1 MHz"]
    #[inline(always)]
    pub fn is_f_1mhz(&self) -> bool {
        *self == MSISRANGE::F1mhz
    }
    #[doc = "Range 5 around 2 MHz"]
    #[inline(always)]
    pub fn is_f_2mhz(&self) -> bool {
        *self == MSISRANGE::F2mhz
    }
    #[doc = "Range 6 around 4 MHz (reset value)"]
    #[inline(always)]
    pub fn is_f_4mhz(&self) -> bool {
        *self == MSISRANGE::F4mhz
    }
    #[doc = "Range 7 around 8 MHz"]
    #[inline(always)]
    pub fn is_f_8mhz(&self) -> bool {
        *self == MSISRANGE::F8mhz
    }
}
#[doc = "Field `MSISRANGE` writer - MSI clock ranges"]
pub type MSISRANGE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MSISRANGE>;
impl<'a, REG> MSISRANGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Range 4 around 1 MHz"]
    #[inline(always)]
    pub fn f_1mhz(self) -> &'a mut crate::W<REG> {
        self.variant(MSISRANGE::F1mhz)
    }
    #[doc = "Range 5 around 2 MHz"]
    #[inline(always)]
    pub fn f_2mhz(self) -> &'a mut crate::W<REG> {
        self.variant(MSISRANGE::F2mhz)
    }
    #[doc = "Range 6 around 4 MHz (reset value)"]
    #[inline(always)]
    pub fn f_4mhz(self) -> &'a mut crate::W<REG> {
        self.variant(MSISRANGE::F4mhz)
    }
    #[doc = "Range 7 around 8 MHz"]
    #[inline(always)]
    pub fn f_8mhz(self) -> &'a mut crate::W<REG> {
        self.variant(MSISRANGE::F8mhz)
    }
}
#[doc = "Radio in reset status flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFRSTF {
    #[doc = "0: Sub-GHz radio out of reset"]
    NoReset = 0,
    #[doc = "1: Sub-GHz radio in reset"]
    Reset = 1,
}
impl From<RFRSTF> for bool {
    #[inline(always)]
    fn from(variant: RFRSTF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFRSTF` reader - Radio in reset status flag"]
pub type RFRSTF_R = crate::BitReader<RFRSTF>;
impl RFRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RFRSTF {
        match self.bits {
            false => RFRSTF::NoReset,
            true => RFRSTF::Reset,
        }
    }
    #[doc = "Sub-GHz radio out of reset"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == RFRSTF::NoReset
    }
    #[doc = "Sub-GHz radio in reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RFRSTF::Reset
    }
}
#[doc = "Radio reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFRST {
    #[doc = "0: Sub-GHz radio software reset removed"]
    Removed = 0,
    #[doc = "1: Sub-GHz radio software reset active"]
    Reset = 1,
}
impl From<RFRST> for bool {
    #[inline(always)]
    fn from(variant: RFRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFRST` reader - Radio reset"]
pub type RFRST_R = crate::BitReader<RFRST>;
impl RFRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RFRST {
        match self.bits {
            false => RFRST::Removed,
            true => RFRST::Reset,
        }
    }
    #[doc = "Sub-GHz radio software reset removed"]
    #[inline(always)]
    pub fn is_removed(&self) -> bool {
        *self == RFRST::Removed
    }
    #[doc = "Sub-GHz radio software reset active"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RFRST::Reset
    }
}
#[doc = "Field `RFRST` writer - Radio reset"]
pub type RFRST_W<'a, REG> = crate::BitWriter<'a, REG, RFRST>;
impl<'a, REG> RFRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sub-GHz radio software reset removed"]
    #[inline(always)]
    pub fn removed(self) -> &'a mut crate::W<REG> {
        self.variant(RFRST::Removed)
    }
    #[doc = "Sub-GHz radio software reset active"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(RFRST::Reset)
    }
}
#[doc = "Remove reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMVF {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset flags reset"]
    Clear = 1,
}
impl From<RMVF> for bool {
    #[inline(always)]
    fn from(variant: RMVF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RMVF` reader - Remove reset flag"]
pub type RMVF_R = crate::BitReader<RMVF>;
impl RMVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RMVF {
        match self.bits {
            false => RMVF::NoEffect,
            true => RMVF::Clear,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == RMVF::NoEffect
    }
    #[doc = "Reset flags reset"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RMVF::Clear
    }
}
#[doc = "Field `RMVF` writer - Remove reset flag"]
pub type RMVF_W<'a, REG> = crate::BitWriter<'a, REG, RMVF>;
impl<'a, REG> RMVF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(RMVF::NoEffect)
    }
    #[doc = "Reset flags reset"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RMVF::Clear)
    }
}
#[doc = "Radio illegal access flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFILARSTF {
    #[doc = "0: No SUBGHZ radio illegal command occurred"]
    NoIllegalCommand = 0,
    #[doc = "1: SUBGHZ radio illegal command occurred"]
    IllegalCommand = 1,
}
impl From<RFILARSTF> for bool {
    #[inline(always)]
    fn from(variant: RFILARSTF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFILARSTF` reader - Radio illegal access flag"]
pub type RFILARSTF_R = crate::BitReader<RFILARSTF>;
impl RFILARSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RFILARSTF {
        match self.bits {
            false => RFILARSTF::NoIllegalCommand,
            true => RFILARSTF::IllegalCommand,
        }
    }
    #[doc = "No SUBGHZ radio illegal command occurred"]
    #[inline(always)]
    pub fn is_no_illegal_command(&self) -> bool {
        *self == RFILARSTF::NoIllegalCommand
    }
    #[doc = "SUBGHZ radio illegal command occurred"]
    #[inline(always)]
    pub fn is_illegal_command(&self) -> bool {
        *self == RFILARSTF::IllegalCommand
    }
}
#[doc = "Option byte loader reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBLRSTF {
    #[doc = "0: No reset occurred"]
    NoReset = 0,
    #[doc = "1: Reset occurred"]
    Reset = 1,
}
impl From<OBLRSTF> for bool {
    #[inline(always)]
    fn from(variant: OBLRSTF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OBLRSTF` reader - Option byte loader reset flag"]
pub type OBLRSTF_R = crate::BitReader<OBLRSTF>;
impl OBLRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OBLRSTF {
        match self.bits {
            false => OBLRSTF::NoReset,
            true => OBLRSTF::Reset,
        }
    }
    #[doc = "No reset occurred"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == OBLRSTF::NoReset
    }
    #[doc = "Reset occurred"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == OBLRSTF::Reset
    }
}
#[doc = "Pin reset flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINRSTF {
    #[doc = "0: No reset occurred"]
    NoReset = 0,
    #[doc = "1: Reset occurred"]
    Reset = 1,
}
impl From<PINRSTF> for bool {
    #[inline(always)]
    fn from(variant: PINRSTF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINRSTF` reader - Pin reset flag"]
pub type PINRSTF_R = crate::BitReader<PINRSTF>;
impl PINRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PINRSTF {
        match self.bits {
            false => PINRSTF::NoReset,
            true => PINRSTF::Reset,
        }
    }
    #[doc = "No reset occurred"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == PINRSTF::NoReset
    }
    #[doc = "Reset occurred"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PINRSTF::Reset
    }
}
#[doc = "BOR flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BORRSTF {
    #[doc = "0: No reset occurred"]
    NoReset = 0,
    #[doc = "1: Reset occurred"]
    Reset = 1,
}
impl From<BORRSTF> for bool {
    #[inline(always)]
    fn from(variant: BORRSTF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BORRSTF` reader - BOR flag"]
pub type BORRSTF_R = crate::BitReader<BORRSTF>;
impl BORRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BORRSTF {
        match self.bits {
            false => BORRSTF::NoReset,
            true => BORRSTF::Reset,
        }
    }
    #[doc = "No reset occurred"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == BORRSTF::NoReset
    }
    #[doc = "Reset occurred"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == BORRSTF::Reset
    }
}
#[doc = "Software reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFTRSTF {
    #[doc = "0: No reset occurred"]
    NoReset = 0,
    #[doc = "1: Reset occurred"]
    Reset = 1,
}
impl From<SFTRSTF> for bool {
    #[inline(always)]
    fn from(variant: SFTRSTF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SFTRSTF` reader - Software reset flag"]
pub type SFTRSTF_R = crate::BitReader<SFTRSTF>;
impl SFTRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SFTRSTF {
        match self.bits {
            false => SFTRSTF::NoReset,
            true => SFTRSTF::Reset,
        }
    }
    #[doc = "No reset occurred"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == SFTRSTF::NoReset
    }
    #[doc = "Reset occurred"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SFTRSTF::Reset
    }
}
#[doc = "Independent window watchdog reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDGRSTF {
    #[doc = "0: No reset occurred"]
    NoReset = 0,
    #[doc = "1: Reset occurred"]
    Reset = 1,
}
impl From<IWDGRSTF> for bool {
    #[inline(always)]
    fn from(variant: IWDGRSTF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWDGRSTF` reader - Independent window watchdog reset flag"]
pub type IWDGRSTF_R = crate::BitReader<IWDGRSTF>;
impl IWDGRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IWDGRSTF {
        match self.bits {
            false => IWDGRSTF::NoReset,
            true => IWDGRSTF::Reset,
        }
    }
    #[doc = "No reset occurred"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == IWDGRSTF::NoReset
    }
    #[doc = "Reset occurred"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == IWDGRSTF::Reset
    }
}
#[doc = "Window watchdog reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDGRSTF {
    #[doc = "0: No reset occurred"]
    NoReset = 0,
    #[doc = "1: Reset occurred"]
    Reset = 1,
}
impl From<WWDGRSTF> for bool {
    #[inline(always)]
    fn from(variant: WWDGRSTF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDGRSTF` reader - Window watchdog reset flag"]
pub type WWDGRSTF_R = crate::BitReader<WWDGRSTF>;
impl WWDGRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WWDGRSTF {
        match self.bits {
            false => WWDGRSTF::NoReset,
            true => WWDGRSTF::Reset,
        }
    }
    #[doc = "No reset occurred"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == WWDGRSTF::NoReset
    }
    #[doc = "Reset occurred"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == WWDGRSTF::Reset
    }
}
#[doc = "Low-power reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPWRRSTF {
    #[doc = "0: No reset occurred"]
    NoReset = 0,
    #[doc = "1: Reset occurred"]
    Reset = 1,
}
impl From<LPWRRSTF> for bool {
    #[inline(always)]
    fn from(variant: LPWRRSTF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPWRRSTF` reader - Low-power reset flag"]
pub type LPWRRSTF_R = crate::BitReader<LPWRRSTF>;
impl LPWRRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPWRRSTF {
        match self.bits {
            false => LPWRRSTF::NoReset,
            true => LPWRRSTF::Reset,
        }
    }
    #[doc = "No reset occurred"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == LPWRRSTF::NoReset
    }
    #[doc = "Reset occurred"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == LPWRRSTF::Reset
    }
}
impl R {
    #[doc = "Bit 0 - LSI oscillator enable"]
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSI oscillator ready"]
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - LSI frequency prescaler"]
    #[inline(always)]
    pub fn lsipre(&self) -> LSIPRE_R {
        LSIPRE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - MSI clock ranges"]
    #[inline(always)]
    pub fn msisrange(&self) -> MSISRANGE_R {
        MSISRANGE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - Radio in reset status flag"]
    #[inline(always)]
    pub fn rfrstf(&self) -> RFRSTF_R {
        RFRSTF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Radio reset"]
    #[inline(always)]
    pub fn rfrst(&self) -> RFRST_R {
        RFRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 23 - Remove reset flag"]
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Radio illegal access flag"]
    #[inline(always)]
    pub fn rfilarstf(&self) -> RFILARSTF_R {
        RFILARSTF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Option byte loader reset flag"]
    #[inline(always)]
    pub fn oblrstf(&self) -> OBLRSTF_R {
        OBLRSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Pin reset flag"]
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - BOR flag"]
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Independent window watchdog reset flag"]
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Window watchdog reset flag"]
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI oscillator enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsion(&mut self) -> LSION_W<CSRrs> {
        LSION_W::new(self, 0)
    }
    #[doc = "Bit 4 - LSI frequency prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn lsipre(&mut self) -> LSIPRE_W<CSRrs> {
        LSIPRE_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - MSI clock ranges"]
    #[inline(always)]
    #[must_use]
    pub fn msisrange(&mut self) -> MSISRANGE_W<CSRrs> {
        MSISRANGE_W::new(self, 8)
    }
    #[doc = "Bit 15 - Radio reset"]
    #[inline(always)]
    #[must_use]
    pub fn rfrst(&mut self) -> RFRST_W<CSRrs> {
        RFRST_W::new(self, 15)
    }
    #[doc = "Bit 23 - Remove reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn rmvf(&mut self) -> RMVF_W<CSRrs> {
        RMVF_W::new(self, 23)
    }
}
#[doc = "Control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CSRrs {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR to value 0x0c01_c600"]
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0x0c01_c600;
}
