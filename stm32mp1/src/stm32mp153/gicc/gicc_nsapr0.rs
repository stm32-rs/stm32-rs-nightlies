#[doc = "Register `GICC_NSAPR0` reader"]
pub type R = crate::R<GICC_NSAPR0rs>;
#[doc = "Register `GICC_NSAPR0` writer"]
pub type W = crate::W<GICC_NSAPR0rs>;
#[doc = "Field `NSAPR0` reader - NSAPR0"]
pub type NSAPR0_R = crate::FieldReader<u32>;
#[doc = "Field `NSAPR0` writer - NSAPR0"]
pub type NSAPR0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - NSAPR0"]
    #[inline(always)]
    pub fn nsapr0(&self) -> NSAPR0_R {
        NSAPR0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - NSAPR0"]
    #[inline(always)]
    #[must_use]
    pub fn nsapr0(&mut self) -> NSAPR0_W<GICC_NSAPR0rs> {
        NSAPR0_W::new(self, 0)
    }
}
#[doc = "GICC non-secure active priority register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicc_nsapr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicc_nsapr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICC_NSAPR0rs;
impl crate::RegisterSpec for GICC_NSAPR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicc_nsapr0::R`](R) reader structure"]
impl crate::Readable for GICC_NSAPR0rs {}
#[doc = "`write(|w| ..)` method takes [`gicc_nsapr0::W`](W) writer structure"]
impl crate::Writable for GICC_NSAPR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICC_NSAPR0 to value 0"]
impl crate::Resettable for GICC_NSAPR0rs {
    const RESET_VALUE: u32 = 0;
}
