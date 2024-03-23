#[doc = "Register `SUSP4R` reader"]
pub type R = crate::R<SUSP4Rrs>;
#[doc = "Register `SUSP4R` writer"]
pub type W = crate::W<SUSP4Rrs>;
#[doc = "Field `SUSP` reader - AES suspend Upon suspend operation, this bitfield of the corresponding AES_SUSPxR register takes the value of one of internal AES registers."]
pub type SUSP_R = crate::FieldReader<u32>;
#[doc = "Field `SUSP` writer - AES suspend Upon suspend operation, this bitfield of the corresponding AES_SUSPxR register takes the value of one of internal AES registers."]
pub type SUSP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AES suspend Upon suspend operation, this bitfield of the corresponding AES_SUSPxR register takes the value of one of internal AES registers."]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES suspend Upon suspend operation, this bitfield of the corresponding AES_SUSPxR register takes the value of one of internal AES registers."]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<SUSP4Rrs> {
        SUSP_W::new(self, 0)
    }
}
#[doc = "AES suspend registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp4r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp4r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SUSP4Rrs;
impl crate::RegisterSpec for SUSP4Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`susp4r::R`](R) reader structure"]
impl crate::Readable for SUSP4Rrs {}
#[doc = "`write(|w| ..)` method takes [`susp4r::W`](W) writer structure"]
impl crate::Writable for SUSP4Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUSP4R to value 0"]
impl crate::Resettable for SUSP4Rrs {
    const RESET_VALUE: u32 = 0;
}
