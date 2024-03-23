#[doc = "Register `HASH_DIN` reader"]
pub type R = crate::R<HASH_DINrs>;
#[doc = "Register `HASH_DIN` writer"]
pub type W = crate::W<HASH_DINrs>;
#[doc = "Field `DATAIN` reader - DATAIN"]
pub type DATAIN_R = crate::FieldReader<u32>;
#[doc = "Field `DATAIN` writer - DATAIN"]
pub type DATAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DATAIN"]
    #[inline(always)]
    pub fn datain(&self) -> DATAIN_R {
        DATAIN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DATAIN"]
    #[inline(always)]
    #[must_use]
    pub fn datain(&mut self) -> DATAIN_W<HASH_DINrs> {
        DATAIN_W::new(self, 0)
    }
}
#[doc = "HASH_DIN is the data input register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_din::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_din::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_DINrs;
impl crate::RegisterSpec for HASH_DINrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_din::R`](R) reader structure"]
impl crate::Readable for HASH_DINrs {}
#[doc = "`write(|w| ..)` method takes [`hash_din::W`](W) writer structure"]
impl crate::Writable for HASH_DINrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_DIN to value 0"]
impl crate::Resettable for HASH_DINrs {
    const RESET_VALUE: u32 = 0;
}
