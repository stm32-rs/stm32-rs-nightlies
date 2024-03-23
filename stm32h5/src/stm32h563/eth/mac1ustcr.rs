#[doc = "Register `MAC1USTCR` reader"]
pub type R = crate::R<MAC1USTCRrs>;
#[doc = "Register `MAC1USTCR` writer"]
pub type W = crate::W<MAC1USTCRrs>;
#[doc = "Field `TIC_1US_CNTR` reader - 1��s tick Counter The application must program this counter so that the number of clock cycles of CSR clock is 1��s (subtract 1 from the value before programming). For example if the CSR clock is 100�MHz then this field needs to be programmed to 100 - 1 = 99 (which is 0x63). This is required to generate the 1��s events that are used to update some of the EEE related counters."]
pub type TIC_1US_CNTR_R = crate::FieldReader<u16>;
#[doc = "Field `TIC_1US_CNTR` writer - 1��s tick Counter The application must program this counter so that the number of clock cycles of CSR clock is 1��s (subtract 1 from the value before programming). For example if the CSR clock is 100�MHz then this field needs to be programmed to 100 - 1 = 99 (which is 0x63). This is required to generate the 1��s events that are used to update some of the EEE related counters."]
pub type TIC_1US_CNTR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - 1��s tick Counter The application must program this counter so that the number of clock cycles of CSR clock is 1��s (subtract 1 from the value before programming). For example if the CSR clock is 100�MHz then this field needs to be programmed to 100 - 1 = 99 (which is 0x63). This is required to generate the 1��s events that are used to update some of the EEE related counters."]
    #[inline(always)]
    pub fn tic_1us_cntr(&self) -> TIC_1US_CNTR_R {
        TIC_1US_CNTR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 1��s tick Counter The application must program this counter so that the number of clock cycles of CSR clock is 1��s (subtract 1 from the value before programming). For example if the CSR clock is 100�MHz then this field needs to be programmed to 100 - 1 = 99 (which is 0x63). This is required to generate the 1��s events that are used to update some of the EEE related counters."]
    #[inline(always)]
    #[must_use]
    pub fn tic_1us_cntr(&mut self) -> TIC_1US_CNTR_W<MAC1USTCRrs> {
        TIC_1US_CNTR_W::new(self, 0)
    }
}
#[doc = "One-microsecond-tick counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac1ustcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac1ustcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC1USTCRrs;
impl crate::RegisterSpec for MAC1USTCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac1ustcr::R`](R) reader structure"]
impl crate::Readable for MAC1USTCRrs {}
#[doc = "`write(|w| ..)` method takes [`mac1ustcr::W`](W) writer structure"]
impl crate::Writable for MAC1USTCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAC1USTCR to value 0"]
impl crate::Resettable for MAC1USTCRrs {
    const RESET_VALUE: u32 = 0;
}
