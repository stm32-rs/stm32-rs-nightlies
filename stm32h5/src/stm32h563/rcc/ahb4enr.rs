///Register `AHB4ENR` reader
pub type R = crate::R<AHB4ENRrs>;
///Register `AHB4ENR` writer
pub type W = crate::W<AHB4ENRrs>;
/**SDMMC1 and SDMMC1 delay peripheral clock enable reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMMC1EN {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<SDMMC1EN> for bool {
    #[inline(always)]
    fn from(variant: SDMMC1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `SDMMC1EN` reader - SDMMC1 and SDMMC1 delay peripheral clock enable reset
pub type SDMMC1EN_R = crate::BitReader<SDMMC1EN>;
impl SDMMC1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDMMC1EN {
        match self.bits {
            false => SDMMC1EN::Disabled,
            true => SDMMC1EN::Enabled,
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SDMMC1EN::Disabled
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SDMMC1EN::Enabled
    }
}
///Field `SDMMC1EN` writer - SDMMC1 and SDMMC1 delay peripheral clock enable reset
pub type SDMMC1EN_W<'a, REG> = crate::BitWriter<'a, REG, SDMMC1EN>;
impl<'a, REG> SDMMC1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMC1EN::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMC1EN::Enabled)
    }
}
///Field `SDMMC2EN` reader - SDMMC2 and SDMMC2 delay peripheral clock enabled Set and reset by software.
pub use SDMMC1EN_R as SDMMC2EN_R;
///Field `FMCEN` reader - FMC clock enable Set and reset by software.
pub use SDMMC1EN_R as FMCEN_R;
///Field `OCTOSPI1EN` reader - OCTOSPI1 clock enable Set and reset by software.
pub use SDMMC1EN_R as OCTOSPI1EN_R;
///Field `SDMMC2EN` writer - SDMMC2 and SDMMC2 delay peripheral clock enabled Set and reset by software.
pub use SDMMC1EN_W as SDMMC2EN_W;
///Field `FMCEN` writer - FMC clock enable Set and reset by software.
pub use SDMMC1EN_W as FMCEN_W;
///Field `OCTOSPI1EN` writer - OCTOSPI1 clock enable Set and reset by software.
pub use SDMMC1EN_W as OCTOSPI1EN_W;
impl R {
    ///Bit 11 - SDMMC1 and SDMMC1 delay peripheral clock enable reset
    #[inline(always)]
    pub fn sdmmc1en(&self) -> SDMMC1EN_R {
        SDMMC1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SDMMC2 and SDMMC2 delay peripheral clock enabled Set and reset by software.
    #[inline(always)]
    pub fn sdmmc2en(&self) -> SDMMC2EN_R {
        SDMMC2EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - FMC clock enable Set and reset by software.
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - OCTOSPI1 clock enable Set and reset by software.
    #[inline(always)]
    pub fn octospi1en(&self) -> OCTOSPI1EN_R {
        OCTOSPI1EN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB4ENR")
            .field("sdmmc1en", &self.sdmmc1en())
            .field("sdmmc2en", &self.sdmmc2en())
            .field("fmcen", &self.fmcen())
            .field("octospi1en", &self.octospi1en())
            .finish()
    }
}
impl W {
    ///Bit 11 - SDMMC1 and SDMMC1 delay peripheral clock enable reset
    #[inline(always)]
    pub fn sdmmc1en(&mut self) -> SDMMC1EN_W<'_, AHB4ENRrs> {
        SDMMC1EN_W::new(self, 11)
    }
    ///Bit 12 - SDMMC2 and SDMMC2 delay peripheral clock enabled Set and reset by software.
    #[inline(always)]
    pub fn sdmmc2en(&mut self) -> SDMMC2EN_W<'_, AHB4ENRrs> {
        SDMMC2EN_W::new(self, 12)
    }
    ///Bit 16 - FMC clock enable Set and reset by software.
    #[inline(always)]
    pub fn fmcen(&mut self) -> FMCEN_W<'_, AHB4ENRrs> {
        FMCEN_W::new(self, 16)
    }
    ///Bit 20 - OCTOSPI1 clock enable Set and reset by software.
    #[inline(always)]
    pub fn octospi1en(&mut self) -> OCTOSPI1EN_W<'_, AHB4ENRrs> {
        OCTOSPI1EN_W::new(self, 20)
    }
}
/**RCC AHB4 peripheral clock register

You can [`read`](crate::Reg::read) this register and get [`ahb4enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#RCC:AHB4ENR)*/
pub struct AHB4ENRrs;
impl crate::RegisterSpec for AHB4ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb4enr::R`](R) reader structure
impl crate::Readable for AHB4ENRrs {}
///`write(|w| ..)` method takes [`ahb4enr::W`](W) writer structure
impl crate::Writable for AHB4ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB4ENR to value 0
impl crate::Resettable for AHB4ENRrs {}
