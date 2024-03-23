#[doc = "Register `OPTKEYR` reader"]
pub type R = crate::R<OPTKEYRrs>;
#[doc = "Register `OPTKEYR` writer"]
pub type W = crate::W<OPTKEYRrs>;
#[doc = "Field `OPTKEYR` reader - Unlock key option bytes"]
pub type OPTKEYR_R = crate::FieldReader<u32>;
#[doc = "Field `OPTKEYR` writer - Unlock key option bytes"]
pub type OPTKEYR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Unlock key option bytes"]
    #[inline(always)]
    pub fn optkeyr(&self) -> OPTKEYR_R {
        OPTKEYR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Unlock key option bytes"]
    #[inline(always)]
    #[must_use]
    pub fn optkeyr(&mut self) -> OPTKEYR_W<OPTKEYRrs> {
        OPTKEYR_W::new(self, 0)
    }
}
#[doc = "FLASH option key register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optkeyr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optkeyr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPTKEYRrs;
impl crate::RegisterSpec for OPTKEYRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`optkeyr::R`](R) reader structure"]
impl crate::Readable for OPTKEYRrs {}
#[doc = "`write(|w| ..)` method takes [`optkeyr::W`](W) writer structure"]
impl crate::Writable for OPTKEYRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPTKEYR to value 0"]
impl crate::Resettable for OPTKEYRrs {
    const RESET_VALUE: u32 = 0;
}
