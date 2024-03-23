#[doc = "Register `PTPTSAR` reader"]
pub type R = crate::R<PTPTSARrs>;
#[doc = "Register `PTPTSAR` writer"]
pub type W = crate::W<PTPTSARrs>;
#[doc = "Field `TSA` reader - Time stamp addend"]
pub type TSA_R = crate::FieldReader<u32>;
#[doc = "Field `TSA` writer - Time stamp addend"]
pub type TSA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Time stamp addend"]
    #[inline(always)]
    pub fn tsa(&self) -> TSA_R {
        TSA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Time stamp addend"]
    #[inline(always)]
    #[must_use]
    pub fn tsa(&mut self) -> TSA_W<PTPTSARrs> {
        TSA_W::new(self, 0)
    }
}
#[doc = "Ethernet PTP time stamp addend register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptsar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptsar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTPTSARrs;
impl crate::RegisterSpec for PTPTSARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptsar::R`](R) reader structure"]
impl crate::Readable for PTPTSARrs {}
#[doc = "`write(|w| ..)` method takes [`ptptsar::W`](W) writer structure"]
impl crate::Writable for PTPTSARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PTPTSAR to value 0"]
impl crate::Resettable for PTPTSARrs {
    const RESET_VALUE: u32 = 0;
}
