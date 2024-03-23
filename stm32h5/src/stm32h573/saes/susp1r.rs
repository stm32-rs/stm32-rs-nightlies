#[doc = "Register `SUSP1R` reader"]
pub type R = crate::R<SUSP1Rrs>;
#[doc = "Register `SUSP1R` writer"]
pub type W = crate::W<SUSP1Rrs>;
#[doc = "Field `SUSP` reader - SAES suspend Upon suspend operation, this bitfield of the corresponding SAES_SUSPxR register takes the value of one of internal SAES registers."]
pub type SUSP_R = crate::FieldReader<u32>;
#[doc = "Field `SUSP` writer - SAES suspend Upon suspend operation, this bitfield of the corresponding SAES_SUSPxR register takes the value of one of internal SAES registers."]
pub type SUSP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SAES suspend Upon suspend operation, this bitfield of the corresponding SAES_SUSPxR register takes the value of one of internal SAES registers."]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SAES suspend Upon suspend operation, this bitfield of the corresponding SAES_SUSPxR register takes the value of one of internal SAES registers."]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<SUSP1Rrs> {
        SUSP_W::new(self, 0)
    }
}
#[doc = "SAES suspend registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SUSP1Rrs;
impl crate::RegisterSpec for SUSP1Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`susp1r::R`](R) reader structure"]
impl crate::Readable for SUSP1Rrs {}
#[doc = "`write(|w| ..)` method takes [`susp1r::W`](W) writer structure"]
impl crate::Writable for SUSP1Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUSP1R to value 0"]
impl crate::Resettable for SUSP1Rrs {
    const RESET_VALUE: u32 = 0;
}
