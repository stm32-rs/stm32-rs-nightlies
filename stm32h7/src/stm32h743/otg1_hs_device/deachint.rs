#[doc = "Register `DEACHINT` reader"]
pub type R = crate::R<DEACHINTrs>;
#[doc = "Register `DEACHINT` writer"]
pub type W = crate::W<DEACHINTrs>;
#[doc = "Field `IEP1INT` reader - IN endpoint 1interrupt bit"]
pub type IEP1INT_R = crate::BitReader;
#[doc = "Field `IEP1INT` writer - IN endpoint 1interrupt bit"]
pub type IEP1INT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEP1INT` reader - OUT endpoint 1 interrupt bit"]
pub type OEP1INT_R = crate::BitReader;
#[doc = "Field `OEP1INT` writer - OUT endpoint 1 interrupt bit"]
pub type OEP1INT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - IN endpoint 1interrupt bit"]
    #[inline(always)]
    pub fn iep1int(&self) -> IEP1INT_R {
        IEP1INT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 17 - OUT endpoint 1 interrupt bit"]
    #[inline(always)]
    pub fn oep1int(&self) -> OEP1INT_R {
        OEP1INT_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - IN endpoint 1interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn iep1int(&mut self) -> IEP1INT_W<DEACHINTrs> {
        IEP1INT_W::new(self, 1)
    }
    #[doc = "Bit 17 - OUT endpoint 1 interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn oep1int(&mut self) -> OEP1INT_W<DEACHINTrs> {
        OEP1INT_W::new(self, 17)
    }
}
#[doc = "OTG_HS device each endpoint interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deachint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deachint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEACHINTrs;
impl crate::RegisterSpec for DEACHINTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`deachint::R`](R) reader structure"]
impl crate::Readable for DEACHINTrs {}
#[doc = "`write(|w| ..)` method takes [`deachint::W`](W) writer structure"]
impl crate::Writable for DEACHINTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEACHINT to value 0"]
impl crate::Resettable for DEACHINTrs {
    const RESET_VALUE: u32 = 0;
}
