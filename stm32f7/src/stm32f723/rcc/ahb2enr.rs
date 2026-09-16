///Register `AHB2ENR` reader
pub type R = crate::R<AHB2ENRrs>;
///Register `AHB2ENR` writer
pub type W = crate::W<AHB2ENRrs>;
/**AES module clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AESEN {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<AESEN> for bool {
    #[inline(always)]
    fn from(variant: AESEN) -> Self {
        variant as u8 != 0
    }
}
///Field `AESEN` reader - AES module clock enable
pub type AESEN_R = crate::BitReader<AESEN>;
impl AESEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AESEN {
        match self.bits {
            false => AESEN::Disabled,
            true => AESEN::Enabled,
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AESEN::Disabled
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AESEN::Enabled
    }
}
///Field `AESEN` writer - AES module clock enable
pub type AESEN_W<'a, REG> = crate::BitWriter<'a, REG, AESEN>;
impl<'a, REG> AESEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AESEN::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AESEN::Enabled)
    }
}
///Field `RNGEN` reader - Random number generator clock enable
pub use AESEN_R as RNGEN_R;
///Field `OTGFSEN` reader - USB OTG FS clock enable
pub use AESEN_R as OTGFSEN_R;
///Field `RNGEN` writer - Random number generator clock enable
pub use AESEN_W as RNGEN_W;
///Field `OTGFSEN` writer - USB OTG FS clock enable
pub use AESEN_W as OTGFSEN_W;
impl R {
    ///Bit 4 - AES module clock enable
    #[inline(always)]
    pub fn aesen(&self) -> AESEN_R {
        AESEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Random number generator clock enable
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
            .field("aesen", &self.aesen())
            .field("otgfsen", &self.otgfsen())
            .field("rngen", &self.rngen())
            .finish()
    }
}
impl W {
    ///Bit 4 - AES module clock enable
    #[inline(always)]
    pub fn aesen(&mut self) -> AESEN_W<'_, AHB2ENRrs> {
        AESEN_W::new(self, 4)
    }
    ///Bit 6 - Random number generator clock enable
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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F723.html#RCC:AHB2ENR)*/
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
