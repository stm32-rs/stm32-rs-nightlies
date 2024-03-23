#[doc = "Register `CMPCR` reader"]
pub type R = crate::R<CMPCRrs>;
#[doc = "Register `CMPCR` writer"]
pub type W = crate::W<CMPCRrs>;
#[doc = "Field `CMP_PD` reader - Compensation cell power-down"]
pub type CMP_PD_R = crate::BitReader;
#[doc = "Field `CMP_PD` writer - Compensation cell power-down"]
pub type CMP_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READY` reader - READY"]
pub type READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Compensation cell power-down"]
    #[inline(always)]
    pub fn cmp_pd(&self) -> CMP_PD_R {
        CMP_PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - READY"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compensation cell power-down"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_pd(&mut self) -> CMP_PD_W<CMPCRrs> {
        CMP_PD_W::new(self, 0)
    }
}
#[doc = "Compensation cell control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMPCRrs;
impl crate::RegisterSpec for CMPCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpcr::R`](R) reader structure"]
impl crate::Readable for CMPCRrs {}
#[doc = "`write(|w| ..)` method takes [`cmpcr::W`](W) writer structure"]
impl crate::Writable for CMPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMPCR to value 0"]
impl crate::Resettable for CMPCRrs {
    const RESET_VALUE: u32 = 0;
}
