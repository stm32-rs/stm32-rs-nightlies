#[doc = "Register `QUADSPI_PSMAR` reader"]
pub type R = crate::R<QUADSPI_PSMARrs>;
#[doc = "Register `QUADSPI_PSMAR` writer"]
pub type W = crate::W<QUADSPI_PSMARrs>;
#[doc = "Field `MATCH` reader - MATCH"]
pub type MATCH_R = crate::FieldReader<u32>;
#[doc = "Field `MATCH` writer - MATCH"]
pub type MATCH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MATCH"]
    #[inline(always)]
    pub fn match_(&self) -> MATCH_R {
        MATCH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MATCH"]
    #[inline(always)]
    #[must_use]
    pub fn match_(&mut self) -> MATCH_W<QUADSPI_PSMARrs> {
        MATCH_W::new(self, 0)
    }
}
#[doc = "QUADSPI polling status match register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`quadspi_psmar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`quadspi_psmar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QUADSPI_PSMARrs;
impl crate::RegisterSpec for QUADSPI_PSMARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`quadspi_psmar::R`](R) reader structure"]
impl crate::Readable for QUADSPI_PSMARrs {}
#[doc = "`write(|w| ..)` method takes [`quadspi_psmar::W`](W) writer structure"]
impl crate::Writable for QUADSPI_PSMARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QUADSPI_PSMAR to value 0"]
impl crate::Resettable for QUADSPI_PSMARrs {
    const RESET_VALUE: u32 = 0;
}
