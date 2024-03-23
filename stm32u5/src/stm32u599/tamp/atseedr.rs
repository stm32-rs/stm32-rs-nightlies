#[doc = "Register `ATSEEDR` reader"]
pub type R = crate::R<ATSEEDRrs>;
#[doc = "Register `ATSEEDR` writer"]
pub type W = crate::W<ATSEEDRrs>;
#[doc = "Field `SEED` reader - SEED"]
pub type SEED_R = crate::FieldReader<u32>;
#[doc = "Field `SEED` writer - SEED"]
pub type SEED_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SEED"]
    #[inline(always)]
    pub fn seed(&self) -> SEED_R {
        SEED_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SEED"]
    #[inline(always)]
    #[must_use]
    pub fn seed(&mut self) -> SEED_W<ATSEEDRrs> {
        SEED_W::new(self, 0)
    }
}
#[doc = "TAMP active tamper seed register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atseedr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atseedr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ATSEEDRrs;
impl crate::RegisterSpec for ATSEEDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atseedr::R`](R) reader structure"]
impl crate::Readable for ATSEEDRrs {}
#[doc = "`write(|w| ..)` method takes [`atseedr::W`](W) writer structure"]
impl crate::Writable for ATSEEDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATSEEDR to value 0"]
impl crate::Resettable for ATSEEDRrs {
    const RESET_VALUE: u32 = 0;
}
