///Register `CCMR2` reader
pub type R = crate::R<CCMR2rs>;
///Register `CCMR2` writer
pub type W = crate::W<CCMR2rs>;
///Field `CC3S` reader - CC3S: Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC3 channel is configured as output 01: CC3 channel is configured as input, IC3 is mapped on TI3 10: CC3 channel is configured as input, IC3 is mapped on TI4 11: CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register) Note: CC3S bits are writable only when the channel is OFF (CC3E = '0' in TIMx_CCER).
pub type CC3S_R = crate::FieldReader;
///Field `CC3S` writer - CC3S: Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC3 channel is configured as output 01: CC3 channel is configured as input, IC3 is mapped on TI3 10: CC3 channel is configured as input, IC3 is mapped on TI4 11: CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register) Note: CC3S bits are writable only when the channel is OFF (CC3E = '0' in TIMx_CCER).
pub type CC3S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OC3FE` reader - OC3FE: Output compare 3 fast enable
pub type OC3FE_R = crate::BitReader;
///Field `OC3FE` writer - OC3FE: Output compare 3 fast enable
pub type OC3FE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC3PE` reader - OC3PE: Output compare 3 preload enable
pub type OC3PE_R = crate::BitReader;
///Field `OC3PE` writer - OC3PE: Output compare 3 preload enable
pub type OC3PE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC3M_2_0` reader - OC3M: Output compare 3 mode
pub type OC3M_2_0_R = crate::FieldReader;
///Field `OC3M_2_0` writer - OC3M: Output compare 3 mode
pub type OC3M_2_0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `OC3CE` reader - OC3CE: Output compare 3 clear enable
pub type OC3CE_R = crate::BitReader;
///Field `OC3CE` writer - OC3CE: Output compare 3 clear enable
pub type OC3CE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4S` reader - CC4S: Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC4 channel is configured as output 01: CC4 channel is configured as input, IC4 is mapped on TI4 10: CC4 channel is configured as input, IC4 is mapped on TI3 11: CC4 channel is configured as input, IC4 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register) Note: CC4S bits are writable only when the channel is OFF (CC4E = '0' in TIMx_CCER).
pub type CC4S_R = crate::FieldReader;
///Field `CC4S` writer - CC4S: Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC4 channel is configured as output 01: CC4 channel is configured as input, IC4 is mapped on TI4 10: CC4 channel is configured as input, IC4 is mapped on TI3 11: CC4 channel is configured as input, IC4 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register) Note: CC4S bits are writable only when the channel is OFF (CC4E = '0' in TIMx_CCER).
pub type CC4S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OC4FE` reader - OC4FE: Output Compare 4 fast enable
pub type OC4FE_R = crate::BitReader;
///Field `OC4FE` writer - OC4FE: Output Compare 4 fast enable
pub type OC4FE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC4PE` reader - OC4PE: Output Compare 4 preload enable
pub type OC4PE_R = crate::BitReader;
///Field `OC4PE` writer - OC4PE: Output Compare 4 preload enable
pub type OC4PE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC4M_2_0` reader - OC4M\[2:0\]: Output Compare 4 mode
pub type OC4M_2_0_R = crate::FieldReader;
///Field `OC4M_2_0` writer - OC4M\[2:0\]: Output Compare 4 mode
pub type OC4M_2_0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `OC4CE` reader - OC4CE: Output Compare 4 clear enable
pub type OC4CE_R = crate::BitReader;
///Field `OC4CE` writer - OC4CE: Output Compare 4 clear enable
pub type OC4CE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC3M_3` reader - OC3M\[3\]: Output Compare 3 mode (bit 3)
pub type OC3M_3_R = crate::BitReader;
///Field `OC3M_3` writer - OC3M\[3\]: Output Compare 3 mode (bit 3)
pub type OC3M_3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC4M_3` reader - OC4M\[3\]: Output Compare 4 mode (bit 3)
pub type OC4M_3_R = crate::BitReader;
///Field `OC4M_3` writer - OC4M\[3\]: Output Compare 4 mode (bit 3)
pub type OC4M_3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - CC3S: Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC3 channel is configured as output 01: CC3 channel is configured as input, IC3 is mapped on TI3 10: CC3 channel is configured as input, IC3 is mapped on TI4 11: CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register) Note: CC3S bits are writable only when the channel is OFF (CC3E = '0' in TIMx_CCER).
    #[inline(always)]
    pub fn cc3s(&self) -> CC3S_R {
        CC3S_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - OC3FE: Output compare 3 fast enable
    #[inline(always)]
    pub fn oc3fe(&self) -> OC3FE_R {
        OC3FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - OC3PE: Output compare 3 preload enable
    #[inline(always)]
    pub fn oc3pe(&self) -> OC3PE_R {
        OC3PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - OC3M: Output compare 3 mode
    #[inline(always)]
    pub fn oc3m_2_0(&self) -> OC3M_2_0_R {
        OC3M_2_0_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - OC3CE: Output compare 3 clear enable
    #[inline(always)]
    pub fn oc3ce(&self) -> OC3CE_R {
        OC3CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - CC4S: Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC4 channel is configured as output 01: CC4 channel is configured as input, IC4 is mapped on TI4 10: CC4 channel is configured as input, IC4 is mapped on TI3 11: CC4 channel is configured as input, IC4 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register) Note: CC4S bits are writable only when the channel is OFF (CC4E = '0' in TIMx_CCER).
    #[inline(always)]
    pub fn cc4s(&self) -> CC4S_R {
        CC4S_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - OC4FE: Output Compare 4 fast enable
    #[inline(always)]
    pub fn oc4fe(&self) -> OC4FE_R {
        OC4FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - OC4PE: Output Compare 4 preload enable
    #[inline(always)]
    pub fn oc4pe(&self) -> OC4PE_R {
        OC4PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:14 - OC4M\[2:0\]: Output Compare 4 mode
    #[inline(always)]
    pub fn oc4m_2_0(&self) -> OC4M_2_0_R {
        OC4M_2_0_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - OC4CE: Output Compare 4 clear enable
    #[inline(always)]
    pub fn oc4ce(&self) -> OC4CE_R {
        OC4CE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - OC3M\[3\]: Output Compare 3 mode (bit 3)
    #[inline(always)]
    pub fn oc3m_3(&self) -> OC3M_3_R {
        OC3M_3_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - OC4M\[3\]: Output Compare 4 mode (bit 3)
    #[inline(always)]
    pub fn oc4m_3(&self) -> OC4M_3_R {
        OC4M_3_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCMR2")
            .field("cc3s", &self.cc3s())
            .field("oc3fe", &self.oc3fe())
            .field("oc3pe", &self.oc3pe())
            .field("oc3m_2_0", &self.oc3m_2_0())
            .field("oc3ce", &self.oc3ce())
            .field("cc4s", &self.cc4s())
            .field("oc4fe", &self.oc4fe())
            .field("oc4pe", &self.oc4pe())
            .field("oc4m_2_0", &self.oc4m_2_0())
            .field("oc4ce", &self.oc4ce())
            .field("oc3m_3", &self.oc3m_3())
            .field("oc4m_3", &self.oc4m_3())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - CC3S: Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC3 channel is configured as output 01: CC3 channel is configured as input, IC3 is mapped on TI3 10: CC3 channel is configured as input, IC3 is mapped on TI4 11: CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register) Note: CC3S bits are writable only when the channel is OFF (CC3E = '0' in TIMx_CCER).
    #[inline(always)]
    pub fn cc3s(&mut self) -> CC3S_W<'_, CCMR2rs> {
        CC3S_W::new(self, 0)
    }
    ///Bit 2 - OC3FE: Output compare 3 fast enable
    #[inline(always)]
    pub fn oc3fe(&mut self) -> OC3FE_W<'_, CCMR2rs> {
        OC3FE_W::new(self, 2)
    }
    ///Bit 3 - OC3PE: Output compare 3 preload enable
    #[inline(always)]
    pub fn oc3pe(&mut self) -> OC3PE_W<'_, CCMR2rs> {
        OC3PE_W::new(self, 3)
    }
    ///Bits 4:6 - OC3M: Output compare 3 mode
    #[inline(always)]
    pub fn oc3m_2_0(&mut self) -> OC3M_2_0_W<'_, CCMR2rs> {
        OC3M_2_0_W::new(self, 4)
    }
    ///Bit 7 - OC3CE: Output compare 3 clear enable
    #[inline(always)]
    pub fn oc3ce(&mut self) -> OC3CE_W<'_, CCMR2rs> {
        OC3CE_W::new(self, 7)
    }
    ///Bits 8:9 - CC4S: Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC4 channel is configured as output 01: CC4 channel is configured as input, IC4 is mapped on TI4 10: CC4 channel is configured as input, IC4 is mapped on TI3 11: CC4 channel is configured as input, IC4 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register) Note: CC4S bits are writable only when the channel is OFF (CC4E = '0' in TIMx_CCER).
    #[inline(always)]
    pub fn cc4s(&mut self) -> CC4S_W<'_, CCMR2rs> {
        CC4S_W::new(self, 8)
    }
    ///Bit 10 - OC4FE: Output Compare 4 fast enable
    #[inline(always)]
    pub fn oc4fe(&mut self) -> OC4FE_W<'_, CCMR2rs> {
        OC4FE_W::new(self, 10)
    }
    ///Bit 11 - OC4PE: Output Compare 4 preload enable
    #[inline(always)]
    pub fn oc4pe(&mut self) -> OC4PE_W<'_, CCMR2rs> {
        OC4PE_W::new(self, 11)
    }
    ///Bits 12:14 - OC4M\[2:0\]: Output Compare 4 mode
    #[inline(always)]
    pub fn oc4m_2_0(&mut self) -> OC4M_2_0_W<'_, CCMR2rs> {
        OC4M_2_0_W::new(self, 12)
    }
    ///Bit 15 - OC4CE: Output Compare 4 clear enable
    #[inline(always)]
    pub fn oc4ce(&mut self) -> OC4CE_W<'_, CCMR2rs> {
        OC4CE_W::new(self, 15)
    }
    ///Bit 16 - OC3M\[3\]: Output Compare 3 mode (bit 3)
    #[inline(always)]
    pub fn oc3m_3(&mut self) -> OC3M_3_W<'_, CCMR2rs> {
        OC3M_3_W::new(self, 16)
    }
    ///Bit 24 - OC4M\[3\]: Output Compare 4 mode (bit 3)
    #[inline(always)]
    pub fn oc4m_3(&mut self) -> OC4M_3_W<'_, CCMR2rs> {
        OC4M_3_W::new(self, 24)
    }
}
/**CCMR2 register

You can [`read`](crate::Reg::read) this register and get [`ccmr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#TIM2:CCMR2)*/
pub struct CCMR2rs;
impl crate::RegisterSpec for CCMR2rs {
    type Ux = u32;
}
///`read()` method returns [`ccmr2::R`](R) reader structure
impl crate::Readable for CCMR2rs {}
///`write(|w| ..)` method takes [`ccmr2::W`](W) writer structure
impl crate::Writable for CCMR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCMR2 to value 0
impl crate::Resettable for CCMR2rs {}
