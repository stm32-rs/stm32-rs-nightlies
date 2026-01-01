///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Register `CSR` writer
pub type W = crate::W<CSRrs>;
/**LSI oscillator enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSION {
    ///0: LSI oscillator off
    Off = 0,
    ///1: LSI oscillator on
    On = 1,
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
            false => LSION::Off,
            true => LSION::On,
        }
    }
    ///LSI oscillator off
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LSION::Off
    }
    ///LSI oscillator on
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == LSION::On
    }
}
///Field `LSION` writer - LSI oscillator enable
pub type LSION_W<'a, REG> = crate::BitWriter<'a, REG, LSION>;
impl<'a, REG> LSION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LSI oscillator off
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(LSION::Off)
    }
    ///LSI oscillator on
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(LSION::On)
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
/**LSI frequency prescaler

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIPRE {
    ///0: LSI clock not divided
    Div1 = 0,
    ///1: LSI clock divided by 128
    Div128 = 1,
}
impl From<LSIPRE> for bool {
    #[inline(always)]
    fn from(variant: LSIPRE) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIPRE` reader - LSI frequency prescaler
pub type LSIPRE_R = crate::BitReader<LSIPRE>;
impl LSIPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSIPRE {
        match self.bits {
            false => LSIPRE::Div1,
            true => LSIPRE::Div128,
        }
    }
    ///LSI clock not divided
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == LSIPRE::Div1
    }
    ///LSI clock divided by 128
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == LSIPRE::Div128
    }
}
///Field `LSIPRE` writer - LSI frequency prescaler
pub type LSIPRE_W<'a, REG> = crate::BitWriter<'a, REG, LSIPRE>;
impl<'a, REG> LSIPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LSI clock not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(LSIPRE::Div1)
    }
    ///LSI clock divided by 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(LSIPRE::Div128)
    }
}
/**MSI clock ranges

Value on reset: 6*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSISRANGE {
    ///4: Range 4 around 1 MHz
    F1mhz = 4,
    ///5: Range 5 around 2 MHz
    F2mhz = 5,
    ///6: Range 6 around 4 MHz (reset value)
    F4mhz = 6,
    ///7: Range 7 around 8 MHz
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
impl crate::IsEnum for MSISRANGE {}
///Field `MSISRANGE` reader - MSI clock ranges
pub type MSISRANGE_R = crate::FieldReader<MSISRANGE>;
impl MSISRANGE_R {
    ///Get enumerated values variant
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
    ///Range 4 around 1 MHz
    #[inline(always)]
    pub fn is_f_1mhz(&self) -> bool {
        *self == MSISRANGE::F1mhz
    }
    ///Range 5 around 2 MHz
    #[inline(always)]
    pub fn is_f_2mhz(&self) -> bool {
        *self == MSISRANGE::F2mhz
    }
    ///Range 6 around 4 MHz (reset value)
    #[inline(always)]
    pub fn is_f_4mhz(&self) -> bool {
        *self == MSISRANGE::F4mhz
    }
    ///Range 7 around 8 MHz
    #[inline(always)]
    pub fn is_f_8mhz(&self) -> bool {
        *self == MSISRANGE::F8mhz
    }
}
///Field `MSISRANGE` writer - MSI clock ranges
pub type MSISRANGE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MSISRANGE>;
impl<'a, REG> MSISRANGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Range 4 around 1 MHz
    #[inline(always)]
    pub fn f_1mhz(self) -> &'a mut crate::W<REG> {
        self.variant(MSISRANGE::F1mhz)
    }
    ///Range 5 around 2 MHz
    #[inline(always)]
    pub fn f_2mhz(self) -> &'a mut crate::W<REG> {
        self.variant(MSISRANGE::F2mhz)
    }
    ///Range 6 around 4 MHz (reset value)
    #[inline(always)]
    pub fn f_4mhz(self) -> &'a mut crate::W<REG> {
        self.variant(MSISRANGE::F4mhz)
    }
    ///Range 7 around 8 MHz
    #[inline(always)]
    pub fn f_8mhz(self) -> &'a mut crate::W<REG> {
        self.variant(MSISRANGE::F8mhz)
    }
}
/**Radio in reset status flag

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFRSTF {
    ///0: Sub-GHz radio out of reset
    NoReset = 0,
    ///1: Sub-GHz radio in reset
    Reset = 1,
}
impl From<RFRSTF> for bool {
    #[inline(always)]
    fn from(variant: RFRSTF) -> Self {
        variant as u8 != 0
    }
}
///Field `RFRSTF` reader - Radio in reset status flag
pub type RFRSTF_R = crate::BitReader<RFRSTF>;
impl RFRSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RFRSTF {
        match self.bits {
            false => RFRSTF::NoReset,
            true => RFRSTF::Reset,
        }
    }
    ///Sub-GHz radio out of reset
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == RFRSTF::NoReset
    }
    ///Sub-GHz radio in reset
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RFRSTF::Reset
    }
}
/**Radio reset

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFRST {
    ///0: Sub-GHz radio software reset removed
    Removed = 0,
    ///1: Sub-GHz radio software reset active
    Reset = 1,
}
impl From<RFRST> for bool {
    #[inline(always)]
    fn from(variant: RFRST) -> Self {
        variant as u8 != 0
    }
}
///Field `RFRST` reader - Radio reset
pub type RFRST_R = crate::BitReader<RFRST>;
impl RFRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RFRST {
        match self.bits {
            false => RFRST::Removed,
            true => RFRST::Reset,
        }
    }
    ///Sub-GHz radio software reset removed
    #[inline(always)]
    pub fn is_removed(&self) -> bool {
        *self == RFRST::Removed
    }
    ///Sub-GHz radio software reset active
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RFRST::Reset
    }
}
///Field `RFRST` writer - Radio reset
pub type RFRST_W<'a, REG> = crate::BitWriter<'a, REG, RFRST>;
impl<'a, REG> RFRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sub-GHz radio software reset removed
    #[inline(always)]
    pub fn removed(self) -> &'a mut crate::W<REG> {
        self.variant(RFRST::Removed)
    }
    ///Sub-GHz radio software reset active
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(RFRST::Reset)
    }
}
/**Remove reset flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMVF {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset flags reset
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
    ///Reset flags reset
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
    ///Reset flags reset
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RMVF::Clear)
    }
}
/**Radio illegal access flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFILARSTF {
    ///0: No SUBGHZ radio illegal command occurred
    NoIllegalCommand = 0,
    ///1: SUBGHZ radio illegal command occurred
    IllegalCommand = 1,
}
impl From<RFILARSTF> for bool {
    #[inline(always)]
    fn from(variant: RFILARSTF) -> Self {
        variant as u8 != 0
    }
}
///Field `RFILARSTF` reader - Radio illegal access flag
pub type RFILARSTF_R = crate::BitReader<RFILARSTF>;
impl RFILARSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RFILARSTF {
        match self.bits {
            false => RFILARSTF::NoIllegalCommand,
            true => RFILARSTF::IllegalCommand,
        }
    }
    ///No SUBGHZ radio illegal command occurred
    #[inline(always)]
    pub fn is_no_illegal_command(&self) -> bool {
        *self == RFILARSTF::NoIllegalCommand
    }
    ///SUBGHZ radio illegal command occurred
    #[inline(always)]
    pub fn is_illegal_command(&self) -> bool {
        *self == RFILARSTF::IllegalCommand
    }
}
/**Option byte loader reset flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBLRSTF {
    ///0: No reset occurred
    NoReset = 0,
    ///1: Reset occurred
    Reset = 1,
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
            false => OBLRSTF::NoReset,
            true => OBLRSTF::Reset,
        }
    }
    ///No reset occurred
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == OBLRSTF::NoReset
    }
    ///Reset occurred
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == OBLRSTF::Reset
    }
}
///Field `PINRSTF` reader - Pin reset flag
pub use OBLRSTF_R as PINRSTF_R;
///Field `BORRSTF` reader - BOR flag
pub use OBLRSTF_R as BORRSTF_R;
///Field `SFTRSTF` reader - Software reset flag
pub use OBLRSTF_R as SFTRSTF_R;
///Field `IWDGRSTF` reader - Independent window watchdog reset flag
pub use OBLRSTF_R as IWDGRSTF_R;
///Field `WWDGRSTF` reader - Window watchdog reset flag
pub use OBLRSTF_R as WWDGRSTF_R;
///Field `LPWRRSTF` reader - Low-power reset flag
pub use OBLRSTF_R as LPWRRSTF_R;
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
    ///Bit 4 - LSI frequency prescaler
    #[inline(always)]
    pub fn lsipre(&self) -> LSIPRE_R {
        LSIPRE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:11 - MSI clock ranges
    #[inline(always)]
    pub fn msisrange(&self) -> MSISRANGE_R {
        MSISRANGE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 14 - Radio in reset status flag
    #[inline(always)]
    pub fn rfrstf(&self) -> RFRSTF_R {
        RFRSTF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Radio reset
    #[inline(always)]
    pub fn rfrst(&self) -> RFRST_R {
        RFRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 23 - Remove reset flag
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Radio illegal access flag
    #[inline(always)]
    pub fn rfilarstf(&self) -> RFILARSTF_R {
        RFILARSTF_R::new(((self.bits >> 24) & 1) != 0)
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
            .field("oblrstf", &self.oblrstf())
            .field("lpwrrstf", &self.lpwrrstf())
            .field("wwdgrstf", &self.wwdgrstf())
            .field("iwdgrstf", &self.iwdgrstf())
            .field("sftrstf", &self.sftrstf())
            .field("borrstf", &self.borrstf())
            .field("pinrstf", &self.pinrstf())
            .field("rfilarstf", &self.rfilarstf())
            .field("rmvf", &self.rmvf())
            .field("rfrst", &self.rfrst())
            .field("rfrstf", &self.rfrstf())
            .field("msisrange", &self.msisrange())
            .field("lsipre", &self.lsipre())
            .field("lsirdy", &self.lsirdy())
            .field("lsion", &self.lsion())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSI oscillator enable
    #[inline(always)]
    pub fn lsion(&mut self) -> LSION_W<'_, CSRrs> {
        LSION_W::new(self, 0)
    }
    ///Bit 4 - LSI frequency prescaler
    #[inline(always)]
    pub fn lsipre(&mut self) -> LSIPRE_W<'_, CSRrs> {
        LSIPRE_W::new(self, 4)
    }
    ///Bits 8:11 - MSI clock ranges
    #[inline(always)]
    pub fn msisrange(&mut self) -> MSISRANGE_W<'_, CSRrs> {
        MSISRANGE_W::new(self, 8)
    }
    ///Bit 15 - Radio reset
    #[inline(always)]
    pub fn rfrst(&mut self) -> RFRST_W<'_, CSRrs> {
        RFRST_W::new(self, 15)
    }
    ///Bit 23 - Remove reset flag
    #[inline(always)]
    pub fn rmvf(&mut self) -> RMVF_W<'_, CSRrs> {
        RMVF_W::new(self, 23)
    }
}
/**Control/status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#RCC:CSR)*/
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
///`reset()` method sets CSR to value 0x0c01_c600
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0x0c01_c600;
}
