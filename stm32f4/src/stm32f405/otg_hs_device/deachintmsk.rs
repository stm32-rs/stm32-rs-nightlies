#[doc = "Register `DEACHINTMSK` reader"]
pub type R = crate::R<DEACHINTMSKrs>;
#[doc = "Register `DEACHINTMSK` writer"]
pub type W = crate::W<DEACHINTMSKrs>;
#[doc = "Field `IEP1INTM` reader - IN Endpoint 1 interrupt mask bit"]
pub type IEP1INTM_R = crate::BitReader;
#[doc = "Field `IEP1INTM` writer - IN Endpoint 1 interrupt mask bit"]
pub type IEP1INTM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEP1INTM` reader - OUT Endpoint 1 interrupt mask bit"]
pub type OEP1INTM_R = crate::BitReader;
#[doc = "Field `OEP1INTM` writer - OUT Endpoint 1 interrupt mask bit"]
pub type OEP1INTM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - IN Endpoint 1 interrupt mask bit"]
    #[inline(always)]
    pub fn iep1intm(&self) -> IEP1INTM_R {
        IEP1INTM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 17 - OUT Endpoint 1 interrupt mask bit"]
    #[inline(always)]
    pub fn oep1intm(&self) -> OEP1INTM_R {
        OEP1INTM_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - IN Endpoint 1 interrupt mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn iep1intm(&mut self) -> IEP1INTM_W<DEACHINTMSKrs> {
        IEP1INTM_W::new(self, 1)
    }
    #[doc = "Bit 17 - OUT Endpoint 1 interrupt mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn oep1intm(&mut self) -> OEP1INTM_W<DEACHINTMSKrs> {
        OEP1INTM_W::new(self, 17)
    }
}
#[doc = "OTG_HS device each endpoint interrupt register mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deachintmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deachintmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEACHINTMSKrs;
impl crate::RegisterSpec for DEACHINTMSKrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`deachintmsk::R`](R) reader structure"]
impl crate::Readable for DEACHINTMSKrs {}
#[doc = "`write(|w| ..)` method takes [`deachintmsk::W`](W) writer structure"]
impl crate::Writable for DEACHINTMSKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEACHINTMSK to value 0"]
impl crate::Resettable for DEACHINTMSKrs {
    const RESET_VALUE: u32 = 0;
}
