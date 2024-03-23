#[doc = "Register `FDCAN_DBTP` reader"]
pub type R = crate::R<FDCAN_DBTPrs>;
#[doc = "Register `FDCAN_DBTP` writer"]
pub type W = crate::W<FDCAN_DBTPrs>;
#[doc = "Field `DSJW` reader - Synchronization jump width Must always be smaller than DTSEG2, valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1: tSJW = (DSJW + 1) x tq."]
pub type DSJW_R = crate::FieldReader;
#[doc = "Field `DSJW` writer - Synchronization jump width Must always be smaller than DTSEG2, valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1: tSJW = (DSJW + 1) x tq."]
pub type DSJW_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DTSEG2` reader - Data time segment after sample point Valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1, i.e. tBS2 = (DTSEG2 + 1) x tq."]
pub type DTSEG2_R = crate::FieldReader;
#[doc = "Field `DTSEG2` writer - Data time segment after sample point Valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1, i.e. tBS2 = (DTSEG2 + 1) x tq."]
pub type DTSEG2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DTSEG1` reader - Data time segment before sample point Valid values are 0 to 31. The value used by the hardware is the one programmed, incremented by 1, i.e. tBS1 = (DTSEG1 + 1) x tq."]
pub type DTSEG1_R = crate::FieldReader;
#[doc = "Field `DTSEG1` writer - Data time segment before sample point Valid values are 0 to 31. The value used by the hardware is the one programmed, incremented by 1, i.e. tBS1 = (DTSEG1 + 1) x tq."]
pub type DTSEG1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DBRP` reader - Data bit rate prescaler The value by which the oscillator frequency is divided to generate the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Baud Rate Prescaler are 0 to 31. The hardware interpreters this value as the value programmed plus 1."]
pub type DBRP_R = crate::FieldReader;
#[doc = "Field `DBRP` writer - Data bit rate prescaler The value by which the oscillator frequency is divided to generate the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Baud Rate Prescaler are 0 to 31. The hardware interpreters this value as the value programmed plus 1."]
pub type DBRP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TDC` reader - Transceiver delay compensation"]
pub type TDC_R = crate::BitReader;
#[doc = "Field `TDC` writer - Transceiver delay compensation"]
pub type TDC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Synchronization jump width Must always be smaller than DTSEG2, valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1: tSJW = (DSJW + 1) x tq."]
    #[inline(always)]
    pub fn dsjw(&self) -> DSJW_R {
        DSJW_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Data time segment after sample point Valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1, i.e. tBS2 = (DTSEG2 + 1) x tq."]
    #[inline(always)]
    pub fn dtseg2(&self) -> DTSEG2_R {
        DTSEG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Data time segment before sample point Valid values are 0 to 31. The value used by the hardware is the one programmed, incremented by 1, i.e. tBS1 = (DTSEG1 + 1) x tq."]
    #[inline(always)]
    pub fn dtseg1(&self) -> DTSEG1_R {
        DTSEG1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Data bit rate prescaler The value by which the oscillator frequency is divided to generate the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Baud Rate Prescaler are 0 to 31. The hardware interpreters this value as the value programmed plus 1."]
    #[inline(always)]
    pub fn dbrp(&self) -> DBRP_R {
        DBRP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Transceiver delay compensation"]
    #[inline(always)]
    pub fn tdc(&self) -> TDC_R {
        TDC_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Synchronization jump width Must always be smaller than DTSEG2, valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1: tSJW = (DSJW + 1) x tq."]
    #[inline(always)]
    #[must_use]
    pub fn dsjw(&mut self) -> DSJW_W<FDCAN_DBTPrs> {
        DSJW_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Data time segment after sample point Valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1, i.e. tBS2 = (DTSEG2 + 1) x tq."]
    #[inline(always)]
    #[must_use]
    pub fn dtseg2(&mut self) -> DTSEG2_W<FDCAN_DBTPrs> {
        DTSEG2_W::new(self, 4)
    }
    #[doc = "Bits 8:12 - Data time segment before sample point Valid values are 0 to 31. The value used by the hardware is the one programmed, incremented by 1, i.e. tBS1 = (DTSEG1 + 1) x tq."]
    #[inline(always)]
    #[must_use]
    pub fn dtseg1(&mut self) -> DTSEG1_W<FDCAN_DBTPrs> {
        DTSEG1_W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Data bit rate prescaler The value by which the oscillator frequency is divided to generate the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Baud Rate Prescaler are 0 to 31. The hardware interpreters this value as the value programmed plus 1."]
    #[inline(always)]
    #[must_use]
    pub fn dbrp(&mut self) -> DBRP_W<FDCAN_DBTPrs> {
        DBRP_W::new(self, 16)
    }
    #[doc = "Bit 23 - Transceiver delay compensation"]
    #[inline(always)]
    #[must_use]
    pub fn tdc(&mut self) -> TDC_W<FDCAN_DBTPrs> {
        TDC_W::new(self, 23)
    }
}
#[doc = "FDCAN data bit timing and prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_dbtp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_dbtp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_DBTPrs;
impl crate::RegisterSpec for FDCAN_DBTPrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_dbtp::R`](R) reader structure"]
impl crate::Readable for FDCAN_DBTPrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_dbtp::W`](W) writer structure"]
impl crate::Writable for FDCAN_DBTPrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_DBTP to value 0x0a33"]
impl crate::Resettable for FDCAN_DBTPrs {
    const RESET_VALUE: u32 = 0x0a33;
}
