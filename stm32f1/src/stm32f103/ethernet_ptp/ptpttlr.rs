#[doc = "Register `PTPTTLR` reader"]
pub type R = crate::R<PTPTTLRrs>;
#[doc = "Register `PTPTTLR` writer"]
pub type W = crate::W<PTPTTLRrs>;
#[doc = "Field `TTSL` reader - Target time stamp low"]
pub type TTSL_R = crate::FieldReader<u32>;
#[doc = "Field `TTSL` writer - Target time stamp low"]
pub type TTSL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Target time stamp low"]
    #[inline(always)]
    pub fn ttsl(&self) -> TTSL_R {
        TTSL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Target time stamp low"]
    #[inline(always)]
    #[must_use]
    pub fn ttsl(&mut self) -> TTSL_W<PTPTTLRrs> {
        TTSL_W::new(self, 0)
    }
}
#[doc = "Ethernet PTP target time low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptpttlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptpttlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTPTTLRrs;
impl crate::RegisterSpec for PTPTTLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptpttlr::R`](R) reader structure"]
impl crate::Readable for PTPTTLRrs {}
#[doc = "`write(|w| ..)` method takes [`ptpttlr::W`](W) writer structure"]
impl crate::Writable for PTPTTLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PTPTTLR to value 0"]
impl crate::Resettable for PTPTTLRrs {
    const RESET_VALUE: u32 = 0;
}
