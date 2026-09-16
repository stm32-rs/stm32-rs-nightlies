///Register `AHB2ENR` reader
pub type R = crate::R<AHB2ENRrs>;
///Register `AHB2ENR` writer
pub type W = crate::W<AHB2ENRrs>;
/**RNGEN

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
///Field `RNGEN` reader - RNGEN
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
///Field `RNGEN` writer - RNGEN
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
///Field `OTGFSEN` reader - USB OTG FS clock enable
pub use RNGEN_R as OTGFSEN_R;
///Field `OTGFSEN` writer - USB OTG FS clock enable
pub use RNGEN_W as OTGFSEN_W;
impl R {
    ///Bit 6 - RNGEN
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - USB OTG FS clock enable
    #[inline(always)]
    pub fn otgfsen(&self) -> OTGFSEN_R {
        OTGFSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2ENR")
            .field("rngen", &self.rngen())
            .field("otgfsen", &self.otgfsen())
            .finish()
    }
}
impl W {
    ///Bit 6 - RNGEN
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W<'_, AHB2ENRrs> {
        RNGEN_W::new(self, 6)
    }
    ///Bit 7 - USB OTG FS clock enable
    #[inline(always)]
    pub fn otgfsen(&mut self) -> OTGFSEN_W<'_, AHB2ENRrs> {
        OTGFSEN_W::new(self, 7)
    }
}
/**AHB2 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F412.html#RCC:AHB2ENR)*/
pub struct AHB2ENRrs;
impl crate::RegisterSpec for AHB2ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb2enr::R`](R) reader structure
impl crate::Readable for AHB2ENRrs {}
///`write(|w| ..)` method takes [`ahb2enr::W`](W) writer structure
impl crate::Writable for AHB2ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2ENR to value 0
impl crate::Resettable for AHB2ENRrs {}
