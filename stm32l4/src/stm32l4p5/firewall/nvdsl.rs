#[doc = "Register `NVDSL` reader"]
pub type R = crate::R<NVDSLrs>;
#[doc = "Register `NVDSL` writer"]
pub type W = crate::W<NVDSLrs>;
#[doc = "Field `LENG` reader - Non-volatile data segment length"]
pub type LENG_R = crate::FieldReader<u16>;
#[doc = "Field `LENG` writer - Non-volatile data segment length"]
pub type LENG_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 8:21 - Non-volatile data segment length"]
    #[inline(always)]
    pub fn leng(&self) -> LENG_R {
        LENG_R::new(((self.bits >> 8) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:21 - Non-volatile data segment length"]
    #[inline(always)]
    #[must_use]
    pub fn leng(&mut self) -> LENG_W<NVDSLrs> {
        LENG_W::new(self, 8)
    }
}
#[doc = "Non-volatile data segment length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvdsl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvdsl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NVDSLrs;
impl crate::RegisterSpec for NVDSLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvdsl::R`](R) reader structure"]
impl crate::Readable for NVDSLrs {}
#[doc = "`write(|w| ..)` method takes [`nvdsl::W`](W) writer structure"]
impl crate::Writable for NVDSLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVDSL to value 0"]
impl crate::Resettable for NVDSLrs {
    const RESET_VALUE: u32 = 0;
}
