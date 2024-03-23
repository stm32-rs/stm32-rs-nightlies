#[doc = "Register `SAR` reader"]
pub type R = crate::R<SARrs>;
#[doc = "Register `SAR` writer"]
pub type W = crate::W<SARrs>;
#[doc = "Field `SAR` reader - source adr base"]
pub type SAR_R = crate::FieldReader<u32>;
#[doc = "Field `SAR` writer - source adr base"]
pub type SAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - source adr base"]
    #[inline(always)]
    pub fn sar(&self) -> SAR_R {
        SAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - source adr base"]
    #[inline(always)]
    #[must_use]
    pub fn sar(&mut self) -> SAR_W<SARrs> {
        SAR_W::new(self, 0)
    }
}
#[doc = "MDMA channel x source address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SARrs;
impl crate::RegisterSpec for SARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar::R`](R) reader structure"]
impl crate::Readable for SARrs {}
#[doc = "`write(|w| ..)` method takes [`sar::W`](W) writer structure"]
impl crate::Writable for SARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR to value 0"]
impl crate::Resettable for SARrs {
    const RESET_VALUE: u32 = 0;
}
