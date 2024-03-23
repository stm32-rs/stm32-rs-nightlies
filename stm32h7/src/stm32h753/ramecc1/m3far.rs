#[doc = "Register `M3FAR` reader"]
pub type R = crate::R<M3FARrs>;
#[doc = "Register `M3FAR` writer"]
pub type W = crate::W<M3FARrs>;
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
#[doc = "RAMECC monitor 3 failing address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3far::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m3far::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M3FARrs;
impl crate::RegisterSpec for M3FARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m3far::R`](R) reader structure"]
impl crate::Readable for M3FARrs {}
#[doc = "`write(|w| ..)` method takes [`m3far::W`](W) writer structure"]
impl crate::Writable for M3FARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M3FAR to value 0"]
impl crate::Resettable for M3FARrs {
    const RESET_VALUE: u32 = 0;
}
