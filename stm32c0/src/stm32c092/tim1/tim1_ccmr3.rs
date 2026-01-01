///Register `TIM1_CCMR3` reader
pub type R = crate::R<TIM1_CCMR3rs>;
///Register `TIM1_CCMR3` writer
pub type W = crate::W<TIM1_CCMR3rs>;
///Field `OC5FE` reader - Output compare 5 fast enable Refer to OC1FE description.
pub type OC5FE_R = crate::BitReader;
///Field `OC5FE` writer - Output compare 5 fast enable Refer to OC1FE description.
pub type OC5FE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC5PE` reader - Output compare 5 preload enable Refer to OC1PE description.
pub type OC5PE_R = crate::BitReader;
///Field `OC5PE` writer - Output compare 5 preload enable Refer to OC1PE description.
pub type OC5PE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC5M` reader - OC5M\[2:0\]: Output compare 5 mode Refer to OC1M description.
pub type OC5M_R = crate::FieldReader;
///Field `OC5M` writer - OC5M\[2:0\]: Output compare 5 mode Refer to OC1M description.
pub type OC5M_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `OC5CE` reader - Output compare 5 clear enable Refer to OC1CE description.
pub type OC5CE_R = crate::BitReader;
///Field `OC5CE` writer - Output compare 5 clear enable Refer to OC1CE description.
pub type OC5CE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC6FE` reader - Output compare 6 fast enable Refer to OC1FE description.
pub type OC6FE_R = crate::BitReader;
///Field `OC6FE` writer - Output compare 6 fast enable Refer to OC1FE description.
pub type OC6FE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC6PE` reader - Output compare 6 preload enable Refer to OC1PE description.
pub type OC6PE_R = crate::BitReader;
///Field `OC6PE` writer - Output compare 6 preload enable Refer to OC1PE description.
pub type OC6PE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC6M` reader - OC6M\[2:0\]: Output compare 6 mode Refer to OC1M description.
pub type OC6M_R = crate::FieldReader;
///Field `OC6M` writer - OC6M\[2:0\]: Output compare 6 mode Refer to OC1M description.
pub type OC6M_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `OC6CE` reader - Output compare 6 clear enable Refer to OC1CE description.
pub type OC6CE_R = crate::BitReader;
///Field `OC6CE` writer - Output compare 6 clear enable Refer to OC1CE description.
pub type OC6CE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC5M_1` reader - OC5M\[3\]
pub type OC5M_1_R = crate::BitReader;
///Field `OC5M_1` writer - OC5M\[3\]
pub type OC5M_1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC6M_1` reader - OC6M\[3\]
pub type OC6M_1_R = crate::BitReader;
///Field `OC6M_1` writer - OC6M\[3\]
pub type OC6M_1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - Output compare 5 fast enable Refer to OC1FE description.
    #[inline(always)]
    pub fn oc5fe(&self) -> OC5FE_R {
        OC5FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Output compare 5 preload enable Refer to OC1PE description.
    #[inline(always)]
    pub fn oc5pe(&self) -> OC5PE_R {
        OC5PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - OC5M\[2:0\]: Output compare 5 mode Refer to OC1M description.
    #[inline(always)]
    pub fn oc5m(&self) -> OC5M_R {
        OC5M_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Output compare 5 clear enable Refer to OC1CE description.
    #[inline(always)]
    pub fn oc5ce(&self) -> OC5CE_R {
        OC5CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 10 - Output compare 6 fast enable Refer to OC1FE description.
    #[inline(always)]
    pub fn oc6fe(&self) -> OC6FE_R {
        OC6FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Output compare 6 preload enable Refer to OC1PE description.
    #[inline(always)]
    pub fn oc6pe(&self) -> OC6PE_R {
        OC6PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:14 - OC6M\[2:0\]: Output compare 6 mode Refer to OC1M description.
    #[inline(always)]
    pub fn oc6m(&self) -> OC6M_R {
        OC6M_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - Output compare 6 clear enable Refer to OC1CE description.
    #[inline(always)]
    pub fn oc6ce(&self) -> OC6CE_R {
        OC6CE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - OC5M\[3\]
    #[inline(always)]
    pub fn oc5m_1(&self) -> OC5M_1_R {
        OC5M_1_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - OC6M\[3\]
    #[inline(always)]
    pub fn oc6m_1(&self) -> OC6M_1_R {
        OC6M_1_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM1_CCMR3")
            .field("oc5fe", &self.oc5fe())
            .field("oc5pe", &self.oc5pe())
            .field("oc5m", &self.oc5m())
            .field("oc5ce", &self.oc5ce())
            .field("oc6fe", &self.oc6fe())
            .field("oc6pe", &self.oc6pe())
            .field("oc6m", &self.oc6m())
            .field("oc6ce", &self.oc6ce())
            .field("oc5m_1", &self.oc5m_1())
            .field("oc6m_1", &self.oc6m_1())
            .finish()
    }
}
impl W {
    ///Bit 2 - Output compare 5 fast enable Refer to OC1FE description.
    #[inline(always)]
    pub fn oc5fe(&mut self) -> OC5FE_W<'_, TIM1_CCMR3rs> {
        OC5FE_W::new(self, 2)
    }
    ///Bit 3 - Output compare 5 preload enable Refer to OC1PE description.
    #[inline(always)]
    pub fn oc5pe(&mut self) -> OC5PE_W<'_, TIM1_CCMR3rs> {
        OC5PE_W::new(self, 3)
    }
    ///Bits 4:6 - OC5M\[2:0\]: Output compare 5 mode Refer to OC1M description.
    #[inline(always)]
    pub fn oc5m(&mut self) -> OC5M_W<'_, TIM1_CCMR3rs> {
        OC5M_W::new(self, 4)
    }
    ///Bit 7 - Output compare 5 clear enable Refer to OC1CE description.
    #[inline(always)]
    pub fn oc5ce(&mut self) -> OC5CE_W<'_, TIM1_CCMR3rs> {
        OC5CE_W::new(self, 7)
    }
    ///Bit 10 - Output compare 6 fast enable Refer to OC1FE description.
    #[inline(always)]
    pub fn oc6fe(&mut self) -> OC6FE_W<'_, TIM1_CCMR3rs> {
        OC6FE_W::new(self, 10)
    }
    ///Bit 11 - Output compare 6 preload enable Refer to OC1PE description.
    #[inline(always)]
    pub fn oc6pe(&mut self) -> OC6PE_W<'_, TIM1_CCMR3rs> {
        OC6PE_W::new(self, 11)
    }
    ///Bits 12:14 - OC6M\[2:0\]: Output compare 6 mode Refer to OC1M description.
    #[inline(always)]
    pub fn oc6m(&mut self) -> OC6M_W<'_, TIM1_CCMR3rs> {
        OC6M_W::new(self, 12)
    }
    ///Bit 15 - Output compare 6 clear enable Refer to OC1CE description.
    #[inline(always)]
    pub fn oc6ce(&mut self) -> OC6CE_W<'_, TIM1_CCMR3rs> {
        OC6CE_W::new(self, 15)
    }
    ///Bit 16 - OC5M\[3\]
    #[inline(always)]
    pub fn oc5m_1(&mut self) -> OC5M_1_W<'_, TIM1_CCMR3rs> {
        OC5M_1_W::new(self, 16)
    }
    ///Bit 24 - OC6M\[3\]
    #[inline(always)]
    pub fn oc6m_1(&mut self) -> OC6M_1_W<'_, TIM1_CCMR3rs> {
        OC6M_1_W::new(self, 24)
    }
}
/**TIM1 capture/compare mode register 3

You can [`read`](crate::Reg::read) this register and get [`tim1_ccmr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_ccmr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM1:TIM1_CCMR3)*/
pub struct TIM1_CCMR3rs;
impl crate::RegisterSpec for TIM1_CCMR3rs {
    type Ux = u32;
}
///`read()` method returns [`tim1_ccmr3::R`](R) reader structure
impl crate::Readable for TIM1_CCMR3rs {}
///`write(|w| ..)` method takes [`tim1_ccmr3::W`](W) writer structure
impl crate::Writable for TIM1_CCMR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM1_CCMR3 to value 0
impl crate::Resettable for TIM1_CCMR3rs {}
