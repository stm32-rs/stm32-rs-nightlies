#[doc = "Register `M3FDRL` reader"]
pub type R = crate::R<M3FDRLrs>;
#[doc = "Register `M3FDRL` writer"]
pub type W = crate::W<M3FDRLrs>;
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
#[doc = "RAMECC monitor 3 failing data low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3fdrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m3fdrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M3FDRLrs;
impl crate::RegisterSpec for M3FDRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m3fdrl::R`](R) reader structure"]
impl crate::Readable for M3FDRLrs {}
#[doc = "`write(|w| ..)` method takes [`m3fdrl::W`](W) writer structure"]
impl crate::Writable for M3FDRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M3FDRL to value 0"]
impl crate::Resettable for M3FDRLrs {
    const RESET_VALUE: u32 = 0;
}
