#[doc = "Register `SUSP7R` reader"]
pub type R = crate::R<SUSP7Rrs>;
#[doc = "Register `SUSP7R` writer"]
pub type W = crate::W<SUSP7Rrs>;
#[doc = "Field `SUSP` reader - AES suspend register 7"]
pub type SUSP_R = crate::FieldReader<u32>;
#[doc = "Field `SUSP` writer - AES suspend register 7"]
pub type SUSP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AES suspend register 7"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES suspend register 7"]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<SUSP7Rrs> {
        SUSP_W::new(self, 0)
    }
}
#[doc = "AES suspend register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp7r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp7r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SUSP7Rrs;
impl crate::RegisterSpec for SUSP7Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`susp7r::R`](R) reader structure"]
impl crate::Readable for SUSP7Rrs {}
#[doc = "`write(|w| ..)` method takes [`susp7r::W`](W) writer structure"]
impl crate::Writable for SUSP7Rrs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUSP7R to value 0"]
impl crate::Resettable for SUSP7Rrs {
    const RESET_VALUE: u32 = 0;
}
