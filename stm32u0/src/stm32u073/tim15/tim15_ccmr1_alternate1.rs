///Register `TIM15_CCMR1_ALTERNATE1` reader
pub type R = crate::R<TIM15_CCMR1_ALTERNATE1rs>;
///Register `TIM15_CCMR1_ALTERNATE1` writer
pub type W = crate::W<TIM15_CCMR1_ALTERNATE1rs>;
///Field `CC1S` reader - Capture/Compare 1 selection
pub type CC1S_R = crate::FieldReader;
///Field `CC1S` writer - Capture/Compare 1 selection
pub type CC1S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OC1FE` reader - Output Compare 1 fast enable
pub type OC1FE_R = crate::BitReader;
///Field `OC1FE` writer - Output Compare 1 fast enable
pub type OC1FE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC1PE` reader - Output Compare 1 preload enable
pub type OC1PE_R = crate::BitReader;
///Field `OC1PE` writer - Output Compare 1 preload enable
pub type OC1PE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC1M` reader - OC1M\[2:0\]: Output Compare 1 mode
pub type OC1M_R = crate::FieldReader;
///Field `OC1M` writer - OC1M\[2:0\]: Output Compare 1 mode
pub type OC1M_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CC2S` reader - Capture/Compare 2 selection
pub type CC2S_R = crate::FieldReader;
///Field `CC2S` writer - Capture/Compare 2 selection
pub type CC2S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OC2FE` reader - Output Compare 2 fast enable
pub type OC2FE_R = crate::BitReader;
///Field `OC2FE` writer - Output Compare 2 fast enable
pub type OC2FE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC2PE` reader - Output Compare 2 preload enable
pub type OC2PE_R = crate::BitReader;
///Field `OC2PE` writer - Output Compare 2 preload enable
pub type OC2PE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC2M` reader - OC2M\[2:0\]: Output Compare 2 mode
pub type OC2M_R = crate::FieldReader;
///Field `OC2M` writer - OC2M\[2:0\]: Output Compare 2 mode
pub type OC2M_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `OC1M_1` reader - OC1M\[3\]
pub type OC1M_1_R = crate::BitReader;
///Field `OC1M_1` writer - OC1M\[3\]
pub type OC1M_1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC2M_1` reader - OC2M\[3\]
pub type OC2M_1_R = crate::BitReader;
///Field `OC2M_1` writer - OC2M\[3\]
pub type OC2M_1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Capture/Compare 1 selection
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Output Compare 1 fast enable
    #[inline(always)]
    pub fn oc1fe(&self) -> OC1FE_R {
        OC1FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Output Compare 1 preload enable
    #[inline(always)]
    pub fn oc1pe(&self) -> OC1PE_R {
        OC1PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - OC1M\[2:0\]: Output Compare 1 mode
    #[inline(always)]
    pub fn oc1m(&self) -> OC1M_R {
        OC1M_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:9 - Capture/Compare 2 selection
    #[inline(always)]
    pub fn cc2s(&self) -> CC2S_R {
        CC2S_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - Output Compare 2 fast enable
    #[inline(always)]
    pub fn oc2fe(&self) -> OC2FE_R {
        OC2FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Output Compare 2 preload enable
    #[inline(always)]
    pub fn oc2pe(&self) -> OC2PE_R {
        OC2PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:14 - OC2M\[2:0\]: Output Compare 2 mode
    #[inline(always)]
    pub fn oc2m(&self) -> OC2M_R {
        OC2M_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 16 - OC1M\[3\]
    #[inline(always)]
    pub fn oc1m_1(&self) -> OC1M_1_R {
        OC1M_1_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - OC2M\[3\]
    #[inline(always)]
    pub fn oc2m_1(&self) -> OC2M_1_R {
        OC2M_1_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM15_CCMR1_ALTERNATE1")
            .field("cc1s", &self.cc1s())
            .field("oc1fe", &self.oc1fe())
            .field("oc1pe", &self.oc1pe())
            .field("oc1m", &self.oc1m())
            .field("cc2s", &self.cc2s())
            .field("oc2fe", &self.oc2fe())
            .field("oc2pe", &self.oc2pe())
            .field("oc2m", &self.oc2m())
            .field("oc1m_1", &self.oc1m_1())
            .field("oc2m_1", &self.oc2m_1())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Capture/Compare 1 selection
    #[inline(always)]
    pub fn cc1s(&mut self) -> CC1S_W<TIM15_CCMR1_ALTERNATE1rs> {
        CC1S_W::new(self, 0)
    }
    ///Bit 2 - Output Compare 1 fast enable
    #[inline(always)]
    pub fn oc1fe(&mut self) -> OC1FE_W<TIM15_CCMR1_ALTERNATE1rs> {
        OC1FE_W::new(self, 2)
    }
    ///Bit 3 - Output Compare 1 preload enable
    #[inline(always)]
    pub fn oc1pe(&mut self) -> OC1PE_W<TIM15_CCMR1_ALTERNATE1rs> {
        OC1PE_W::new(self, 3)
    }
    ///Bits 4:6 - OC1M\[2:0\]: Output Compare 1 mode
    #[inline(always)]
    pub fn oc1m(&mut self) -> OC1M_W<TIM15_CCMR1_ALTERNATE1rs> {
        OC1M_W::new(self, 4)
    }
    ///Bits 8:9 - Capture/Compare 2 selection
    #[inline(always)]
    pub fn cc2s(&mut self) -> CC2S_W<TIM15_CCMR1_ALTERNATE1rs> {
        CC2S_W::new(self, 8)
    }
    ///Bit 10 - Output Compare 2 fast enable
    #[inline(always)]
    pub fn oc2fe(&mut self) -> OC2FE_W<TIM15_CCMR1_ALTERNATE1rs> {
        OC2FE_W::new(self, 10)
    }
    ///Bit 11 - Output Compare 2 preload enable
    #[inline(always)]
    pub fn oc2pe(&mut self) -> OC2PE_W<TIM15_CCMR1_ALTERNATE1rs> {
        OC2PE_W::new(self, 11)
    }
    ///Bits 12:14 - OC2M\[2:0\]: Output Compare 2 mode
    #[inline(always)]
    pub fn oc2m(&mut self) -> OC2M_W<TIM15_CCMR1_ALTERNATE1rs> {
        OC2M_W::new(self, 12)
    }
    ///Bit 16 - OC1M\[3\]
    #[inline(always)]
    pub fn oc1m_1(&mut self) -> OC1M_1_W<TIM15_CCMR1_ALTERNATE1rs> {
        OC1M_1_W::new(self, 16)
    }
    ///Bit 24 - OC2M\[3\]
    #[inline(always)]
    pub fn oc2m_1(&mut self) -> OC2M_1_W<TIM15_CCMR1_ALTERNATE1rs> {
        OC2M_1_W::new(self, 24)
    }
}
/**TIM15 capture/compare mode register 1

You can [`read`](crate::Reg::read) this register and get [`tim15_ccmr1_alternate1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_ccmr1_alternate1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#TIM15:TIM15_CCMR1_ALTERNATE1)*/
pub struct TIM15_CCMR1_ALTERNATE1rs;
impl crate::RegisterSpec for TIM15_CCMR1_ALTERNATE1rs {
    type Ux = u32;
}
///`read()` method returns [`tim15_ccmr1_alternate1::R`](R) reader structure
impl crate::Readable for TIM15_CCMR1_ALTERNATE1rs {}
///`write(|w| ..)` method takes [`tim15_ccmr1_alternate1::W`](W) writer structure
impl crate::Writable for TIM15_CCMR1_ALTERNATE1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TIM15_CCMR1_ALTERNATE1 to value 0
impl crate::Resettable for TIM15_CCMR1_ALTERNATE1rs {
    const RESET_VALUE: u32 = 0;
}