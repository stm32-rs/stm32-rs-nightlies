#[doc = "Register `M1FDRL` reader"]
pub type R = crate::R<M1FDRLrs>;
#[doc = "Register `M1FDRL` writer"]
pub type W = crate::W<M1FDRLrs>;
#[doc = "Field `FDATAL` reader - ECC failing data low"]
pub type FDATAL_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ECC failing data low"]
    #[inline(always)]
    pub fn fdatal(&self) -> FDATAL_R {
        FDATAL_R::new(self.bits)
    }
}
impl W {}
#[doc = "RAMECC monitor 1 failing data low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m1fdrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m1fdrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M1FDRLrs;
impl crate::RegisterSpec for M1FDRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m1fdrl::R`](R) reader structure"]
impl crate::Readable for M1FDRLrs {}
#[doc = "`write(|w| ..)` method takes [`m1fdrl::W`](W) writer structure"]
impl crate::Writable for M1FDRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M1FDRL to value 0"]
impl crate::Resettable for M1FDRLrs {
    const RESET_VALUE: u32 = 0;
}
