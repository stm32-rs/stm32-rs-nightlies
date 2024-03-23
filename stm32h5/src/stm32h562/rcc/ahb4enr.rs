#[doc = "Register `AHB4ENR` reader"]
pub type R = crate::R<AHB4ENRrs>;
#[doc = "Register `AHB4ENR` writer"]
pub type W = crate::W<AHB4ENRrs>;
#[doc = "SDMMC1 and SDMMC1 delay peripheral clock enable reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMMC1EN {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<SDMMC1EN> for bool {
    #[inline(always)]
    fn from(variant: SDMMC1EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMMC1EN` reader - SDMMC1 and SDMMC1 delay peripheral clock enable reset"]
pub type SDMMC1EN_R = crate::BitReader<SDMMC1EN>;
impl SDMMC1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDMMC1EN {
        match self.bits {
            false => SDMMC1EN::Disabled,
            true => SDMMC1EN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SDMMC1EN::Disabled
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SDMMC1EN::Enabled
    }
}
#[doc = "Field `SDMMC1EN` writer - SDMMC1 and SDMMC1 delay peripheral clock enable reset"]
pub type SDMMC1EN_W<'a, REG> = crate::BitWriter<'a, REG, SDMMC1EN>;
impl<'a, REG> SDMMC1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMC1EN::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMC1EN::Enabled)
    }
}
#[doc = "Field `FMCEN` reader - FMC clock enable Set and reset by software."]
pub use SDMMC1EN_R as FMCEN_R;
#[doc = "Field `OCTOSPI1EN` reader - OCTOSPI1 clock enable Set and reset by software."]
pub use SDMMC1EN_R as OCTOSPI1EN_R;
#[doc = "Field `FMCEN` writer - FMC clock enable Set and reset by software."]
pub use SDMMC1EN_W as FMCEN_W;
#[doc = "Field `OCTOSPI1EN` writer - OCTOSPI1 clock enable Set and reset by software."]
pub use SDMMC1EN_W as OCTOSPI1EN_W;
impl R {
    #[doc = "Bit 11 - SDMMC1 and SDMMC1 delay peripheral clock enable reset"]
    #[inline(always)]
    pub fn sdmmc1en(&self) -> SDMMC1EN_R {
        SDMMC1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - FMC clock enable Set and reset by software."]
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - OCTOSPI1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn octospi1en(&self) -> OCTOSPI1EN_R {
        OCTOSPI1EN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - SDMMC1 and SDMMC1 delay peripheral clock enable reset"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1en(&mut self) -> SDMMC1EN_W<AHB4ENRrs> {
        SDMMC1EN_W::new(self, 11)
    }
    #[doc = "Bit 16 - FMC clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn fmcen(&mut self) -> FMCEN_W<AHB4ENRrs> {
        FMCEN_W::new(self, 16)
    }
    #[doc = "Bit 20 - OCTOSPI1 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn octospi1en(&mut self) -> OCTOSPI1EN_W<AHB4ENRrs> {
        OCTOSPI1EN_W::new(self, 20)
    }
}
#[doc = "RCC AHB4 peripheral clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb4enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb4enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB4ENRrs;
impl crate::RegisterSpec for AHB4ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb4enr::R`](R) reader structure"]
impl crate::Readable for AHB4ENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb4enr::W`](W) writer structure"]
impl crate::Writable for AHB4ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB4ENR to value 0"]
impl crate::Resettable for AHB4ENRrs {
    const RESET_VALUE: u32 = 0;
}
