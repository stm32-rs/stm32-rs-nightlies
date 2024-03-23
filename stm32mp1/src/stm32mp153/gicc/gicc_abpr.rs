#[doc = "Register `GICC_ABPR` reader"]
pub type R = crate::R<GICC_ABPRrs>;
#[doc = "Register `GICC_ABPR` writer"]
pub type W = crate::W<GICC_ABPRrs>;
#[doc = "Field `BINARY_POINT` reader - BINARY_POINT"]
pub type BINARY_POINT_R = crate::FieldReader;
#[doc = "Field `BINARY_POINT` writer - BINARY_POINT"]
pub type BINARY_POINT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - BINARY_POINT"]
    #[inline(always)]
    pub fn binary_point(&self) -> BINARY_POINT_R {
        BINARY_POINT_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - BINARY_POINT"]
    #[inline(always)]
    #[must_use]
    pub fn binary_point(&mut self) -> BINARY_POINT_W<GICC_ABPRrs> {
        BINARY_POINT_W::new(self, 0)
    }
}
#[doc = "GICC_ABPR is an alias of the non-secure GICC_BPR. When GICC_CTLR.CBPR is set to 0, a secure access to this register is equivalent to a non-secure access to GICC_BPR.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicc_abpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicc_abpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICC_ABPRrs;
impl crate::RegisterSpec for GICC_ABPRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicc_abpr::R`](R) reader structure"]
impl crate::Readable for GICC_ABPRrs {}
#[doc = "`write(|w| ..)` method takes [`gicc_abpr::W`](W) writer structure"]
impl crate::Writable for GICC_ABPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICC_ABPR to value 0x03"]
impl crate::Resettable for GICC_ABPRrs {
    const RESET_VALUE: u32 = 0x03;
}
