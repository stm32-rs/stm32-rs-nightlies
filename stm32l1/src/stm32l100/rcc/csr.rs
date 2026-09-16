///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Register `CSR` writer
pub type W = crate::W<CSRrs>;
///Field `LSION` reader - Internal low-speed oscillator enable
pub type LSION_R = crate::BitReader;
///Field `LSION` writer - Internal low-speed oscillator enable
pub type LSION_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSIRDY` reader - Internal low-speed oscillator ready
pub type LSIRDY_R = crate::BitReader;
///Field `LSEON` reader - External low-speed oscillator enable
pub type LSEON_R = crate::BitReader;
///Field `LSEON` writer - External low-speed oscillator enable
pub type LSEON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSERDY` reader - External low-speed oscillator ready
pub type LSERDY_R = crate::BitReader;
///Field `LSEBYP` reader - External low-speed oscillator bypass
pub type LSEBYP_R = crate::BitReader;
///Field `LSEBYP` writer - External low-speed oscillator bypass
pub type LSEBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSECSSON` reader - CSS on LSE enable
pub type LSECSSON_R = crate::BitReader;
///Field `LSECSSON` writer - CSS on LSE enable
pub type LSECSSON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSECSSD` reader - CSS on LSE failure Detection
pub type LSECSSD_R = crate::BitReader;
///Field `LSECSSD` writer - CSS on LSE failure Detection
pub type LSECSSD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCSEL` reader - RTC and LCD clock source selection
pub type RTCSEL_R = crate::FieldReader;
///Field `RTCSEL` writer - RTC and LCD clock source selection
pub type RTCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RTCEN` reader - RTC clock enable
pub type RTCEN_R = crate::BitReader;
///Field `RTCEN` writer - RTC clock enable
pub type RTCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
/**RTC software reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCRSTW {
    ///1: Resets the RTC peripheral
    Reset = 1,
}
impl From<RTCRSTW> for bool {
    #[inline(always)]
    fn from(variant: RTCRSTW) -> Self {
        variant as u8 != 0
    }
}
///Field `RTCRST` reader - RTC software reset
pub type RTCRST_R = crate::BitReader<RTCRSTW>;
impl RTCRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<RTCRSTW> {
        match self.bits {
            true => Some(RTCRSTW::Reset),
            _ => None,
        }
    }
    ///Resets the RTC peripheral
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RTCRSTW::Reset
    }
}
///Field `RTCRST` writer - RTC software reset
pub type RTCRST_W<'a, REG> = crate::BitWriter<'a, REG, RTCRSTW>;
impl<'a, REG> RTCRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Resets the RTC peripheral
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(RTCRSTW::Reset)
    }
}
/**Remove reset flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMVFW {
    ///1: Clears the reset flag
    Clear = 1,
}
impl From<RMVFW> for bool {
    #[inline(always)]
    fn from(variant: RMVFW) -> Self {
        variant as u8 != 0
    }
}
///Field `RMVF` reader - Remove reset flag
pub type RMVF_R = crate::BitReader<RMVFW>;
impl RMVF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<RMVFW> {
        match self.bits {
            true => Some(RMVFW::Clear),
            _ => None,
        }
    }
    ///Clears the reset flag
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RMVFW::Clear
    }
}
///Field `RMVF` writer - Remove reset flag
pub type RMVF_W<'a, REG> = crate::BitWriter<'a, REG, RMVFW>;
impl<'a, REG> RMVF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the reset flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RMVFW::Clear)
    }
}
/**Options bytes loading reset flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBLRSTFR {
    ///0: No reset has occured
    NoReset = 0,
    ///1: A reset has occured
    Reset = 1,
}
impl From<OBLRSTFR> for bool {
    #[inline(always)]
    fn from(variant: OBLRSTFR) -> Self {
        variant as u8 != 0
    }
}
///Field `OBLRSTF` reader - Options bytes loading reset flag
pub type OBLRSTF_R = crate::BitReader<OBLRSTFR>;
impl OBLRSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OBLRSTFR {
        match self.bits {
            false => OBLRSTFR::NoReset,
            true => OBLRSTFR::Reset,
        }
    }
    ///No reset has occured
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == OBLRSTFR::NoReset
    }
    ///A reset has occured
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == OBLRSTFR::Reset
    }
}
///Field `OBLRSTF` writer - Options bytes loading reset flag
pub type OBLRSTF_W<'a, REG> = crate::BitWriter<'a, REG, OBLRSTFR>;
impl<'a, REG> OBLRSTF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No reset has occured
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut crate::W<REG> {
        self.variant(OBLRSTFR::NoReset)
    }
    ///A reset has occured
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(OBLRSTFR::Reset)
    }
}
///Field `PINRSTF` reader - PIN reset flag
pub use OBLRSTF_R as PINRSTF_R;
///Field `PORRSTF` reader - POR/PDR reset flag
pub use OBLRSTF_R as PORRSTF_R;
///Field `SFTRSTF` reader - Software reset flag
pub use OBLRSTF_R as SFTRSTF_R;
///Field `IWDGRSTF` reader - Independent watchdog reset flag
pub use OBLRSTF_R as IWDGRSTF_R;
///Field `WWDGRSTF` reader - Window watchdog reset flag
pub use OBLRSTF_R as WWDGRSTF_R;
///Field `LPWRSTF` reader - Low-power reset flag
pub use OBLRSTF_R as LPWRSTF_R;
///Field `PINRSTF` writer - PIN reset flag
pub use OBLRSTF_W as PINRSTF_W;
///Field `PORRSTF` writer - POR/PDR reset flag
pub use OBLRSTF_W as PORRSTF_W;
///Field `SFTRSTF` writer - Software reset flag
pub use OBLRSTF_W as SFTRSTF_W;
///Field `IWDGRSTF` writer - Independent watchdog reset flag
pub use OBLRSTF_W as IWDGRSTF_W;
///Field `WWDGRSTF` writer - Window watchdog reset flag
pub use OBLRSTF_W as WWDGRSTF_W;
///Field `LPWRSTF` writer - Low-power reset flag
pub use OBLRSTF_W as LPWRSTF_W;
impl R {
    ///Bit 0 - Internal low-speed oscillator enable
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Internal low-speed oscillator ready
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - External low-speed oscillator enable
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - External low-speed oscillator ready
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - External low-speed oscillator bypass
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CSS on LSE enable
    #[inline(always)]
    pub fn lsecsson(&self) -> LSECSSON_R {
        LSECSSON_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CSS on LSE failure Detection
    #[inline(always)]
    pub fn lsecssd(&self) -> LSECSSD_R {
        LSECSSD_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 16:17 - RTC and LCD clock source selection
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 22 - RTC clock enable
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - RTC software reset
    #[inline(always)]
    pub fn rtcrst(&self) -> RTCRST_R {
        RTCRST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Remove reset flag
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Options bytes loading reset flag
    #[inline(always)]
    pub fn oblrstf(&self) -> OBLRSTF_R {
        OBLRSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - PIN reset flag
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - POR/PDR reset flag
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Software reset flag
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Independent watchdog reset flag
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
    pub fn lpwrstf(&self) -> LPWRSTF_R {
        LPWRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("oblrstf", &self.oblrstf())
            .field("lpwrstf", &self.lpwrstf())
            .field("wwdgrstf", &self.wwdgrstf())
            .field("iwdgrstf", &self.iwdgrstf())
            .field("sftrstf", &self.sftrstf())
            .field("porrstf", &self.porrstf())
            .field("pinrstf", &self.pinrstf())
            .field("rmvf", &self.rmvf())
            .field("rtcrst", &self.rtcrst())
            .field("rtcen", &self.rtcen())
            .field("rtcsel", &self.rtcsel())
            .field("lsebyp", &self.lsebyp())
            .field("lserdy", &self.lserdy())
            .field("lseon", &self.lseon())
            .field("lsirdy", &self.lsirdy())
            .field("lsion", &self.lsion())
            .field("lsecssd", &self.lsecssd())
            .field("lsecsson", &self.lsecsson())
            .finish()
    }
}
impl W {
    ///Bit 0 - Internal low-speed oscillator enable
    #[inline(always)]
    pub fn lsion(&mut self) -> LSION_W<'_, CSRrs> {
        LSION_W::new(self, 0)
    }
    ///Bit 8 - External low-speed oscillator enable
    #[inline(always)]
    pub fn lseon(&mut self) -> LSEON_W<'_, CSRrs> {
        LSEON_W::new(self, 8)
    }
    ///Bit 10 - External low-speed oscillator bypass
    #[inline(always)]
    pub fn lsebyp(&mut self) -> LSEBYP_W<'_, CSRrs> {
        LSEBYP_W::new(self, 10)
    }
    ///Bit 11 - CSS on LSE enable
    #[inline(always)]
    pub fn lsecsson(&mut self) -> LSECSSON_W<'_, CSRrs> {
        LSECSSON_W::new(self, 11)
    }
    ///Bit 12 - CSS on LSE failure Detection
    #[inline(always)]
    pub fn lsecssd(&mut self) -> LSECSSD_W<'_, CSRrs> {
        LSECSSD_W::new(self, 12)
    }
    ///Bits 16:17 - RTC and LCD clock source selection
    #[inline(always)]
    pub fn rtcsel(&mut self) -> RTCSEL_W<'_, CSRrs> {
        RTCSEL_W::new(self, 16)
    }
    ///Bit 22 - RTC clock enable
    #[inline(always)]
    pub fn rtcen(&mut self) -> RTCEN_W<'_, CSRrs> {
        RTCEN_W::new(self, 22)
    }
    ///Bit 23 - RTC software reset
    #[inline(always)]
    pub fn rtcrst(&mut self) -> RTCRST_W<'_, CSRrs> {
        RTCRST_W::new(self, 23)
    }
    ///Bit 24 - Remove reset flag
    #[inline(always)]
    pub fn rmvf(&mut self) -> RMVF_W<'_, CSRrs> {
        RMVF_W::new(self, 24)
    }
    ///Bit 25 - Options bytes loading reset flag
    #[inline(always)]
    pub fn oblrstf(&mut self) -> OBLRSTF_W<'_, CSRrs> {
        OBLRSTF_W::new(self, 25)
    }
    ///Bit 26 - PIN reset flag
    #[inline(always)]
    pub fn pinrstf(&mut self) -> PINRSTF_W<'_, CSRrs> {
        PINRSTF_W::new(self, 26)
    }
    ///Bit 27 - POR/PDR reset flag
    #[inline(always)]
    pub fn porrstf(&mut self) -> PORRSTF_W<'_, CSRrs> {
        PORRSTF_W::new(self, 27)
    }
    ///Bit 28 - Software reset flag
    #[inline(always)]
    pub fn sftrstf(&mut self) -> SFTRSTF_W<'_, CSRrs> {
        SFTRSTF_W::new(self, 28)
    }
    ///Bit 29 - Independent watchdog reset flag
    #[inline(always)]
    pub fn iwdgrstf(&mut self) -> IWDGRSTF_W<'_, CSRrs> {
        IWDGRSTF_W::new(self, 29)
    }
    ///Bit 30 - Window watchdog reset flag
    #[inline(always)]
    pub fn wwdgrstf(&mut self) -> WWDGRSTF_W<'_, CSRrs> {
        WWDGRSTF_W::new(self, 30)
    }
    ///Bit 31 - Low-power reset flag
    #[inline(always)]
    pub fn lpwrstf(&mut self) -> LPWRSTF_W<'_, CSRrs> {
        LPWRSTF_W::new(self, 31)
    }
}
/**Control/status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RCC:CSR)*/
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
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSRrs {}
