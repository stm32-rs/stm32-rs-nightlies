#[doc = "Register `DFSDM0_AWCFR` reader"]
pub type R = crate::R<DFSDM0_AWCFRrs>;
#[doc = "Register `DFSDM0_AWCFR` writer"]
pub type W = crate::W<DFSDM0_AWCFRrs>;
#[doc = "Field `CLRAWLTF` reader - Clear the analog watchdog low threshold flag"]
pub type CLRAWLTF_R = crate::FieldReader;
#[doc = "Field `CLRAWLTF` writer - Clear the analog watchdog low threshold flag"]
pub type CLRAWLTF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLRAWHTF` reader - Clear the analog watchdog high threshold flag"]
pub type CLRAWHTF_R = crate::FieldReader;
#[doc = "Field `CLRAWHTF` writer - Clear the analog watchdog high threshold flag"]
pub type CLRAWHTF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Clear the analog watchdog low threshold flag"]
    #[inline(always)]
    pub fn clrawltf(&self) -> CLRAWLTF_R {
        CLRAWLTF_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Clear the analog watchdog high threshold flag"]
    #[inline(always)]
    pub fn clrawhtf(&self) -> CLRAWHTF_R {
        CLRAWHTF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clear the analog watchdog low threshold flag"]
    #[inline(always)]
    #[must_use]
    pub fn clrawltf(&mut self) -> CLRAWLTF_W<DFSDM0_AWCFRrs> {
        CLRAWLTF_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Clear the analog watchdog high threshold flag"]
    #[inline(always)]
    #[must_use]
    pub fn clrawhtf(&mut self) -> CLRAWHTF_W<DFSDM0_AWCFRrs> {
        CLRAWHTF_W::new(self, 8)
    }
}
#[doc = "DFSDM analog watchdog clear flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm0_awcfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm0_awcfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM0_AWCFRrs;
impl crate::RegisterSpec for DFSDM0_AWCFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm0_awcfr::R`](R) reader structure"]
impl crate::Readable for DFSDM0_AWCFRrs {}
#[doc = "`write(|w| ..)` method takes [`dfsdm0_awcfr::W`](W) writer structure"]
impl crate::Writable for DFSDM0_AWCFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFSDM0_AWCFR to value 0"]
impl crate::Resettable for DFSDM0_AWCFRrs {
    const RESET_VALUE: u32 = 0;
}
