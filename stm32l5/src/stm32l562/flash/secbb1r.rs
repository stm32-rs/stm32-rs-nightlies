#[doc = "Register `SECBB1R%s` reader"]
pub type R = crate::R<SECBB1Rrs>;
#[doc = "Register `SECBB1R%s` writer"]
pub type W = crate::W<SECBB1Rrs>;
#[doc = "Field `SECBB1` reader - SECBB1"]
pub type SECBB1_R = crate::FieldReader<u32>;
#[doc = "Field `SECBB1` writer - SECBB1"]
pub type SECBB1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SECBB1"]
    #[inline(always)]
    pub fn secbb1(&self) -> SECBB1_R {
        SECBB1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SECBB1"]
    #[inline(always)]
    #[must_use]
    pub fn secbb1(&mut self) -> SECBB1_W<SECBB1Rrs> {
        SECBB1_W::new(self, 0)
    }
}
#[doc = "FLASH secure block based bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secbb1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secbb1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECBB1Rrs;
impl crate::RegisterSpec for SECBB1Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secbb1r::R`](R) reader structure"]
impl crate::Readable for SECBB1Rrs {}
#[doc = "`write(|w| ..)` method takes [`secbb1r::W`](W) writer structure"]
impl crate::Writable for SECBB1Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECBB1R%s to value 0"]
impl crate::Resettable for SECBB1Rrs {
    const RESET_VALUE: u32 = 0;
}
