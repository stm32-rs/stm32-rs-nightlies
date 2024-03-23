#[doc = "Register `SUSP3R` reader"]
pub type R = crate::R<SUSP3Rrs>;
#[doc = "Register `SUSP3R` writer"]
pub type W = crate::W<SUSP3Rrs>;
#[doc = "Field `SUSP` reader - AES suspend register 3"]
pub type SUSP_R = crate::FieldReader<u32>;
#[doc = "Field `SUSP` writer - AES suspend register 3"]
pub type SUSP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AES suspend register 3"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES suspend register 3"]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<SUSP3Rrs> {
        SUSP_W::new(self, 0)
    }
}
#[doc = "AES suspend register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp3r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp3r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SUSP3Rrs;
impl crate::RegisterSpec for SUSP3Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`susp3r::R`](R) reader structure"]
impl crate::Readable for SUSP3Rrs {}
#[doc = "`write(|w| ..)` method takes [`susp3r::W`](W) writer structure"]
impl crate::Writable for SUSP3Rrs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUSP3R to value 0"]
impl crate::Resettable for SUSP3Rrs {
    const RESET_VALUE: u32 = 0;
}
