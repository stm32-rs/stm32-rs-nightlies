///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Register `CSR` writer
pub type W = crate::W<CSRrs>;
/**LSI oscillator enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSION {
    ///0: LSI oscillator OFF
    Disabled = 0,
    ///1: LSI oscillator ON
    Enabled = 1,
}
impl From<LSION> for bool {
    #[inline(always)]
    fn from(variant: LSION) -> Self {
        variant as u8 != 0
    }
}
///Field `LSION` reader - LSI oscillator enable
pub type LSION_R = crate::BitReader<LSION>;
impl LSION_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSION {
        match self.bits {
            false => LSION::Disabled,
            true => LSION::Enabled,
        }
    }
    ///LSI oscillator OFF
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSION::Disabled
    }
    ///LSI oscillator ON
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSION::Enabled
    }
}
///Field `LSION` writer - LSI oscillator enable
pub type LSION_W<'a, REG> = crate::BitWriter<'a, REG, LSION>;
impl<'a, REG> LSION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LSI oscillator OFF
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSION::Disabled)
    }
    ///LSI oscillator ON
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSION::Enabled)
    }
}
/**LSI oscillator ready

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDY {
    ///0: LSI oscillator not ready
    NotReady = 0,
    ///1: LSI oscillator ready
    Ready = 1,
}
impl From<LSIRDY> for bool {
    #[inline(always)]
    fn from(variant: LSIRDY) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIRDY` reader - LSI oscillator ready
pub type LSIRDY_R = crate::BitReader<LSIRDY>;
impl LSIRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSIRDY {
        match self.bits {
            false => LSIRDY::NotReady,
            true => LSIRDY::Ready,
        }
    }
    ///LSI oscillator not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LSIRDY::NotReady
    }
    ///LSI oscillator ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LSIRDY::Ready
    }
}
/**Internal low-speed oscillator predivided by 128

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIPREDIV {
    ///0: LSI PREDIV OFF
    Disabled = 0,
    ///1: LSI PREDIV ON
    Enabled = 1,
}
impl From<LSIPREDIV> for bool {
    #[inline(always)]
    fn from(variant: LSIPREDIV) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIPREDIV` reader - Internal low-speed oscillator predivided by 128
pub type LSIPREDIV_R = crate::BitReader<LSIPREDIV>;
impl LSIPREDIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSIPREDIV {
        match self.bits {
            false => LSIPREDIV::Disabled,
            true => LSIPREDIV::Enabled,
        }
    }
    ///LSI PREDIV OFF
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSIPREDIV::Disabled
    }
    ///LSI PREDIV ON
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSIPREDIV::Enabled
    }
}
///Field `LSIPREDIV` writer - Internal low-speed oscillator predivided by 128
pub type LSIPREDIV_W<'a, REG> = crate::BitWriter<'a, REG, LSIPREDIV>;
impl<'a, REG> LSIPREDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LSI PREDIV OFF
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSIPREDIV::Disabled)
    }
    ///LSI PREDIV ON
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSIPREDIV::Enabled)
    }
}
/**SI range after Standby mode

Value on reset: 6*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSISRANGE {
    ///4: range 4 around 1 MHz
    Range1m = 4,
    ///5: range 5 around 2 MHz
    Range2m = 5,
    ///6: range 6 around 4 MHz
    Range4m = 6,
    ///7: range 7 around 8 MHz
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
impl crate::IsEnum for MSISRANGE {}
///Field `MSISRANGE` reader - SI range after Standby mode
pub type MSISRANGE_R = crate::FieldReader<MSISRANGE>;
impl MSISRANGE_R {
    ///Get enumerated values variant
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
    ///range 4 around 1 MHz
    #[inline(always)]
    pub fn is_range1m(&self) -> bool {
        *self == MSISRANGE::Range1m
    }
    ///range 5 around 2 MHz
    #[inline(always)]
    pub fn is_range2m(&self) -> bool {
        *self == MSISRANGE::Range2m
    }
    ///range 6 around 4 MHz
    #[inline(always)]
    pub fn is_range4m(&self) -> bool {
        *self == MSISRANGE::Range4m
    }
    ///range 7 around 8 MHz
    #[inline(always)]
    pub fn is_range8m(&self) -> bool {
        *self == MSISRANGE::Range8m
    }
}
///Field `MSISRANGE` writer - SI range after Standby mode
pub type MSISRANGE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MSISRANGE>;
impl<'a, REG> MSISRANGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///range 4 around 1 MHz
    #[inline(always)]
    pub fn range1m(self) -> &'a mut crate::W<REG> {
        self.variant(MSISRANGE::Range1m)
    }
    ///range 5 around 2 MHz
    #[inline(always)]
    pub fn range2m(self) -> &'a mut crate::W<REG> {
        self.variant(MSISRANGE::Range2m)
    }
    ///range 6 around 4 MHz
    #[inline(always)]
    pub fn range4m(self) -> &'a mut crate::W<REG> {
        self.variant(MSISRANGE::Range4m)
    }
    ///range 7 around 8 MHz
    #[inline(always)]
    pub fn range8m(self) -> &'a mut crate::W<REG> {
        self.variant(MSISRANGE::Range8m)
    }
}
/**Remove reset flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMVF {
    ///0: No effect
    NoEffect = 0,
    ///1: Clear the reset flags
    Clear = 1,
}
impl From<RMVF> for bool {
    #[inline(always)]
    fn from(variant: RMVF) -> Self {
        variant as u8 != 0
    }
}
///Field `RMVF` reader - Remove reset flag
pub type RMVF_R = crate::BitReader<RMVF>;
impl RMVF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RMVF {
        match self.bits {
            false => RMVF::NoEffect,
            true => RMVF::Clear,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == RMVF::NoEffect
    }
    ///Clear the reset flags
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RMVF::Clear
    }
}
///Field `RMVF` writer - Remove reset flag
pub type RMVF_W<'a, REG> = crate::BitWriter<'a, REG, RMVF>;
impl<'a, REG> RMVF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(RMVF::NoEffect)
    }
    ///Clear the reset flags
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RMVF::Clear)
    }
}
/**Firewall reset flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FWRSTF {
    ///0: No reset from the firewall occurred
    NotOccured = 0,
    ///1: Reset from the firewall occurred
    Occured = 1,
}
impl From<FWRSTF> for bool {
    #[inline(always)]
    fn from(variant: FWRSTF) -> Self {
        variant as u8 != 0
    }
}
///Field `FWRSTF` reader - Firewall reset flag
pub type FWRSTF_R = crate::BitReader<FWRSTF>;
impl FWRSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FWRSTF {
        match self.bits {
            false => FWRSTF::NotOccured,
            true => FWRSTF::Occured,
        }
    }
    ///No reset from the firewall occurred
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        *self == FWRSTF::NotOccured
    }
    ///Reset from the firewall occurred
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        *self == FWRSTF::Occured
    }
}
/**Option byte loader reset flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBLRSTF {
    ///0: No reset from Option Byte loading occurred
    NotOccured = 0,
    ///1: Reset from Option Byte loading occurred
    Occured = 1,
}
impl From<OBLRSTF> for bool {
    #[inline(always)]
    fn from(variant: OBLRSTF) -> Self {
        variant as u8 != 0
    }
}
///Field `OBLRSTF` reader - Option byte loader reset flag
pub type OBLRSTF_R = crate::BitReader<OBLRSTF>;
impl OBLRSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OBLRSTF {
        match self.bits {
            false => OBLRSTF::NotOccured,
            true => OBLRSTF::Occured,
        }
    }
    ///No reset from Option Byte loading occurred
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        *self == OBLRSTF::NotOccured
    }
    ///Reset from Option Byte loading occurred
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        *self == OBLRSTF::Occured
    }
}
/**Pin reset flag

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINRSTF {
    ///0: No reset from NRST pin occurred
    NotOccured = 0,
    ///1: Reset from NRST pin occurred
    Occured = 1,
}
impl From<PINRSTF> for bool {
    #[inline(always)]
    fn from(variant: PINRSTF) -> Self {
        variant as u8 != 0
    }
}
///Field `PINRSTF` reader - Pin reset flag
pub type PINRSTF_R = crate::BitReader<PINRSTF>;
impl PINRSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PINRSTF {
        match self.bits {
            false => PINRSTF::NotOccured,
            true => PINRSTF::Occured,
        }
    }
    ///No reset from NRST pin occurred
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        *self == PINRSTF::NotOccured
    }
    ///Reset from NRST pin occurred
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        *self == PINRSTF::Occured
    }
}
/**BOR flag

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BORRSTF {
    ///0: No BOR occurred
    NotOccured = 0,
    ///1: BOR occurred
    Occured = 1,
}
impl From<BORRSTF> for bool {
    #[inline(always)]
    fn from(variant: BORRSTF) -> Self {
        variant as u8 != 0
    }
}
///Field `BORRSTF` reader - BOR flag
pub type BORRSTF_R = crate::BitReader<BORRSTF>;
impl BORRSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BORRSTF {
        match self.bits {
            false => BORRSTF::NotOccured,
            true => BORRSTF::Occured,
        }
    }
    ///No BOR occurred
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        *self == BORRSTF::NotOccured
    }
    ///BOR occurred
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        *self == BORRSTF::Occured
    }
}
/**Software reset flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFTRSTF {
    ///0: No software reset occurred
    NotOccured = 0,
    ///1: Software reset occurred
    Occured = 1,
}
impl From<SFTRSTF> for bool {
    #[inline(always)]
    fn from(variant: SFTRSTF) -> Self {
        variant as u8 != 0
    }
}
///Field `SFTRSTF` reader - Software reset flag
pub type SFTRSTF_R = crate::BitReader<SFTRSTF>;
impl SFTRSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SFTRSTF {
        match self.bits {
            false => SFTRSTF::NotOccured,
            true => SFTRSTF::Occured,
        }
    }
    ///No software reset occurred
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        *self == SFTRSTF::NotOccured
    }
    ///Software reset occurred
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        *self == SFTRSTF::Occured
    }
}
/**Independent window watchdog reset flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDGRSTF {
    ///0: No independent watchdog reset occurred
    NotOccured = 0,
    ///1: Independent watchdog reset occurred
    Occured = 1,
}
impl From<IWDGRSTF> for bool {
    #[inline(always)]
    fn from(variant: IWDGRSTF) -> Self {
        variant as u8 != 0
    }
}
///Field `IWDGRSTF` reader - Independent window watchdog reset flag
pub type IWDGRSTF_R = crate::BitReader<IWDGRSTF>;
impl IWDGRSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IWDGRSTF {
        match self.bits {
            false => IWDGRSTF::NotOccured,
            true => IWDGRSTF::Occured,
        }
    }
    ///No independent watchdog reset occurred
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        *self == IWDGRSTF::NotOccured
    }
    ///Independent watchdog reset occurred
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        *self == IWDGRSTF::Occured
    }
}
/**Window watchdog reset flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDGRSTF {
    ///0: No window watchdog reset occurred
    NotOccured = 0,
    ///1: Window watchdog reset occurred
    Occured = 1,
}
impl From<WWDGRSTF> for bool {
    #[inline(always)]
    fn from(variant: WWDGRSTF) -> Self {
        variant as u8 != 0
    }
}
///Field `WWDGRSTF` reader - Window watchdog reset flag
pub type WWDGRSTF_R = crate::BitReader<WWDGRSTF>;
impl WWDGRSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WWDGRSTF {
        match self.bits {
            false => WWDGRSTF::NotOccured,
            true => WWDGRSTF::Occured,
        }
    }
    ///No window watchdog reset occurred
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        *self == WWDGRSTF::NotOccured
    }
    ///Window watchdog reset occurred
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        *self == WWDGRSTF::Occured
    }
}
/**Low-power reset flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPWRRSTF {
    ///0: No illegal mode reset occurred
    NotOccured = 0,
    ///1: Illegal mode reset occurred
    Occured = 1,
}
impl From<LPWRRSTF> for bool {
    #[inline(always)]
    fn from(variant: LPWRRSTF) -> Self {
        variant as u8 != 0
    }
}
///Field `LPWRRSTF` reader - Low-power reset flag
pub type LPWRRSTF_R = crate::BitReader<LPWRRSTF>;
impl LPWRRSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPWRRSTF {
        match self.bits {
            false => LPWRRSTF::NotOccured,
            true => LPWRRSTF::Occured,
        }
    }
    ///No illegal mode reset occurred
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        *self == LPWRRSTF::NotOccured
    }
    ///Illegal mode reset occurred
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        *self == LPWRRSTF::Occured
    }
}
impl R {
    ///Bit 0 - LSI oscillator enable
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSI oscillator ready
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - Internal low-speed oscillator predivided by 128
    #[inline(always)]
    pub fn lsiprediv(&self) -> LSIPREDIV_R {
        LSIPREDIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:11 - SI range after Standby mode
    #[inline(always)]
    pub fn msisrange(&self) -> MSISRANGE_R {
        MSISRANGE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 23 - Remove reset flag
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Firewall reset flag
    #[inline(always)]
    pub fn fwrstf(&self) -> FWRSTF_R {
        FWRSTF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Option byte loader reset flag
    #[inline(always)]
    pub fn oblrstf(&self) -> OBLRSTF_R {
        OBLRSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Pin reset flag
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - BOR flag
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Software reset flag
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Independent window watchdog reset flag
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Window watchdog reset flag
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Low-power reset flag
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("lpwrrstf", &self.lpwrrstf())
            .field("wwdgrstf", &self.wwdgrstf())
            .field("iwdgrstf", &self.iwdgrstf())
            .field("sftrstf", &self.sftrstf())
            .field("borrstf", &self.borrstf())
            .field("pinrstf", &self.pinrstf())
            .field("oblrstf", &self.oblrstf())
            .field("fwrstf", &self.fwrstf())
            .field("rmvf", &self.rmvf())
            .field("msisrange", &self.msisrange())
            .field("lsirdy", &self.lsirdy())
            .field("lsion", &self.lsion())
            .field("lsiprediv", &self.lsiprediv())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSI oscillator enable
    #[inline(always)]
    pub fn lsion(&mut self) -> LSION_W<'_, CSRrs> {
        LSION_W::new(self, 0)
    }
    ///Bit 4 - Internal low-speed oscillator predivided by 128
    #[inline(always)]
    pub fn lsiprediv(&mut self) -> LSIPREDIV_W<'_, CSRrs> {
        LSIPREDIV_W::new(self, 4)
    }
    ///Bits 8:11 - SI range after Standby mode
    #[inline(always)]
    pub fn msisrange(&mut self) -> MSISRANGE_W<'_, CSRrs> {
        MSISRANGE_W::new(self, 8)
    }
    ///Bit 23 - Remove reset flag
    #[inline(always)]
    pub fn rmvf(&mut self) -> RMVF_W<'_, CSRrs> {
        RMVF_W::new(self, 23)
    }
}
/**CSR

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#RCC:CSR)*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`write(|w| ..)` method takes [`csr::W`](W) writer structure
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR to value 0x0c00_0600
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0x0c00_0600;
}
