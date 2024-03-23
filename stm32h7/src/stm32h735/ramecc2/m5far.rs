#[doc = "Register `M5FAR` reader"]
pub type R = crate::R<M5FARrs>;
#[doc = "Register `M5FAR` writer"]
pub type W = crate::W<M5FARrs>;
#[doc = "Field `FADD` reader - ECC error failing address"]
pub type FADD_R = crate::FieldReader<u32>;
#[doc = "Field `FADD` writer - ECC error failing address"]
pub type FADD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ECC error failing address"]
    #[inline(always)]
    pub fn fadd(&self) -> FADD_R {
        FADD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ECC error failing address"]
    #[inline(always)]
    #[must_use]
    pub fn fadd(&mut self) -> FADD_W<M5FARrs> {
        FADD_W::new(self, 0)
    }
}
#[doc = "RAMECC monitor x failing address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5far::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m5far::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M5FARrs;
impl crate::RegisterSpec for M5FARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m5far::R`](R) reader structure"]
impl crate::Readable for M5FARrs {}
#[doc = "`write(|w| ..)` method takes [`m5far::W`](W) writer structure"]
impl crate::Writable for M5FARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M5FAR to value 0"]
impl crate::Resettable for M5FARrs {
    const RESET_VALUE: u32 = 0;
}
