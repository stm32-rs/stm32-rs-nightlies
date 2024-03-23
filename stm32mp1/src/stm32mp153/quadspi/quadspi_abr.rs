#[doc = "Register `QUADSPI_ABR` reader"]
pub type R = crate::R<QUADSPI_ABRrs>;
#[doc = "Register `QUADSPI_ABR` writer"]
pub type W = crate::W<QUADSPI_ABRrs>;
#[doc = "Field `ALTERNATE` reader - ALTERNATE"]
pub type ALTERNATE_R = crate::FieldReader<u32>;
#[doc = "Field `ALTERNATE` writer - ALTERNATE"]
pub type ALTERNATE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ALTERNATE"]
    #[inline(always)]
    pub fn alternate(&self) -> ALTERNATE_R {
        ALTERNATE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ALTERNATE"]
    #[inline(always)]
    #[must_use]
    pub fn alternate(&mut self) -> ALTERNATE_W<QUADSPI_ABRrs> {
        ALTERNATE_W::new(self, 0)
    }
}
#[doc = "QUADSPI alternate bytes registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`quadspi_abr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`quadspi_abr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QUADSPI_ABRrs;
impl crate::RegisterSpec for QUADSPI_ABRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`quadspi_abr::R`](R) reader structure"]
impl crate::Readable for QUADSPI_ABRrs {}
#[doc = "`write(|w| ..)` method takes [`quadspi_abr::W`](W) writer structure"]
impl crate::Writable for QUADSPI_ABRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QUADSPI_ABR to value 0"]
impl crate::Resettable for QUADSPI_ABRrs {
    const RESET_VALUE: u32 = 0;
}
