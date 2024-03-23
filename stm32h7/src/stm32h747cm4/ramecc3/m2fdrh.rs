#[doc = "Register `M2FDRH` reader"]
pub type R = crate::R<M2FDRHrs>;
#[doc = "Register `M2FDRH` writer"]
pub type W = crate::W<M2FDRHrs>;
#[doc = "Field `FDATAH` reader - ECC failing data high"]
pub type FDATAH_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ECC failing data high"]
    #[inline(always)]
    pub fn fdatah(&self) -> FDATAH_R {
        FDATAH_R::new(self.bits)
    }
}
impl W {}
#[doc = "RAMECC monitor 2 failing data high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m2fdrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m2fdrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M2FDRHrs;
impl crate::RegisterSpec for M2FDRHrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m2fdrh::R`](R) reader structure"]
impl crate::Readable for M2FDRHrs {}
#[doc = "`write(|w| ..)` method takes [`m2fdrh::W`](W) writer structure"]
impl crate::Writable for M2FDRHrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M2FDRH to value 0"]
impl crate::Resettable for M2FDRHrs {
    const RESET_VALUE: u32 = 0;
}
