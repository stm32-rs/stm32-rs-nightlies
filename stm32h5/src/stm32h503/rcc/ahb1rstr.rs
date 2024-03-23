#[doc = "Register `AHB1RSTR` reader"]
pub type R = crate::R<AHB1RSTRrs>;
#[doc = "Register `AHB1RSTR` writer"]
pub type W = crate::W<AHB1RSTRrs>;
#[doc = "GPDMA1 block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPDMA1RST {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<GPDMA1RST> for bool {
    #[inline(always)]
    fn from(variant: GPDMA1RST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPDMA1RST` reader - GPDMA1 block reset Set and reset by software."]
pub type GPDMA1RST_R = crate::BitReader<GPDMA1RST>;
impl GPDMA1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GPDMA1RST> {
        match self.bits {
            true => Some(GPDMA1RST::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == GPDMA1RST::Reset
    }
}
#[doc = "Field `GPDMA1RST` writer - GPDMA1 block reset Set and reset by software."]
pub type GPDMA1RST_W<'a, REG> = crate::BitWriter<'a, REG, GPDMA1RST>;
impl<'a, REG> GPDMA1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(GPDMA1RST::Reset)
    }
}
#[doc = "Field `GPDMA2RST` reader - GPDMA2 block reset Set and reset by software."]
pub use GPDMA1RST_R as GPDMA2RST_R;
#[doc = "Field `CRCRST` reader - CRC block reset Set and reset by software."]
pub use GPDMA1RST_R as CRCRST_R;
#[doc = "Field `RAMCFGRST` reader - RAMCFG block reset Set and reset by software."]
pub use GPDMA1RST_R as RAMCFGRST_R;
#[doc = "Field `GPDMA2RST` writer - GPDMA2 block reset Set and reset by software."]
pub use GPDMA1RST_W as GPDMA2RST_W;
#[doc = "Field `CRCRST` writer - CRC block reset Set and reset by software."]
pub use GPDMA1RST_W as CRCRST_W;
#[doc = "Field `RAMCFGRST` writer - RAMCFG block reset Set and reset by software."]
pub use GPDMA1RST_W as RAMCFGRST_W;
impl R {
    #[doc = "Bit 0 - GPDMA1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpdma1rst(&self) -> GPDMA1RST_R {
        GPDMA1RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPDMA2 block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpdma2rst(&self) -> GPDMA2RST_R {
        GPDMA2RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC block reset Set and reset by software."]
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 17 - RAMCFG block reset Set and reset by software."]
    #[inline(always)]
    pub fn ramcfgrst(&self) -> RAMCFGRST_R {
        RAMCFGRST_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPDMA1 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpdma1rst(&mut self) -> GPDMA1RST_W<AHB1RSTRrs> {
        GPDMA1RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - GPDMA2 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpdma2rst(&mut self) -> GPDMA2RST_W<AHB1RSTRrs> {
        GPDMA2RST_W::new(self, 1)
    }
    #[doc = "Bit 12 - CRC block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn crcrst(&mut self) -> CRCRST_W<AHB1RSTRrs> {
        CRCRST_W::new(self, 12)
    }
    #[doc = "Bit 17 - RAMCFG block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn ramcfgrst(&mut self) -> RAMCFGRST_W<AHB1RSTRrs> {
        RAMCFGRST_W::new(self, 17)
    }
}
#[doc = "RCC AHB1 reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB1RSTRrs;
impl crate::RegisterSpec for AHB1RSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1rstr::R`](R) reader structure"]
impl crate::Readable for AHB1RSTRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb1rstr::W`](W) writer structure"]
impl crate::Writable for AHB1RSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB1RSTR to value 0"]
impl crate::Resettable for AHB1RSTRrs {
    const RESET_VALUE: u32 = 0;
}
