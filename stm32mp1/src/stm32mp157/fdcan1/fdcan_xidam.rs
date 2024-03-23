#[doc = "Register `FDCAN_XIDAM` reader"]
pub type R = crate::R<FDCAN_XIDAMrs>;
#[doc = "Register `FDCAN_XIDAM` writer"]
pub type W = crate::W<FDCAN_XIDAMrs>;
#[doc = "Field `EIDM` reader - EIDM"]
pub type EIDM_R = crate::FieldReader<u32>;
#[doc = "Field `EIDM` writer - EIDM"]
pub type EIDM_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - EIDM"]
    #[inline(always)]
    pub fn eidm(&self) -> EIDM_R {
        EIDM_R::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - EIDM"]
    #[inline(always)]
    #[must_use]
    pub fn eidm(&mut self) -> EIDM_W<FDCAN_XIDAMrs> {
        EIDM_W::new(self, 0)
    }
}
#[doc = "FDCAN extended ID and mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_xidam::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_xidam::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_XIDAMrs;
impl crate::RegisterSpec for FDCAN_XIDAMrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_xidam::R`](R) reader structure"]
impl crate::Readable for FDCAN_XIDAMrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_xidam::W`](W) writer structure"]
impl crate::Writable for FDCAN_XIDAMrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_XIDAM to value 0x1fff_ffff"]
impl crate::Resettable for FDCAN_XIDAMrs {
    const RESET_VALUE: u32 = 0x1fff_ffff;
}
