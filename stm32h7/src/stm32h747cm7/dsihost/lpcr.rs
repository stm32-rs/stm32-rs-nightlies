#[doc = "Register `LPCR` reader"]
pub type R = crate::R<LPCRrs>;
#[doc = "Register `LPCR` writer"]
pub type W = crate::W<LPCRrs>;
#[doc = "Field `DEP` reader - Data enable polarity"]
pub type DEP_R = crate::BitReader;
#[doc = "Field `DEP` writer - Data enable polarity"]
pub type DEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSP` reader - VSYNC polarity"]
pub type VSP_R = crate::BitReader;
#[doc = "Field `VSP` writer - VSYNC polarity"]
pub type VSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSP` reader - HSYNC polarity"]
pub type HSP_R = crate::BitReader;
#[doc = "Field `HSP` writer - HSYNC polarity"]
pub type HSP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Data enable polarity"]
    #[inline(always)]
    pub fn dep(&self) -> DEP_R {
        DEP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VSYNC polarity"]
    #[inline(always)]
    pub fn vsp(&self) -> VSP_R {
        VSP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSYNC polarity"]
    #[inline(always)]
    pub fn hsp(&self) -> HSP_R {
        HSP_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data enable polarity"]
    #[inline(always)]
    #[must_use]
    pub fn dep(&mut self) -> DEP_W<LPCRrs> {
        DEP_W::new(self, 0)
    }
    #[doc = "Bit 1 - VSYNC polarity"]
    #[inline(always)]
    #[must_use]
    pub fn vsp(&mut self) -> VSP_W<LPCRrs> {
        VSP_W::new(self, 1)
    }
    #[doc = "Bit 2 - HSYNC polarity"]
    #[inline(always)]
    #[must_use]
    pub fn hsp(&mut self) -> HSP_W<LPCRrs> {
        HSP_W::new(self, 2)
    }
}
#[doc = "DSI Host LTDC polarity configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPCRrs;
impl crate::RegisterSpec for LPCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpcr::R`](R) reader structure"]
impl crate::Readable for LPCRrs {}
#[doc = "`write(|w| ..)` method takes [`lpcr::W`](W) writer structure"]
impl crate::Writable for LPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPCR to value 0"]
impl crate::Resettable for LPCRrs {
    const RESET_VALUE: u32 = 0;
}
