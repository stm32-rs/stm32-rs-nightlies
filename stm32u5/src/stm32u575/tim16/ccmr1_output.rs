///Register `CCMR1_Output` reader
pub type R = crate::R<CCMR1_OUTPUTrs>;
///Register `CCMR1_Output` writer
pub type W = crate::W<CCMR1_OUTPUTrs>;
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
///Field `OC1M` reader - Output Compare 1 mode
pub type OC1M_R = crate::FieldReader;
///Field `OC1M` writer - Output Compare 1 mode
pub type OC1M_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `OC1CE` reader - Output Compare 1 clear enable
pub type OC1CE_R = crate::BitReader;
///Field `OC1CE` writer - Output Compare 1 clear enable
pub type OC1CE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC1M_2` reader - Output Compare 1 mode
pub type OC1M_2_R = crate::BitReader;
///Field `OC1M_2` writer - Output Compare 1 mode
pub type OC1M_2_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bits 4:6 - Output Compare 1 mode
    #[inline(always)]
    pub fn oc1m(&self) -> OC1M_R {
        OC1M_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Output Compare 1 clear enable
    #[inline(always)]
    pub fn oc1ce(&self) -> OC1CE_R {
        OC1CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - Output Compare 1 mode
    #[inline(always)]
    pub fn oc1m_2(&self) -> OC1M_2_R {
        OC1M_2_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCMR1_Output")
            .field("oc1m_2", &self.oc1m_2())
            .field("oc1ce", &self.oc1ce())
            .field("oc1m", &self.oc1m())
            .field("oc1pe", &self.oc1pe())
            .field("oc1fe", &self.oc1fe())
            .field("cc1s", &self.cc1s())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Capture/Compare 1 selection
    #[inline(always)]
    #[must_use]
    pub fn cc1s(&mut self) -> CC1S_W<CCMR1_OUTPUTrs> {
        CC1S_W::new(self, 0)
    }
    ///Bit 2 - Output Compare 1 fast enable
    #[inline(always)]
    #[must_use]
    pub fn oc1fe(&mut self) -> OC1FE_W<CCMR1_OUTPUTrs> {
        OC1FE_W::new(self, 2)
    }
    ///Bit 3 - Output Compare 1 preload enable
    #[inline(always)]
    #[must_use]
    pub fn oc1pe(&mut self) -> OC1PE_W<CCMR1_OUTPUTrs> {
        OC1PE_W::new(self, 3)
    }
    ///Bits 4:6 - Output Compare 1 mode
    #[inline(always)]
    #[must_use]
    pub fn oc1m(&mut self) -> OC1M_W<CCMR1_OUTPUTrs> {
        OC1M_W::new(self, 4)
    }
    ///Bit 7 - Output Compare 1 clear enable
    #[inline(always)]
    #[must_use]
    pub fn oc1ce(&mut self) -> OC1CE_W<CCMR1_OUTPUTrs> {
        OC1CE_W::new(self, 7)
    }
    ///Bit 16 - Output Compare 1 mode
    #[inline(always)]
    #[must_use]
    pub fn oc1m_2(&mut self) -> OC1M_2_W<CCMR1_OUTPUTrs> {
        OC1M_2_W::new(self, 16)
    }
}
/**capture/compare mode register (output mode)

You can [`read`](crate::Reg::read) this register and get [`ccmr1_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TIM16:CCMR1_Output)*/
pub struct CCMR1_OUTPUTrs;
impl crate::RegisterSpec for CCMR1_OUTPUTrs {
    type Ux = u32;
}
///`read()` method returns [`ccmr1_output::R`](R) reader structure
impl crate::Readable for CCMR1_OUTPUTrs {}
///`write(|w| ..)` method takes [`ccmr1_output::W`](W) writer structure
impl crate::Writable for CCMR1_OUTPUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCMR1_Output to value 0
impl crate::Resettable for CCMR1_OUTPUTrs {
    const RESET_VALUE: u32 = 0;
}
