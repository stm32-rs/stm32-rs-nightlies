#[doc = "Register `DMACCARxDR` reader"]
pub type R = crate::R<DMACCARX_DRrs>;
#[doc = "Register `DMACCARxDR` writer"]
pub type W = crate::W<DMACCARX_DRrs>;
#[doc = "Field `CURRDESAPTR` reader - Application Receive Descriptor Address Pointer"]
pub type CURRDESAPTR_R = crate::FieldReader<u32>;
#[doc = "Field `CURRDESAPTR` writer - Application Receive Descriptor Address Pointer"]
pub type CURRDESAPTR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Application Receive Descriptor Address Pointer"]
    #[inline(always)]
    pub fn currdesaptr(&self) -> CURRDESAPTR_R {
        CURRDESAPTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Application Receive Descriptor Address Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn currdesaptr(&mut self) -> CURRDESAPTR_W<DMACCARX_DRrs> {
        CURRDESAPTR_W::new(self, 0)
    }
}
#[doc = "Channel current application receive descriptor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaccarx_dr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaccarx_dr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACCARX_DRrs;
impl crate::RegisterSpec for DMACCARX_DRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaccarx_dr::R`](R) reader structure"]
impl crate::Readable for DMACCARX_DRrs {}
#[doc = "`write(|w| ..)` method takes [`dmaccarx_dr::W`](W) writer structure"]
impl crate::Writable for DMACCARX_DRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACCARxDR to value 0"]
impl crate::Resettable for DMACCARX_DRrs {
    const RESET_VALUE: u32 = 0;
}
