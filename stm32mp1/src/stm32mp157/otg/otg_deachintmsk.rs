#[doc = "Register `OTG_DEACHINTMSK` reader"]
pub type R = crate::R<OTG_DEACHINTMSKrs>;
#[doc = "Register `OTG_DEACHINTMSK` writer"]
pub type W = crate::W<OTG_DEACHINTMSKrs>;
#[doc = "Field `IEP1INTM` reader - IEP1INTM"]
pub type IEP1INTM_R = crate::BitReader;
#[doc = "Field `IEP1INTM` writer - IEP1INTM"]
pub type IEP1INTM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEP1INTM` reader - OEP1INTM"]
pub type OEP1INTM_R = crate::BitReader;
#[doc = "Field `OEP1INTM` writer - OEP1INTM"]
pub type OEP1INTM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - IEP1INTM"]
    #[inline(always)]
    pub fn iep1intm(&self) -> IEP1INTM_R {
        IEP1INTM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 17 - OEP1INTM"]
    #[inline(always)]
    pub fn oep1intm(&self) -> OEP1INTM_R {
        OEP1INTM_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - IEP1INTM"]
    #[inline(always)]
    #[must_use]
    pub fn iep1intm(&mut self) -> IEP1INTM_W<OTG_DEACHINTMSKrs> {
        IEP1INTM_W::new(self, 1)
    }
    #[doc = "Bit 17 - OEP1INTM"]
    #[inline(always)]
    #[must_use]
    pub fn oep1intm(&mut self) -> OEP1INTM_W<OTG_DEACHINTMSKrs> {
        OEP1INTM_W::new(self, 17)
    }
}
#[doc = "There is one interrupt bit for endpoint 1 IN and one interrupt bit for endpoint 1 OUT.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_deachintmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_deachintmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_DEACHINTMSKrs;
impl crate::RegisterSpec for OTG_DEACHINTMSKrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_deachintmsk::R`](R) reader structure"]
impl crate::Readable for OTG_DEACHINTMSKrs {}
#[doc = "`write(|w| ..)` method takes [`otg_deachintmsk::W`](W) writer structure"]
impl crate::Writable for OTG_DEACHINTMSKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_DEACHINTMSK to value 0"]
impl crate::Resettable for OTG_DEACHINTMSKrs {
    const RESET_VALUE: u32 = 0;
}
