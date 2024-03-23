#[doc = "Register `M4FAR` reader"]
pub type R = crate::R<M4FARrs>;
#[doc = "Register `M4FAR` writer"]
pub type W = crate::W<M4FARrs>;
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
#[doc = "RAMECC monitor 4 failing address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m4far::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m4far::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M4FARrs;
impl crate::RegisterSpec for M4FARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m4far::R`](R) reader structure"]
impl crate::Readable for M4FARrs {}
#[doc = "`write(|w| ..)` method takes [`m4far::W`](W) writer structure"]
impl crate::Writable for M4FARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M4FAR to value 0"]
impl crate::Resettable for M4FARrs {
    const RESET_VALUE: u32 = 0;
}
