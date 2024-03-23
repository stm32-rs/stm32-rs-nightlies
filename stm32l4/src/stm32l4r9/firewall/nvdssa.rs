#[doc = "Register `NVDSSA` reader"]
pub type R = crate::R<NVDSSArs>;
#[doc = "Register `NVDSSA` writer"]
pub type W = crate::W<NVDSSArs>;
#[doc = "Field `ADD` reader - Non-volatile data segment start address"]
pub type ADD_R = crate::FieldReader<u16>;
#[doc = "Field `ADD` writer - Non-volatile data segment start address"]
pub type ADD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 8:23 - Non-volatile data segment start address"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:23 - Non-volatile data segment start address"]
    #[inline(always)]
    #[must_use]
    pub fn add(&mut self) -> ADD_W<NVDSSArs> {
        ADD_W::new(self, 8)
    }
}
#[doc = "Non-volatile data segment start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvdssa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvdssa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NVDSSArs;
impl crate::RegisterSpec for NVDSSArs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvdssa::R`](R) reader structure"]
impl crate::Readable for NVDSSArs {}
#[doc = "`write(|w| ..)` method takes [`nvdssa::W`](W) writer structure"]
impl crate::Writable for NVDSSArs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVDSSA to value 0"]
impl crate::Resettable for NVDSSArs {
    const RESET_VALUE: u32 = 0;
}
