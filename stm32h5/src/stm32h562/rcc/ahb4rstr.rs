#[doc = "Register `AHB4RSTR` reader"]
pub type R = crate::R<AHB4RSTRrs>;
#[doc = "Register `AHB4RSTR` writer"]
pub type W = crate::W<AHB4RSTRrs>;
#[doc = "SDMMC1 and SDMMC1 delay blocks reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMMC1RST {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<SDMMC1RST> for bool {
    #[inline(always)]
    fn from(variant: SDMMC1RST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMMC1RST` reader - SDMMC1 and SDMMC1 delay blocks reset Set and reset by software."]
pub type SDMMC1RST_R = crate::BitReader<SDMMC1RST>;
impl SDMMC1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SDMMC1RST> {
        match self.bits {
            true => Some(SDMMC1RST::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SDMMC1RST::Reset
    }
}
#[doc = "Field `SDMMC1RST` writer - SDMMC1 and SDMMC1 delay blocks reset Set and reset by software."]
pub type SDMMC1RST_W<'a, REG> = crate::BitWriter<'a, REG, SDMMC1RST>;
impl<'a, REG> SDMMC1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMC1RST::Reset)
    }
}
#[doc = "Field `FMCRST` reader - FMC block reset Set and reset by software."]
pub use SDMMC1RST_R as FMCRST_R;
#[doc = "Field `OCTOSPI1RST` reader - OCTOSPI1 block reset Set and reset by software."]
pub use SDMMC1RST_R as OCTOSPI1RST_R;
#[doc = "Field `FMCRST` writer - FMC block reset Set and reset by software."]
pub use SDMMC1RST_W as FMCRST_W;
#[doc = "Field `OCTOSPI1RST` writer - OCTOSPI1 block reset Set and reset by software."]
pub use SDMMC1RST_W as OCTOSPI1RST_W;
impl R {
    #[doc = "Bit 11 - SDMMC1 and SDMMC1 delay blocks reset Set and reset by software."]
    #[inline(always)]
    pub fn sdmmc1rst(&self) -> SDMMC1RST_R {
        SDMMC1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - FMC block reset Set and reset by software."]
    #[inline(always)]
    pub fn fmcrst(&self) -> FMCRST_R {
        FMCRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - OCTOSPI1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn octospi1rst(&self) -> OCTOSPI1RST_R {
        OCTOSPI1RST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - SDMMC1 and SDMMC1 delay blocks reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1rst(&mut self) -> SDMMC1RST_W<AHB4RSTRrs> {
        SDMMC1RST_W::new(self, 11)
    }
    #[doc = "Bit 16 - FMC block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn fmcrst(&mut self) -> FMCRST_W<AHB4RSTRrs> {
        FMCRST_W::new(self, 16)
    }
    #[doc = "Bit 20 - OCTOSPI1 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn octospi1rst(&mut self) -> OCTOSPI1RST_W<AHB4RSTRrs> {
        OCTOSPI1RST_W::new(self, 20)
    }
}
#[doc = "RCC AHB4 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb4rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb4rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB4RSTRrs;
impl crate::RegisterSpec for AHB4RSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb4rstr::R`](R) reader structure"]
impl crate::Readable for AHB4RSTRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb4rstr::W`](W) writer structure"]
impl crate::Writable for AHB4RSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB4RSTR to value 0"]
impl crate::Resettable for AHB4RSTRrs {
    const RESET_VALUE: u32 = 0;
}
