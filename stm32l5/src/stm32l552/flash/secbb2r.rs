#[doc = "Register `SECBB2R%s` reader"]
pub type R = crate::R<SECBB2Rrs>;
#[doc = "Register `SECBB2R%s` writer"]
pub type W = crate::W<SECBB2Rrs>;
#[doc = "Field `SECBB2` reader - SECBB2"]
pub type SECBB2_R = crate::FieldReader<u32>;
#[doc = "Field `SECBB2` writer - SECBB2"]
pub type SECBB2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SECBB2"]
    #[inline(always)]
    pub fn secbb2(&self) -> SECBB2_R {
        SECBB2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SECBB2"]
    #[inline(always)]
    #[must_use]
    pub fn secbb2(&mut self) -> SECBB2_W<SECBB2Rrs> {
        SECBB2_W::new(self, 0)
    }
}
#[doc = "FLASH secure block based bank 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secbb2r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secbb2r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECBB2Rrs;
impl crate::RegisterSpec for SECBB2Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secbb2r::R`](R) reader structure"]
impl crate::Readable for SECBB2Rrs {}
#[doc = "`write(|w| ..)` method takes [`secbb2r::W`](W) writer structure"]
impl crate::Writable for SECBB2Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECBB2R%s to value 0"]
impl crate::Resettable for SECBB2Rrs {
    const RESET_VALUE: u32 = 0;
}
