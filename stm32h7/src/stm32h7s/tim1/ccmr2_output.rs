///Register `CCMR2_OUTPUT` reader
pub type R = crate::R<CCMR2_OUTPUTrs>;
///Register `CCMR2_OUTPUT` writer
pub type W = crate::W<CCMR2_OUTPUTrs>;
///Field `CC3S` reader - Capture/compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER).
pub type CC3S_R = crate::FieldReader;
///Field `CC3S` writer - Capture/compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER).
pub type CC3S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OC3FE` reader - Output compare 3 fast enable
pub type OC3FE_R = crate::BitReader;
///Field `OC3FE` writer - Output compare 3 fast enable
pub type OC3FE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC3PE` reader - Output compare 3 preload enable
pub type OC3PE_R = crate::BitReader;
///Field `OC3PE` writer - Output compare 3 preload enable
pub type OC3PE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC3M` reader - OC3M\[2:0\]: Output compare 3 mode These bits define the behavior of the output reference signal tim_oc3ref from which tim_oc3 and tim_oc3n are derived. tim_oc3ref is active high whereas tim_oc3 and tim_oc3n active level depends on CC3P and CC3NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=00 (the channel is configured in output). Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from frozen mode to PWM mode. On channels having a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC3M active bits take the new value from the preloaded bits only when a COM event is generated.
pub type OC3M_R = crate::FieldReader;
///Field `OC3M` writer - OC3M\[2:0\]: Output compare 3 mode These bits define the behavior of the output reference signal tim_oc3ref from which tim_oc3 and tim_oc3n are derived. tim_oc3ref is active high whereas tim_oc3 and tim_oc3n active level depends on CC3P and CC3NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=00 (the channel is configured in output). Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from frozen mode to PWM mode. On channels having a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC3M active bits take the new value from the preloaded bits only when a COM event is generated.
pub type OC3M_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `OC3CE` reader - Output compare 3 clear enable
pub type OC3CE_R = crate::BitReader;
///Field `OC3CE` writer - Output compare 3 clear enable
pub type OC3CE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4S` reader - Capture/compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER).
pub type CC4S_R = crate::FieldReader;
///Field `CC4S` writer - Capture/compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER).
pub type CC4S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OC4FE` reader - Output compare 4 fast enable
pub type OC4FE_R = crate::BitReader;
///Field `OC4FE` writer - Output compare 4 fast enable
pub type OC4FE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC4PE` reader - Output compare 4 preload enable
pub type OC4PE_R = crate::BitReader;
///Field `OC4PE` writer - Output compare 4 preload enable
pub type OC4PE_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `OC4M` reader - OC4M\[2:0\]: Output compare 4 mode Refer to OC3M\[3:0\]
bit description*/
pub type OC4M_R = crate::FieldReader;
/**Field `OC4M` writer - OC4M\[2:0\]: Output compare 4 mode Refer to OC3M\[3:0\]
bit description*/
pub type OC4M_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `OC4CE` reader - Output compare 4 clear enable
pub type OC4CE_R = crate::BitReader;
///Field `OC4CE` writer - Output compare 4 clear enable
pub type OC4CE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC3M_1` reader - OC3M\[3\]
pub type OC3M_1_R = crate::BitReader;
///Field `OC3M_1` writer - OC3M\[3\]
pub type OC3M_1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC4M_1` reader - OC4M\[3\]
pub type OC4M_1_R = crate::BitReader;
///Field `OC4M_1` writer - OC4M\[3\]
pub type OC4M_1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Capture/compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER).
    #[inline(always)]
    pub fn cc3s(&self) -> CC3S_R {
        CC3S_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Output compare 3 fast enable
    #[inline(always)]
    pub fn oc3fe(&self) -> OC3FE_R {
        OC3FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Output compare 3 preload enable
    #[inline(always)]
    pub fn oc3pe(&self) -> OC3PE_R {
        OC3PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - OC3M\[2:0\]: Output compare 3 mode These bits define the behavior of the output reference signal tim_oc3ref from which tim_oc3 and tim_oc3n are derived. tim_oc3ref is active high whereas tim_oc3 and tim_oc3n active level depends on CC3P and CC3NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=00 (the channel is configured in output). Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from frozen mode to PWM mode. On channels having a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC3M active bits take the new value from the preloaded bits only when a COM event is generated.
    #[inline(always)]
    pub fn oc3m(&self) -> OC3M_R {
        OC3M_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Output compare 3 clear enable
    #[inline(always)]
    pub fn oc3ce(&self) -> OC3CE_R {
        OC3CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - Capture/compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER).
    #[inline(always)]
    pub fn cc4s(&self) -> CC4S_R {
        CC4S_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - Output compare 4 fast enable
    #[inline(always)]
    pub fn oc4fe(&self) -> OC4FE_R {
        OC4FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Output compare 4 preload enable
    #[inline(always)]
    pub fn oc4pe(&self) -> OC4PE_R {
        OC4PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    /**Bits 12:14 - OC4M\[2:0\]: Output compare 4 mode Refer to OC3M\[3:0\]
    bit description*/
    #[inline(always)]
    pub fn oc4m(&self) -> OC4M_R {
        OC4M_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - Output compare 4 clear enable
    #[inline(always)]
    pub fn oc4ce(&self) -> OC4CE_R {
        OC4CE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - OC3M\[3\]
    #[inline(always)]
    pub fn oc3m_1(&self) -> OC3M_1_R {
        OC3M_1_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - OC4M\[3\]
    #[inline(always)]
    pub fn oc4m_1(&self) -> OC4M_1_R {
        OC4M_1_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCMR2_OUTPUT")
            .field("cc3s", &self.cc3s())
            .field("oc3fe", &self.oc3fe())
            .field("oc3pe", &self.oc3pe())
            .field("oc3m", &self.oc3m())
            .field("oc3ce", &self.oc3ce())
            .field("cc4s", &self.cc4s())
            .field("oc4fe", &self.oc4fe())
            .field("oc4pe", &self.oc4pe())
            .field("oc4m", &self.oc4m())
            .field("oc4ce", &self.oc4ce())
            .field("oc3m_1", &self.oc3m_1())
            .field("oc4m_1", &self.oc4m_1())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Capture/compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER).
    #[inline(always)]
    pub fn cc3s(&mut self) -> CC3S_W<CCMR2_OUTPUTrs> {
        CC3S_W::new(self, 0)
    }
    ///Bit 2 - Output compare 3 fast enable
    #[inline(always)]
    pub fn oc3fe(&mut self) -> OC3FE_W<CCMR2_OUTPUTrs> {
        OC3FE_W::new(self, 2)
    }
    ///Bit 3 - Output compare 3 preload enable
    #[inline(always)]
    pub fn oc3pe(&mut self) -> OC3PE_W<CCMR2_OUTPUTrs> {
        OC3PE_W::new(self, 3)
    }
    ///Bits 4:6 - OC3M\[2:0\]: Output compare 3 mode These bits define the behavior of the output reference signal tim_oc3ref from which tim_oc3 and tim_oc3n are derived. tim_oc3ref is active high whereas tim_oc3 and tim_oc3n active level depends on CC3P and CC3NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=00 (the channel is configured in output). Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from frozen mode to PWM mode. On channels having a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC3M active bits take the new value from the preloaded bits only when a COM event is generated.
    #[inline(always)]
    pub fn oc3m(&mut self) -> OC3M_W<CCMR2_OUTPUTrs> {
        OC3M_W::new(self, 4)
    }
    ///Bit 7 - Output compare 3 clear enable
    #[inline(always)]
    pub fn oc3ce(&mut self) -> OC3CE_W<CCMR2_OUTPUTrs> {
        OC3CE_W::new(self, 7)
    }
    ///Bits 8:9 - Capture/compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER).
    #[inline(always)]
    pub fn cc4s(&mut self) -> CC4S_W<CCMR2_OUTPUTrs> {
        CC4S_W::new(self, 8)
    }
    ///Bit 10 - Output compare 4 fast enable
    #[inline(always)]
    pub fn oc4fe(&mut self) -> OC4FE_W<CCMR2_OUTPUTrs> {
        OC4FE_W::new(self, 10)
    }
    ///Bit 11 - Output compare 4 preload enable
    #[inline(always)]
    pub fn oc4pe(&mut self) -> OC4PE_W<CCMR2_OUTPUTrs> {
        OC4PE_W::new(self, 11)
    }
    /**Bits 12:14 - OC4M\[2:0\]: Output compare 4 mode Refer to OC3M\[3:0\]
    bit description*/
    #[inline(always)]
    pub fn oc4m(&mut self) -> OC4M_W<CCMR2_OUTPUTrs> {
        OC4M_W::new(self, 12)
    }
    ///Bit 15 - Output compare 4 clear enable
    #[inline(always)]
    pub fn oc4ce(&mut self) -> OC4CE_W<CCMR2_OUTPUTrs> {
        OC4CE_W::new(self, 15)
    }
    ///Bit 16 - OC3M\[3\]
    #[inline(always)]
    pub fn oc3m_1(&mut self) -> OC3M_1_W<CCMR2_OUTPUTrs> {
        OC3M_1_W::new(self, 16)
    }
    ///Bit 24 - OC4M\[3\]
    #[inline(always)]
    pub fn oc4m_1(&mut self) -> OC4M_1_W<CCMR2_OUTPUTrs> {
        OC4M_1_W::new(self, 24)
    }
}
/**TIM1 capture/compare mode register 2

You can [`read`](crate::Reg::read) this register and get [`ccmr2_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#TIM1:CCMR2_OUTPUT)*/
pub struct CCMR2_OUTPUTrs;
impl crate::RegisterSpec for CCMR2_OUTPUTrs {
    type Ux = u32;
}
///`read()` method returns [`ccmr2_output::R`](R) reader structure
impl crate::Readable for CCMR2_OUTPUTrs {}
///`write(|w| ..)` method takes [`ccmr2_output::W`](W) writer structure
impl crate::Writable for CCMR2_OUTPUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCMR2_OUTPUT to value 0
impl crate::Resettable for CCMR2_OUTPUTrs {
    const RESET_VALUE: u32 = 0;
}
