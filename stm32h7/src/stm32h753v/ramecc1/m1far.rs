#[doc = "Register `M1FAR` reader"]
pub type R = crate::R<M1FARrs>;
#[doc = "Register `M1FAR` writer"]
pub type W = crate::W<M1FARrs>;
#[doc = "Field `FADD` reader - ECC failing address"]
pub type FADD_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ECC failing address"]
    #[inline(always)]
    pub fn fadd(&self) -> FADD_R {
        FADD_R::new(self.bits)
    }
}
impl W {}
#[doc = "RAMECC monitor 1 failing address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m1far::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m1far::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M1FARrs;
impl crate::RegisterSpec for M1FARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m1far::R`](R) reader structure"]
impl crate::Readable for M1FARrs {}
#[doc = "`write(|w| ..)` method takes [`m1far::W`](W) writer structure"]
impl crate::Writable for M1FARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M1FAR to value 0"]
impl crate::Resettable for M1FARrs {
    const RESET_VALUE: u32 = 0;
}
