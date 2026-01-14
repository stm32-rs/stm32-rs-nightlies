///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Register `CSR` writer
pub type W = crate::W<CSRrs>;
/**Internal low-speed oscillator enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSION {
    ///0: LSI oscillator Off
    Off = 0,
    ///1: LSI oscillator On
    On = 1,
}
impl From<LSION> for bool {
    #[inline(always)]
    fn from(variant: LSION) -> Self {
        variant as u8 != 0
    }
}
///Field `LSION` reader - Internal low-speed oscillator enable
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
    ///LSI oscillator Off
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LSION::Off
    }
    ///LSI oscillator On
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == LSION::On
    }
}
///Field `LSION` writer - Internal low-speed oscillator enable
pub type LSION_W<'a, REG> = crate::BitWriter<'a, REG, LSION>;
impl<'a, REG> LSION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LSI oscillator Off
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(LSION::Off)
    }
    ///LSI oscillator On
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(LSION::On)
    }
}
/**Internal low-speed oscillator ready

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYR {
    ///0: LSI oscillator not ready
    NotReady = 0,
    ///1: LSI oscillator ready
    Ready = 1,
}
impl From<LSIRDYR> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYR) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIRDY` reader - Internal low-speed oscillator ready
pub type LSIRDY_R = crate::BitReader<LSIRDYR>;
impl LSIRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSIRDYR {
        match self.bits {
            false => LSIRDYR::NotReady,
            true => LSIRDYR::Ready,
        }
    }
    ///LSI oscillator not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LSIRDYR::NotReady
    }
    ///LSI oscillator ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LSIRDYR::Ready
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
/**BOR reset flag

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BORRSTFR {
    ///0: No reset has occured
    NoReset = 0,
    ///1: A reset has occured
    Reset = 1,
}
impl From<BORRSTFR> for bool {
    #[inline(always)]
    fn from(variant: BORRSTFR) -> Self {
        variant as u8 != 0
    }
}
///Field `BORRSTF` reader - BOR reset flag
pub type BORRSTF_R = crate::BitReader<BORRSTFR>;
impl BORRSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BORRSTFR {
        match self.bits {
            false => BORRSTFR::NoReset,
            true => BORRSTFR::Reset,
        }
    }
    ///No reset has occured
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == BORRSTFR::NoReset
    }
    ///A reset has occured
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == BORRSTFR::Reset
    }
}
///Field `BORRSTF` writer - BOR reset flag
pub type BORRSTF_W<'a, REG> = crate::BitWriter<'a, REG, BORRSTFR>;
impl<'a, REG> BORRSTF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No reset has occured
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut crate::W<REG> {
        self.variant(BORRSTFR::NoReset)
    }
    ///A reset has occured
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(BORRSTFR::Reset)
    }
}
///Field `PADRSTF` reader - PIN reset flag
pub use BORRSTF_R as PADRSTF_R;
///Field `PORRSTF` reader - POR/PDR reset flag
pub use BORRSTF_R as PORRSTF_R;
///Field `SFTRSTF` reader - Software reset flag
pub use BORRSTF_R as SFTRSTF_R;
///Field `WDGRSTF` reader - Independent watchdog reset flag
pub use BORRSTF_R as WDGRSTF_R;
///Field `WWDGRSTF` reader - Window watchdog reset flag
pub use BORRSTF_R as WWDGRSTF_R;
///Field `LPWRRSTF` reader - Low-power reset flag
pub use BORRSTF_R as LPWRRSTF_R;
///Field `PADRSTF` writer - PIN reset flag
pub use BORRSTF_W as PADRSTF_W;
///Field `PORRSTF` writer - POR/PDR reset flag
pub use BORRSTF_W as PORRSTF_W;
///Field `SFTRSTF` writer - Software reset flag
pub use BORRSTF_W as SFTRSTF_W;
///Field `WDGRSTF` writer - Independent watchdog reset flag
pub use BORRSTF_W as WDGRSTF_W;
///Field `WWDGRSTF` writer - Window watchdog reset flag
pub use BORRSTF_W as WWDGRSTF_W;
///Field `LPWRRSTF` writer - Low-power reset flag
pub use BORRSTF_W as LPWRRSTF_W;
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
    ///Bit 24 - Remove reset flag
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - BOR reset flag
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - PIN reset flag
    #[inline(always)]
    pub fn padrstf(&self) -> PADRSTF_R {
        PADRSTF_R::new(((self.bits >> 26) & 1) != 0)
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
    pub fn wdgrstf(&self) -> WDGRSTF_R {
        WDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
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
            .field("borrstf", &self.borrstf())
            .field("lpwrrstf", &self.lpwrrstf())
            .field("wwdgrstf", &self.wwdgrstf())
            .field("wdgrstf", &self.wdgrstf())
            .field("sftrstf", &self.sftrstf())
            .field("porrstf", &self.porrstf())
            .field("padrstf", &self.padrstf())
            .field("rmvf", &self.rmvf())
            .field("lsirdy", &self.lsirdy())
            .field("lsion", &self.lsion())
            .finish()
    }
}
impl W {
    ///Bit 0 - Internal low-speed oscillator enable
    #[inline(always)]
    pub fn lsion(&mut self) -> LSION_W<'_, CSRrs> {
        LSION_W::new(self, 0)
    }
    ///Bit 24 - Remove reset flag
    #[inline(always)]
    pub fn rmvf(&mut self) -> RMVF_W<'_, CSRrs> {
        RMVF_W::new(self, 24)
    }
    ///Bit 25 - BOR reset flag
    #[inline(always)]
    pub fn borrstf(&mut self) -> BORRSTF_W<'_, CSRrs> {
        BORRSTF_W::new(self, 25)
    }
    ///Bit 26 - PIN reset flag
    #[inline(always)]
    pub fn padrstf(&mut self) -> PADRSTF_W<'_, CSRrs> {
        PADRSTF_W::new(self, 26)
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
    pub fn wdgrstf(&mut self) -> WDGRSTF_W<'_, CSRrs> {
        WDGRSTF_W::new(self, 29)
    }
    ///Bit 30 - Window watchdog reset flag
    #[inline(always)]
    pub fn wwdgrstf(&mut self) -> WWDGRSTF_W<'_, CSRrs> {
        WWDGRSTF_W::new(self, 30)
    }
    ///Bit 31 - Low-power reset flag
    #[inline(always)]
    pub fn lpwrrstf(&mut self) -> LPWRRSTF_W<'_, CSRrs> {
        LPWRRSTF_W::new(self, 31)
    }
}
/**clock control & status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F732.html#RCC:CSR)*/
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
///`reset()` method sets CSR to value 0x0e00_0000
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0x0e00_0000;
}
