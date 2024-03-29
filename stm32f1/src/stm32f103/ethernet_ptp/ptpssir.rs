#[doc = "Register `PTPSSIR` reader"]
pub type R = crate::R<PTPSSIRrs>;
#[doc = "Register `PTPSSIR` writer"]
pub type W = crate::W<PTPSSIRrs>;
#[doc = "Field `STSSI` reader - System time subsecond increment"]
pub type STSSI_R = crate::FieldReader;
#[doc = "Field `STSSI` writer - System time subsecond increment"]
pub type STSSI_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - System time subsecond increment"]
    #[inline(always)]
    pub fn stssi(&self) -> STSSI_R {
        STSSI_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - System time subsecond increment"]
    #[inline(always)]
    #[must_use]
    pub fn stssi(&mut self) -> STSSI_W<PTPSSIRrs> {
        STSSI_W::new(self, 0)
    }
}
#[doc = "Ethernet PTP subsecond increment register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptpssir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptpssir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTPSSIRrs;
impl crate::RegisterSpec for PTPSSIRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptpssir::R`](R) reader structure"]
impl crate::Readable for PTPSSIRrs {}
#[doc = "`write(|w| ..)` method takes [`ptpssir::W`](W) writer structure"]
impl crate::Writable for PTPSSIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PTPSSIR to value 0"]
impl crate::Resettable for PTPSSIRrs {
    const RESET_VALUE: u32 = 0;
}
