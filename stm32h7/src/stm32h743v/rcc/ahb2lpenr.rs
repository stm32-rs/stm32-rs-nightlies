///Register `AHB2LPENR` reader
pub type R = crate::R<AHB2LPENRrs>;
///Register `AHB2LPENR` writer
pub type W = crate::W<AHB2LPENRrs>;
/**DCMI peripheral clock enable during csleep mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMILPEN {
    ///0: The selected clock is disabled during csleep mode
    Disabled = 0,
    ///1: The selected clock is enabled during csleep mode
    Enabled = 1,
}
impl From<DCMILPEN> for bool {
    #[inline(always)]
    fn from(variant: DCMILPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DCMILPEN` reader - DCMI peripheral clock enable during csleep mode
pub type DCMILPEN_R = crate::BitReader<DCMILPEN>;
impl DCMILPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DCMILPEN {
        match self.bits {
            false => DCMILPEN::Disabled,
            true => DCMILPEN::Enabled,
        }
    }
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCMILPEN::Disabled
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCMILPEN::Enabled
    }
}
///Field `DCMILPEN` writer - DCMI peripheral clock enable during csleep mode
pub type DCMILPEN_W<'a, REG> = crate::BitWriter<'a, REG, DCMILPEN>;
impl<'a, REG> DCMILPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCMILPEN::Disabled)
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCMILPEN::Enabled)
    }
}
///Field `CRYPTLPEN` reader - CRYPT peripheral clock enable during CSleep mode
pub use DCMILPEN_R as CRYPTLPEN_R;
///Field `HASHLPEN` reader - HASH peripheral clock enable during CSleep mode
pub use DCMILPEN_R as HASHLPEN_R;
///Field `RNGLPEN` reader - RNG peripheral clock enable during CSleep mode
pub use DCMILPEN_R as RNGLPEN_R;
///Field `SDMMC2LPEN` reader - SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode
pub use DCMILPEN_R as SDMMC2LPEN_R;
///Field `SRAM1LPEN` reader - SRAM1 Clock Enable During CSleep Mode
pub use DCMILPEN_R as SRAM1LPEN_R;
///Field `SRAM2LPEN` reader - SRAM2 Clock Enable During CSleep Mode
pub use DCMILPEN_R as SRAM2LPEN_R;
///Field `SRAM3LPEN` reader - SRAM3 Clock Enable During CSleep Mode
pub use DCMILPEN_R as SRAM3LPEN_R;
///Field `CRYPTLPEN` writer - CRYPT peripheral clock enable during CSleep mode
pub use DCMILPEN_W as CRYPTLPEN_W;
///Field `HASHLPEN` writer - HASH peripheral clock enable during CSleep mode
pub use DCMILPEN_W as HASHLPEN_W;
///Field `RNGLPEN` writer - RNG peripheral clock enable during CSleep mode
pub use DCMILPEN_W as RNGLPEN_W;
///Field `SDMMC2LPEN` writer - SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode
pub use DCMILPEN_W as SDMMC2LPEN_W;
///Field `SRAM1LPEN` writer - SRAM1 Clock Enable During CSleep Mode
pub use DCMILPEN_W as SRAM1LPEN_W;
///Field `SRAM2LPEN` writer - SRAM2 Clock Enable During CSleep Mode
pub use DCMILPEN_W as SRAM2LPEN_W;
///Field `SRAM3LPEN` writer - SRAM3 Clock Enable During CSleep Mode
pub use DCMILPEN_W as SRAM3LPEN_W;
impl R {
    ///Bit 0 - DCMI peripheral clock enable during csleep mode
    #[inline(always)]
    pub fn dcmilpen(&self) -> DCMILPEN_R {
        DCMILPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - CRYPT peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn cryptlpen(&self) -> CRYPTLPEN_R {
        CRYPTLPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HASH peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn hashlpen(&self) -> HASHLPEN_R {
        HASHLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RNG peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn rnglpen(&self) -> RNGLPEN_R {
        RNGLPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn sdmmc2lpen(&self) -> SDMMC2LPEN_R {
        SDMMC2LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 29 - SRAM1 Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn sram1lpen(&self) -> SRAM1LPEN_R {
        SRAM1LPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - SRAM2 Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn sram2lpen(&self) -> SRAM2LPEN_R {
        SRAM2LPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - SRAM3 Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn sram3lpen(&self) -> SRAM3LPEN_R {
        SRAM3LPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2LPENR")
            .field("dcmilpen", &self.dcmilpen())
            .field("cryptlpen", &self.cryptlpen())
            .field("hashlpen", &self.hashlpen())
            .field("sdmmc2lpen", &self.sdmmc2lpen())
            .field("rnglpen", &self.rnglpen())
            .field("sram1lpen", &self.sram1lpen())
            .field("sram2lpen", &self.sram2lpen())
            .field("sram3lpen", &self.sram3lpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - DCMI peripheral clock enable during csleep mode
    #[inline(always)]
    pub fn dcmilpen(&mut self) -> DCMILPEN_W<'_, AHB2LPENRrs> {
        DCMILPEN_W::new(self, 0)
    }
    ///Bit 4 - CRYPT peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn cryptlpen(&mut self) -> CRYPTLPEN_W<'_, AHB2LPENRrs> {
        CRYPTLPEN_W::new(self, 4)
    }
    ///Bit 5 - HASH peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn hashlpen(&mut self) -> HASHLPEN_W<'_, AHB2LPENRrs> {
        HASHLPEN_W::new(self, 5)
    }
    ///Bit 6 - RNG peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn rnglpen(&mut self) -> RNGLPEN_W<'_, AHB2LPENRrs> {
        RNGLPEN_W::new(self, 6)
    }
    ///Bit 9 - SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn sdmmc2lpen(&mut self) -> SDMMC2LPEN_W<'_, AHB2LPENRrs> {
        SDMMC2LPEN_W::new(self, 9)
    }
    ///Bit 29 - SRAM1 Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn sram1lpen(&mut self) -> SRAM1LPEN_W<'_, AHB2LPENRrs> {
        SRAM1LPEN_W::new(self, 29)
    }
    ///Bit 30 - SRAM2 Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn sram2lpen(&mut self) -> SRAM2LPEN_W<'_, AHB2LPENRrs> {
        SRAM2LPEN_W::new(self, 30)
    }
    ///Bit 31 - SRAM3 Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn sram3lpen(&mut self) -> SRAM3LPEN_W<'_, AHB2LPENRrs> {
        SRAM3LPEN_W::new(self, 31)
    }
}
/**RCC AHB2 Sleep Clock Register

You can [`read`](crate::Reg::read) this register and get [`ahb2lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743V.html#RCC:AHB2LPENR)*/
pub struct AHB2LPENRrs;
impl crate::RegisterSpec for AHB2LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb2lpenr::R`](R) reader structure
impl crate::Readable for AHB2LPENRrs {}
///`write(|w| ..)` method takes [`ahb2lpenr::W`](W) writer structure
impl crate::Writable for AHB2LPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2LPENR to value 0
impl crate::Resettable for AHB2LPENRrs {}
