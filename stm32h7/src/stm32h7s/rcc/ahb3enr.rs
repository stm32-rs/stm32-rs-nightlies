///Register `AHB3ENR` reader
pub type R = crate::R<AHB3ENRrs>;
///Register `AHB3ENR` writer
pub type W = crate::W<AHB3ENRrs>;
/**RNG peripheral clocks enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNGEN {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<RNGEN> for bool {
    #[inline(always)]
    fn from(variant: RNGEN) -> Self {
        variant as u8 != 0
    }
}
///Field `RNGEN` reader - RNG peripheral clocks enable
pub type RNGEN_R = crate::BitReader<RNGEN>;
impl RNGEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RNGEN {
        match self.bits {
            false => RNGEN::Disabled,
            true => RNGEN::Enabled,
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RNGEN::Disabled
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RNGEN::Enabled
    }
}
///Field `RNGEN` writer - RNG peripheral clocks enable
pub type RNGEN_W<'a, REG> = crate::BitWriter<'a, REG, RNGEN>;
impl<'a, REG> RNGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RNGEN::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RNGEN::Enabled)
    }
}
///Field `HASHEN` reader - HASH peripheral clock enable
pub use RNGEN_R as HASHEN_R;
///Field `CRYPEN` reader - CRYP peripheral clock enable
pub use RNGEN_R as CRYPEN_R;
///Field `SAESEN` reader - SAES peripheral clock enable
pub use RNGEN_R as SAESEN_R;
///Field `PKAEN` reader - PKA peripheral clock enable
pub use RNGEN_R as PKAEN_R;
///Field `HASHEN` writer - HASH peripheral clock enable
pub use RNGEN_W as HASHEN_W;
///Field `CRYPEN` writer - CRYP peripheral clock enable
pub use RNGEN_W as CRYPEN_W;
///Field `SAESEN` writer - SAES peripheral clock enable
pub use RNGEN_W as SAESEN_W;
///Field `PKAEN` writer - PKA peripheral clock enable
pub use RNGEN_W as PKAEN_W;
impl R {
    ///Bit 0 - RNG peripheral clocks enable
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HASH peripheral clock enable
    #[inline(always)]
    pub fn hashen(&self) -> HASHEN_R {
        HASHEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CRYP peripheral clock enable
    #[inline(always)]
    pub fn crypen(&self) -> CRYPEN_R {
        CRYPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - SAES peripheral clock enable
    #[inline(always)]
    pub fn saesen(&self) -> SAESEN_R {
        SAESEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - PKA peripheral clock enable
    #[inline(always)]
    pub fn pkaen(&self) -> PKAEN_R {
        PKAEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB3ENR")
            .field("rngen", &self.rngen())
            .field("hashen", &self.hashen())
            .field("crypen", &self.crypen())
            .field("saesen", &self.saesen())
            .field("pkaen", &self.pkaen())
            .finish()
    }
}
impl W {
    ///Bit 0 - RNG peripheral clocks enable
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W<'_, AHB3ENRrs> {
        RNGEN_W::new(self, 0)
    }
    ///Bit 1 - HASH peripheral clock enable
    #[inline(always)]
    pub fn hashen(&mut self) -> HASHEN_W<'_, AHB3ENRrs> {
        HASHEN_W::new(self, 1)
    }
    ///Bit 2 - CRYP peripheral clock enable
    #[inline(always)]
    pub fn crypen(&mut self) -> CRYPEN_W<'_, AHB3ENRrs> {
        CRYPEN_W::new(self, 2)
    }
    ///Bit 4 - SAES peripheral clock enable
    #[inline(always)]
    pub fn saesen(&mut self) -> SAESEN_W<'_, AHB3ENRrs> {
        SAESEN_W::new(self, 4)
    }
    ///Bit 6 - PKA peripheral clock enable
    #[inline(always)]
    pub fn pkaen(&mut self) -> PKAEN_W<'_, AHB3ENRrs> {
        PKAEN_W::new(self, 6)
    }
}
/**RCC AHB3 clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb3enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#RCC:AHB3ENR)*/
pub struct AHB3ENRrs;
impl crate::RegisterSpec for AHB3ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb3enr::R`](R) reader structure
impl crate::Readable for AHB3ENRrs {}
///`write(|w| ..)` method takes [`ahb3enr::W`](W) writer structure
impl crate::Writable for AHB3ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB3ENR to value 0
impl crate::Resettable for AHB3ENRrs {}
