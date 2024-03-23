#[doc = "Register `PTPTSHUR` reader"]
pub type R = crate::R<PTPTSHURrs>;
#[doc = "Register `PTPTSHUR` writer"]
pub type W = crate::W<PTPTSHURrs>;
#[doc = "Field `TSUS` reader - TSUS"]
pub type TSUS_R = crate::FieldReader<u32>;
#[doc = "Field `TSUS` writer - TSUS"]
pub type TSUS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TSUS"]
    #[inline(always)]
    pub fn tsus(&self) -> TSUS_R {
        TSUS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TSUS"]
    #[inline(always)]
    #[must_use]
    pub fn tsus(&mut self) -> TSUS_W<PTPTSHURrs> {
        TSUS_W::new(self, 0)
    }
}
#[doc = "Ethernet PTP time stamp high update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptshur::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptshur::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTPTSHURrs;
impl crate::RegisterSpec for PTPTSHURrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptshur::R`](R) reader structure"]
impl crate::Readable for PTPTSHURrs {}
#[doc = "`write(|w| ..)` method takes [`ptptshur::W`](W) writer structure"]
impl crate::Writable for PTPTSHURrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PTPTSHUR to value 0"]
impl crate::Resettable for PTPTSHURrs {
    const RESET_VALUE: u32 = 0;
}
