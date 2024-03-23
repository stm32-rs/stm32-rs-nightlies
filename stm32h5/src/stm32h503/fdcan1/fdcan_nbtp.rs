#[doc = "Register `FDCAN_NBTP` reader"]
pub type R = crate::R<FDCAN_NBTPrs>;
#[doc = "Register `FDCAN_NBTP` writer"]
pub type W = crate::W<FDCAN_NBTPrs>;
#[doc = "Field `NTSEG2` reader - Nominal time segment after sample point Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
pub type NTSEG2_R = crate::FieldReader;
#[doc = "Field `NTSEG2` writer - Nominal time segment after sample point Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
pub type NTSEG2_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NTSEG1` reader - Nominal time segment before sample point Valid values are 0 to 255. The actual interpretation by the hardware of this value is such that one more than the programmed value is used. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type NTSEG1_R = crate::FieldReader;
#[doc = "Field `NTSEG1` writer - Nominal time segment before sample point Valid values are 0 to 255. The actual interpretation by the hardware of this value is such that one more than the programmed value is used. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type NTSEG1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NBRP` reader - Bit rate prescaler Value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values are 0 to 511. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type NBRP_R = crate::FieldReader<u16>;
#[doc = "Field `NBRP` writer - Bit rate prescaler Value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values are 0 to 511. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type NBRP_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `NSJW` reader - Nominal (re)synchronization jump width Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that the used value is the one programmed incremented by one. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type NSJW_R = crate::FieldReader;
#[doc = "Field `NSJW` writer - Nominal (re)synchronization jump width Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that the used value is the one programmed incremented by one. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type NSJW_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Nominal time segment after sample point Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
    #[inline(always)]
    pub fn ntseg2(&self) -> NTSEG2_R {
        NTSEG2_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - Nominal time segment before sample point Valid values are 0 to 255. The actual interpretation by the hardware of this value is such that one more than the programmed value is used. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn ntseg1(&self) -> NTSEG1_R {
        NTSEG1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:24 - Bit rate prescaler Value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values are 0 to 511. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn nbrp(&self) -> NBRP_R {
        NBRP_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31 - Nominal (re)synchronization jump width Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that the used value is the one programmed incremented by one. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn nsjw(&self) -> NSJW_R {
        NSJW_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Nominal time segment after sample point Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
    #[inline(always)]
    #[must_use]
    pub fn ntseg2(&mut self) -> NTSEG2_W<FDCAN_NBTPrs> {
        NTSEG2_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Nominal time segment before sample point Valid values are 0 to 255. The actual interpretation by the hardware of this value is such that one more than the programmed value is used. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn ntseg1(&mut self) -> NTSEG1_W<FDCAN_NBTPrs> {
        NTSEG1_W::new(self, 8)
    }
    #[doc = "Bits 16:24 - Bit rate prescaler Value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values are 0 to 511. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn nbrp(&mut self) -> NBRP_W<FDCAN_NBTPrs> {
        NBRP_W::new(self, 16)
    }
    #[doc = "Bits 25:31 - Nominal (re)synchronization jump width Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that the used value is the one programmed incremented by one. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn nsjw(&mut self) -> NSJW_W<FDCAN_NBTPrs> {
        NSJW_W::new(self, 25)
    }
}
#[doc = "FDCAN nominal bit timing and prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_nbtp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_nbtp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_NBTPrs;
impl crate::RegisterSpec for FDCAN_NBTPrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_nbtp::R`](R) reader structure"]
impl crate::Readable for FDCAN_NBTPrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_nbtp::W`](W) writer structure"]
impl crate::Writable for FDCAN_NBTPrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_NBTP to value 0x0600_0a03"]
impl crate::Resettable for FDCAN_NBTPrs {
    const RESET_VALUE: u32 = 0x0600_0a03;
}
