#[doc = "Register `EPOCHSELCR` reader"]
pub type R = crate::R<EPOCHSELCRrs>;
#[doc = "Register `EPOCHSELCR` writer"]
pub type W = crate::W<EPOCHSELCRrs>;
#[doc = "Field `EPOCH_SEL` reader - select EPOCH value to be sent to the SAES 1x: EPOCH forced to zero (value used to retrieve PUF reference value at boot time)"]
pub type EPOCH_SEL_R = crate::FieldReader;
#[doc = "Field `EPOCH_SEL` writer - select EPOCH value to be sent to the SAES 1x: EPOCH forced to zero (value used to retrieve PUF reference value at boot time)"]
pub type EPOCH_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - select EPOCH value to be sent to the SAES 1x: EPOCH forced to zero (value used to retrieve PUF reference value at boot time)"]
    #[inline(always)]
    pub fn epoch_sel(&self) -> EPOCH_SEL_R {
        EPOCH_SEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - select EPOCH value to be sent to the SAES 1x: EPOCH forced to zero (value used to retrieve PUF reference value at boot time)"]
    #[inline(always)]
    #[must_use]
    pub fn epoch_sel(&mut self) -> EPOCH_SEL_W<EPOCHSELCRrs> {
        EPOCH_SEL_W::new(self, 0)
    }
}
#[doc = "SBS EPOCH selection control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epochselcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epochselcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPOCHSELCRrs;
impl crate::RegisterSpec for EPOCHSELCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epochselcr::R`](R) reader structure"]
impl crate::Readable for EPOCHSELCRrs {}
#[doc = "`write(|w| ..)` method takes [`epochselcr::W`](W) writer structure"]
impl crate::Writable for EPOCHSELCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EPOCHSELCR to value 0"]
impl crate::Resettable for EPOCHSELCRrs {
    const RESET_VALUE: u32 = 0;
}
