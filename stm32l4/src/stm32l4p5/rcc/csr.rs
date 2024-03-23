#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSRrs>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CSRrs>;
#[doc = "LSI oscillator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSION {
    #[doc = "0: LSI oscillator OFF"]
    Disabled = 0,
    #[doc = "1: LSI oscillator ON"]
    Enabled = 1,
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
            false => LSION::Disabled,
            true => LSION::Enabled,
        }
    }
    #[doc = "LSI oscillator OFF"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSION::Disabled
    }
    #[doc = "LSI oscillator ON"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSION::Enabled
    }
}
#[doc = "Field `LSION` writer - LSI oscillator enable"]
pub type LSION_W<'a, REG> = crate::BitWriter<'a, REG, LSION>;
impl<'a, REG> LSION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSI oscillator OFF"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSION::Disabled)
    }
    #[doc = "LSI oscillator ON"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSION::Enabled)
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
#[doc = "Internal low-speed oscillator predivided by 128\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIPREDIV {
    #[doc = "0: LSI PREDIV OFF"]
    Disabled = 0,
    #[doc = "1: LSI PREDIV ON"]
    Enabled = 1,
}
impl From<LSIPREDIV> for bool {
    #[inline(always)]
    fn from(variant: LSIPREDIV) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIPREDIV` reader - Internal low-speed oscillator predivided by 128"]
pub type LSIPREDIV_R = crate::BitReader<LSIPREDIV>;
impl LSIPREDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSIPREDIV {
        match self.bits {
            false => LSIPREDIV::Disabled,
            true => LSIPREDIV::Enabled,
        }
    }
    #[doc = "LSI PREDIV OFF"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSIPREDIV::Disabled
    }
    #[doc = "LSI PREDIV ON"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSIPREDIV::Enabled
    }
}
#[doc = "Field `LSIPREDIV` writer - Internal low-speed oscillator predivided by 128"]
pub type LSIPREDIV_W<'a, REG> = crate::BitWriter<'a, REG, LSIPREDIV>;
impl<'a, REG> LSIPREDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSI PREDIV OFF"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSIPREDIV::Disabled)
    }
    #[doc = "LSI PREDIV ON"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSIPREDIV::Enabled)
    }
}
#[doc = "SI range after Standby mode\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSISRANGE {
    #[doc = "4: range 4 around 1 MHz"]
    Range1m = 4,
    #[doc = "5: range 5 around 2 MHz"]
    Range2m = 5,
    #[doc = "6: range 6 around 4 MHz"]
    Range4m = 6,
    #[doc = "7: range 7 around 8 MHz"]
    Range8m = 7,
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
#[doc = "Field `MSISRANGE` reader - SI range after Standby mode"]
pub type MSISRANGE_R = crate::FieldReader<MSISRANGE>;
impl MSISRANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MSISRANGE> {
        match self.bits {
            4 => Some(MSISRANGE::Range1m),
            5 => Some(MSISRANGE::Range2m),
            6 => Some(MSISRANGE::Range4m),
            7 => Some(MSISRANGE::Range8m),
            _ => None,
        }
    }
    #[doc = "range 4 around 1 MHz"]
    #[inline(always)]
    pub fn is_range1m(&self) -> bool {
        *self == MSISRANGE::Range1m
    }
    #[doc = "range 5 around 2 MHz"]
    #[inline(always)]
    pub fn is_range2m(&self) -> bool {
        *self == MSISRANGE::Range2m
    }
    #[doc = "range 6 around 4 MHz"]
    #[inline(always)]
    pub fn is_range4m(&self) -> bool {
        *self == MSISRANGE::Range4m
    }
    #[doc = "range 7 around 8 MHz"]
    #[inline(always)]
    pub fn is_range8m(&self) -> bool {
        *self == MSISRANGE::Range8m
    }
}
#[doc = "Field `MSISRANGE` writer - SI range after Standby mode"]
pub type MSISRANGE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MSISRANGE>;
impl<'a, REG> MSISRANGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "range 4 around 1 MHz"]
    #[inline(always)]
    pub fn range1m(self) -> &'a mut crate::W<REG> {
        self.variant(MSISRANGE::Range1m)
    }
    #[doc = "range 5 around 2 MHz"]
    #[inline(always)]
    pub fn range2m(self) -> &'a mut crate::W<REG> {
        self.variant(MSISRANGE::Range2m)
    }
    #[doc = "range 6 around 4 MHz"]
    #[inline(always)]
    pub fn range4m(self) -> &'a mut crate::W<REG> {
        self.variant(MSISRANGE::Range4m)
    }
    #[doc = "range 7 around 8 MHz"]
    #[inline(always)]
    pub fn range8m(self) -> &'a mut crate::W<REG> {
        self.variant(MSISRANGE::Range8m)
    }
}
#[doc = "Remove reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMVF {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clear the reset flags"]
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
    #[doc = "Clear the reset flags"]
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
    #[doc = "Clear the reset flags"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RMVF::Clear)
    }
}
#[doc = "Firewall reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FWRSTF {
    #[doc = "0: No reset from the firewall occurred"]
    NotOccured = 0,
    #[doc = "1: Reset from the firewall occurred"]
    Occured = 1,
}
impl From<FWRSTF> for bool {
    #[inline(always)]
    fn from(variant: FWRSTF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FWRSTF` reader - Firewall reset flag"]
pub type FWRSTF_R = crate::BitReader<FWRSTF>;
impl FWRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FWRSTF {
        match self.bits {
            false => FWRSTF::NotOccured,
            true => FWRSTF::Occured,
        }
    }
    #[doc = "No reset from the firewall occurred"]
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        *self == FWRSTF::NotOccured
    }
    #[doc = "Reset from the firewall occurred"]
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        *self == FWRSTF::Occured
    }
}
#[doc = "Option byte loader reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBLRSTF {
    #[doc = "0: No reset from Option Byte loading occurred"]
    NotOccured = 0,
    #[doc = "1: Reset from Option Byte loading occurred"]
    Occured = 1,
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
            false => OBLRSTF::NotOccured,
            true => OBLRSTF::Occured,
        }
    }
    #[doc = "No reset from Option Byte loading occurred"]
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        *self == OBLRSTF::NotOccured
    }
    #[doc = "Reset from Option Byte loading occurred"]
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        *self == OBLRSTF::Occured
    }
}
#[doc = "Pin reset flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINRSTF {
    #[doc = "0: No reset from NRST pin occurred"]
    NotOccured = 0,
    #[doc = "1: Reset from NRST pin occurred"]
    Occured = 1,
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
            false => PINRSTF::NotOccured,
            true => PINRSTF::Occured,
        }
    }
    #[doc = "No reset from NRST pin occurred"]
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        *self == PINRSTF::NotOccured
    }
    #[doc = "Reset from NRST pin occurred"]
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        *self == PINRSTF::Occured
    }
}
#[doc = "BOR flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BORRSTF {
    #[doc = "0: No BOR occurred"]
    NotOccured = 0,
    #[doc = "1: BOR occurred"]
    Occured = 1,
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
            false => BORRSTF::NotOccured,
            true => BORRSTF::Occured,
        }
    }
    #[doc = "No BOR occurred"]
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        *self == BORRSTF::NotOccured
    }
    #[doc = "BOR occurred"]
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        *self == BORRSTF::Occured
    }
}
#[doc = "Software reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFTRSTF {
    #[doc = "0: No software reset occurred"]
    NotOccured = 0,
    #[doc = "1: Software reset occurred"]
    Occured = 1,
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
            false => SFTRSTF::NotOccured,
            true => SFTRSTF::Occured,
        }
    }
    #[doc = "No software reset occurred"]
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        *self == SFTRSTF::NotOccured
    }
    #[doc = "Software reset occurred"]
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        *self == SFTRSTF::Occured
    }
}
#[doc = "Independent window watchdog reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDGRSTF {
    #[doc = "0: No independent watchdog reset occurred"]
    NotOccured = 0,
    #[doc = "1: Independent watchdog reset occurred"]
    Occured = 1,
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
            false => IWDGRSTF::NotOccured,
            true => IWDGRSTF::Occured,
        }
    }
    #[doc = "No independent watchdog reset occurred"]
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        *self == IWDGRSTF::NotOccured
    }
    #[doc = "Independent watchdog reset occurred"]
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        *self == IWDGRSTF::Occured
    }
}
#[doc = "Window watchdog reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDGRSTF {
    #[doc = "0: No window watchdog reset occurred"]
    NotOccured = 0,
    #[doc = "1: Window watchdog reset occurred"]
    Occured = 1,
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
            false => WWDGRSTF::NotOccured,
            true => WWDGRSTF::Occured,
        }
    }
    #[doc = "No window watchdog reset occurred"]
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        *self == WWDGRSTF::NotOccured
    }
    #[doc = "Window watchdog reset occurred"]
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        *self == WWDGRSTF::Occured
    }
}
#[doc = "Low-power reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPWRRSTF {
    #[doc = "0: No illegal mode reset occurred"]
    NotOccured = 0,
    #[doc = "1: Illegal mode reset occurred"]
    Occured = 1,
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
            false => LPWRRSTF::NotOccured,
            true => LPWRRSTF::Occured,
        }
    }
    #[doc = "No illegal mode reset occurred"]
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        *self == LPWRRSTF::NotOccured
    }
    #[doc = "Illegal mode reset occurred"]
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        *self == LPWRRSTF::Occured
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
    #[doc = "Bit 4 - Internal low-speed oscillator predivided by 128"]
    #[inline(always)]
    pub fn lsiprediv(&self) -> LSIPREDIV_R {
        LSIPREDIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - SI range after Standby mode"]
    #[inline(always)]
    pub fn msisrange(&self) -> MSISRANGE_R {
        MSISRANGE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - Remove reset flag"]
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Firewall reset flag"]
    #[inline(always)]
    pub fn fwrstf(&self) -> FWRSTF_R {
        FWRSTF_R::new(((self.bits >> 24) & 1) != 0)
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
    #[doc = "Bit 4 - Internal low-speed oscillator predivided by 128"]
    #[inline(always)]
    #[must_use]
    pub fn lsiprediv(&mut self) -> LSIPREDIV_W<CSRrs> {
        LSIPREDIV_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - SI range after Standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn msisrange(&mut self) -> MSISRANGE_W<CSRrs> {
        MSISRANGE_W::new(self, 8)
    }
    #[doc = "Bit 23 - Remove reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn rmvf(&mut self) -> RMVF_W<CSRrs> {
        RMVF_W::new(self, 23)
    }
}
#[doc = "CSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CSR to value 0x0c00_0600"]
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0x0c00_0600;
}
