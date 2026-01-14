///Register `AHB3LPENR` reader
pub type R = crate::R<AHB3LPENRrs>;
///Register `AHB3LPENR` writer
pub type W = crate::W<AHB3LPENRrs>;
/**RNG peripheral clock enable in low-power mode

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNGLPEN {
    ///0: The selected clock is disabled during csleep mode
    Disabled = 0,
    ///1: The selected clock is enabled during csleep mode
    Enabled = 1,
}
impl From<RNGLPEN> for bool {
    #[inline(always)]
    fn from(variant: RNGLPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `RNGLPEN` reader - RNG peripheral clock enable in low-power mode
pub type RNGLPEN_R = crate::BitReader<RNGLPEN>;
impl RNGLPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RNGLPEN {
        match self.bits {
            false => RNGLPEN::Disabled,
            true => RNGLPEN::Enabled,
        }
    }
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RNGLPEN::Disabled
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RNGLPEN::Enabled
    }
}
///Field `RNGLPEN` writer - RNG peripheral clock enable in low-power mode
pub type RNGLPEN_W<'a, REG> = crate::BitWriter<'a, REG, RNGLPEN>;
impl<'a, REG> RNGLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RNGLPEN::Disabled)
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RNGLPEN::Enabled)
    }
}
///Field `HASHLPEN` reader - HASH peripheral clock enable in low-power mode
pub use RNGLPEN_R as HASHLPEN_R;
///Field `CRYPLPEN` reader - CRYP peripheral clock enable in low-power mode
pub use RNGLPEN_R as CRYPLPEN_R;
///Field `SAESLPEN` reader - SAES peripheral clock enable in low-power mode
pub use RNGLPEN_R as SAESLPEN_R;
///Field `PKALPEN` reader - PKA peripheral clock enable in low-power mode
pub use RNGLPEN_R as PKALPEN_R;
///Field `HASHLPEN` writer - HASH peripheral clock enable in low-power mode
pub use RNGLPEN_W as HASHLPEN_W;
///Field `CRYPLPEN` writer - CRYP peripheral clock enable in low-power mode
pub use RNGLPEN_W as CRYPLPEN_W;
///Field `SAESLPEN` writer - SAES peripheral clock enable in low-power mode
pub use RNGLPEN_W as SAESLPEN_W;
///Field `PKALPEN` writer - PKA peripheral clock enable in low-power mode
pub use RNGLPEN_W as PKALPEN_W;
impl R {
    ///Bit 0 - RNG peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn rnglpen(&self) -> RNGLPEN_R {
        RNGLPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HASH peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn hashlpen(&self) -> HASHLPEN_R {
        HASHLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CRYP peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn cryplpen(&self) -> CRYPLPEN_R {
        CRYPLPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - SAES peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn saeslpen(&self) -> SAESLPEN_R {
        SAESLPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - PKA peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn pkalpen(&self) -> PKALPEN_R {
        PKALPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB3LPENR")
            .field("rnglpen", &self.rnglpen())
            .field("hashlpen", &self.hashlpen())
            .field("cryplpen", &self.cryplpen())
            .field("saeslpen", &self.saeslpen())
            .field("pkalpen", &self.pkalpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - RNG peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn rnglpen(&mut self) -> RNGLPEN_W<'_, AHB3LPENRrs> {
        RNGLPEN_W::new(self, 0)
    }
    ///Bit 1 - HASH peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn hashlpen(&mut self) -> HASHLPEN_W<'_, AHB3LPENRrs> {
        HASHLPEN_W::new(self, 1)
    }
    ///Bit 2 - CRYP peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn cryplpen(&mut self) -> CRYPLPEN_W<'_, AHB3LPENRrs> {
        CRYPLPEN_W::new(self, 2)
    }
    ///Bit 4 - SAES peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn saeslpen(&mut self) -> SAESLPEN_W<'_, AHB3LPENRrs> {
        SAESLPEN_W::new(self, 4)
    }
    ///Bit 6 - PKA peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn pkalpen(&mut self) -> PKALPEN_W<'_, AHB3LPENRrs> {
        PKALPEN_W::new(self, 6)
    }
}
/**RCC AHB3 low-power clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb3lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:AHB3LPENR)*/
pub struct AHB3LPENRrs;
impl crate::RegisterSpec for AHB3LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb3lpenr::R`](R) reader structure
impl crate::Readable for AHB3LPENRrs {}
///`write(|w| ..)` method takes [`ahb3lpenr::W`](W) writer structure
impl crate::Writable for AHB3LPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB3LPENR to value 0x57
impl crate::Resettable for AHB3LPENRrs {
    const RESET_VALUE: u32 = 0x57;
}
