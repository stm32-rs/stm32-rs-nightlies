#[doc = "Register `PCSEL` reader"]
pub type R = crate::R<PCSELrs>;
#[doc = "Register `PCSEL` writer"]
pub type W = crate::W<PCSELrs>;
#[doc = "Channel x (VINP\\[i\\]) pre selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum PCSEL {
    #[doc = "0: Input channel x is not pre-selected"]
    NotPreselected = 0,
    #[doc = "1: Pre-select input channel x"]
    Preselected = 1,
}
impl From<PCSEL> for u32 {
    #[inline(always)]
    fn from(variant: PCSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCSEL {
    type Ux = u32;
}
#[doc = "Field `PCSEL` reader - Channel x (VINP\\[i\\]) pre selection"]
pub type PCSEL_R = crate::FieldReader<PCSEL>;
impl PCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PCSEL> {
        match self.bits {
            0 => Some(PCSEL::NotPreselected),
            1 => Some(PCSEL::Preselected),
            _ => None,
        }
    }
    #[doc = "Input channel x is not pre-selected"]
    #[inline(always)]
    pub fn is_not_preselected(&self) -> bool {
        *self == PCSEL::NotPreselected
    }
    #[doc = "Pre-select input channel x"]
    #[inline(always)]
    pub fn is_preselected(&self) -> bool {
        *self == PCSEL::Preselected
    }
}
#[doc = "Field `PCSEL` writer - Channel x (VINP\\[i\\]) pre selection"]
pub type PCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 20, PCSEL>;
impl<'a, REG> PCSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Input channel x is not pre-selected"]
    #[inline(always)]
    pub fn not_preselected(self) -> &'a mut crate::W<REG> {
        self.variant(PCSEL::NotPreselected)
    }
    #[doc = "Pre-select input channel x"]
    #[inline(always)]
    pub fn preselected(self) -> &'a mut crate::W<REG> {
        self.variant(PCSEL::Preselected)
    }
}
impl R {
    #[doc = "Bits 0:19 - Channel x (VINP\\[i\\]) pre selection"]
    #[inline(always)]
    pub fn pcsel(&self) -> PCSEL_R {
        PCSEL_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Channel x (VINP\\[i\\]) pre selection"]
    #[inline(always)]
    #[must_use]
    pub fn pcsel(&mut self) -> PCSEL_W<PCSELrs> {
        PCSEL_W::new(self, 0)
    }
}
#[doc = "ADC pre channel selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCSELrs;
impl crate::RegisterSpec for PCSELrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcsel::R`](R) reader structure"]
impl crate::Readable for PCSELrs {}
#[doc = "`write(|w| ..)` method takes [`pcsel::W`](W) writer structure"]
impl crate::Writable for PCSELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCSEL to value 0"]
impl crate::Resettable for PCSELrs {
    const RESET_VALUE: u32 = 0;
}
