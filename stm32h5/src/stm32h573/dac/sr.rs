#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "Field `DAC1RDY` reader - DAC channel1 ready status bit This bit is set and cleared by hardware."]
pub type DAC1RDY_R = crate::BitReader;
#[doc = "Field `DORSTAT1` reader - DAC channel1 output register status bit This bit is set and cleared by hardware. It is applicable only when the DAC operates in Double data mode."]
pub type DORSTAT1_R = crate::BitReader;
#[doc = "Field `DMAUDR1` reader - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
pub type DMAUDR1_R = crate::BitReader;
#[doc = "Field `DMAUDR1` writer - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
pub type DMAUDR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAL_FLAG1` reader - DAC channel1 calibration offset status This bit is set and cleared by hardware"]
pub type CAL_FLAG1_R = crate::BitReader;
#[doc = "Field `BWST1` reader - DAC channel1 busy writing sample time flag This bit is systematically set just after Sample and hold mode enable and is set each time the software writes the register DAC_SHSR1, It is cleared by hardware when the write operation of DAC_SHSR1 is complete. (It takes about 3 LSI/LSE periods of synchronization)."]
pub type BWST1_R = crate::BitReader;
#[doc = "Field `DAC2RDY` reader - DAC channel2 ready status bit This bit is set and cleared by hardware. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
pub type DAC2RDY_R = crate::BitReader;
#[doc = "Field `DORSTAT2` reader - DAC channel2 output register status bit This bit is set and cleared by hardware. It is applicable only when the DAC operates in Double data mode. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
pub type DORSTAT2_R = crate::BitReader;
#[doc = "Field `DMAUDR2` reader - DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1). Note: This bit is available only on dual-channel DACs. Refer to implementation."]
pub type DMAUDR2_R = crate::BitReader;
#[doc = "Field `DMAUDR2` writer - DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1). Note: This bit is available only on dual-channel DACs. Refer to implementation."]
pub type DMAUDR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAL_FLAG2` reader - DAC channel2 calibration offset status This bit is set and cleared by hardware Note: This bit is available only on dual-channel DACs. Refer to implementation."]
pub type CAL_FLAG2_R = crate::BitReader;
#[doc = "Field `BWST2` reader - DAC channel2 busy writing sample time flag This bit is systematically set just after Sample and hold mode enable. It is set each time the software writes the register DAC_SHSR2, It is cleared by hardware when the write operation of DAC_SHSR2 is complete. (It takes about 3 LSI/LSE periods of synchronization). Note: This bit is available only on dual-channel DACs. Refer to implementation."]
pub type BWST2_R = crate::BitReader;
impl R {
    #[doc = "Bit 11 - DAC channel1 ready status bit This bit is set and cleared by hardware."]
    #[inline(always)]
    pub fn dac1rdy(&self) -> DAC1RDY_R {
        DAC1RDY_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DAC channel1 output register status bit This bit is set and cleared by hardware. It is applicable only when the DAC operates in Double data mode."]
    #[inline(always)]
    pub fn dorstat1(&self) -> DORSTAT1_R {
        DORSTAT1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
    #[inline(always)]
    pub fn dmaudr1(&self) -> DMAUDR1_R {
        DMAUDR1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DAC channel1 calibration offset status This bit is set and cleared by hardware"]
    #[inline(always)]
    pub fn cal_flag1(&self) -> CAL_FLAG1_R {
        CAL_FLAG1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DAC channel1 busy writing sample time flag This bit is systematically set just after Sample and hold mode enable and is set each time the software writes the register DAC_SHSR1, It is cleared by hardware when the write operation of DAC_SHSR1 is complete. (It takes about 3 LSI/LSE periods of synchronization)."]
    #[inline(always)]
    pub fn bwst1(&self) -> BWST1_R {
        BWST1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 27 - DAC channel2 ready status bit This bit is set and cleared by hardware. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn dac2rdy(&self) -> DAC2RDY_R {
        DAC2RDY_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DAC channel2 output register status bit This bit is set and cleared by hardware. It is applicable only when the DAC operates in Double data mode. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn dorstat2(&self) -> DORSTAT2_R {
        DORSTAT2_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1). Note: This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn dmaudr2(&self) -> DMAUDR2_R {
        DMAUDR2_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DAC channel2 calibration offset status This bit is set and cleared by hardware Note: This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn cal_flag2(&self) -> CAL_FLAG2_R {
        CAL_FLAG2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DAC channel2 busy writing sample time flag This bit is systematically set just after Sample and hold mode enable. It is set each time the software writes the register DAC_SHSR2, It is cleared by hardware when the write operation of DAC_SHSR2 is complete. (It takes about 3 LSI/LSE periods of synchronization). Note: This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn bwst2(&self) -> BWST2_R {
        BWST2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
    #[inline(always)]
    #[must_use]
    pub fn dmaudr1(&mut self) -> DMAUDR1_W<SRrs> {
        DMAUDR1_W::new(self, 13)
    }
    #[doc = "Bit 29 - DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1). Note: This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    #[must_use]
    pub fn dmaudr2(&mut self) -> DMAUDR2_W<SRrs> {
        DMAUDR2_W::new(self, 29)
    }
}
#[doc = "DAC status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
