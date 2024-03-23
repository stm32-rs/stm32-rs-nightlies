#[doc = "Register `GICV_BPR` reader"]
pub type R = crate::R<GICV_BPRrs>;
#[doc = "Register `GICV_BPR` writer"]
pub type W = crate::W<GICV_BPRrs>;
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
    pub fn binary_point(&mut self) -> BINARY_POINT_W<GICV_BPRrs> {
        BINARY_POINT_W::new(self, 0)
    }
}
#[doc = "GICV VM binary point register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicv_bpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicv_bpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICV_BPRrs;
impl crate::RegisterSpec for GICV_BPRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicv_bpr::R`](R) reader structure"]
impl crate::Readable for GICV_BPRrs {}
#[doc = "`write(|w| ..)` method takes [`gicv_bpr::W`](W) writer structure"]
impl crate::Writable for GICV_BPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICV_BPR to value 0x02"]
impl crate::Resettable for GICV_BPRrs {
    const RESET_VALUE: u32 = 0x02;
}
