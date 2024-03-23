#[doc = "Register `SWTRGR` writer"]
pub type W = crate::W<SWTRGRrs>;
#[doc = "Field `SWTRIG1` writer - DAC channel1 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one APB1 clock cycle later) once the DAC_DHR1 register value has been loaded into the DAC_DOR1 register."]
pub type SWTRIG1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWTRIG2` writer - DAC channel2 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one APB1 clock cycle later) once the DAC_DHR2 register value has been loaded into the DAC_DOR2 register."]
pub type SWTRIG2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - DAC channel1 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one APB1 clock cycle later) once the DAC_DHR1 register value has been loaded into the DAC_DOR1 register."]
    #[inline(always)]
    #[must_use]
    pub fn swtrig1(&mut self) -> SWTRIG1_W<SWTRGRrs> {
        SWTRIG1_W::new(self, 0)
    }
    #[doc = "Bit 1 - DAC channel2 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one APB1 clock cycle later) once the DAC_DHR2 register value has been loaded into the DAC_DOR2 register."]
    #[inline(always)]
    #[must_use]
    pub fn swtrig2(&mut self) -> SWTRIG2_W<SWTRGRrs> {
        SWTRIG2_W::new(self, 1)
    }
}
#[doc = "DAC software trigger register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swtrgr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWTRGRrs;
impl crate::RegisterSpec for SWTRGRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swtrgr::W`](W) writer structure"]
impl crate::Writable for SWTRGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWTRGR to value 0"]
impl crate::Resettable for SWTRGRrs {
    const RESET_VALUE: u32 = 0;
}
