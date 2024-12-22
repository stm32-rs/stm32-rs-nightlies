///Register `CCMR2_Output` reader
pub type R = crate::R<CCMR2_OUTPUTrs>;
///Register `CCMR2_Output` writer
pub type W = crate::W<CCMR2_OUTPUTrs>;
///Field `CC3S_1_0` reader - Capture/Compare 3 selection
pub type CC3S_1_0_R = crate::FieldReader;
///Field `CC3S_1_0` writer - Capture/Compare 3 selection
pub type CC3S_1_0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OC3FE` reader - Output compare 3 fast enable
pub type OC3FE_R = crate::BitReader;
///Field `OC3FE` writer - Output compare 3 fast enable
pub type OC3FE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC3PE` reader - Output compare 3 preload enable
pub type OC3PE_R = crate::BitReader;
///Field `OC3PE` writer - Output compare 3 preload enable
pub type OC3PE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC3M_2_0` reader - Output compare 3 mode
pub type OC3M_2_0_R = crate::FieldReader;
///Field `OC3M_2_0` writer - Output compare 3 mode
pub type OC3M_2_0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `OC3CE` reader - Output compare 3 clear enable
pub type OC3CE_R = crate::BitReader;
///Field `OC3CE` writer - Output compare 3 clear enable
pub type OC3CE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4S_1_0` reader - Capture/Compare 4 selection
pub type CC4S_1_0_R = crate::FieldReader;
///Field `CC4S_1_0` writer - Capture/Compare 4 selection
pub type CC4S_1_0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OC4FE` reader - Output compare 4 fast enable
pub type OC4FE_R = crate::BitReader;
///Field `OC4FE` writer - Output compare 4 fast enable
pub type OC4FE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC4PE` reader - Output compare 4 preload enable
pub type OC4PE_R = crate::BitReader;
///Field `OC4PE` writer - Output compare 4 preload enable
pub type OC4PE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC4M_3_0` reader - Output compare 4 mode
pub type OC4M_3_0_R = crate::FieldReader;
///Field `OC4M_3_0` writer - Output compare 4 mode
pub type OC4M_3_0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `OC4CE` reader - Output compare 4 clear enable
pub type OC4CE_R = crate::BitReader;
///Field `OC4CE` writer - Output compare 4 clear enable
pub type OC4CE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC3M_3` reader - Output compare 3 mode
pub type OC3M_3_R = crate::BitReader;
///Field `OC3M_3` writer - Output compare 3 mode
pub type OC3M_3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC4M_bit3` reader - Output Compare 4 mode - bit 3
pub type OC4M_BIT3_R = crate::BitReader;
///Field `OC4M_bit3` writer - Output Compare 4 mode - bit 3
pub type OC4M_BIT3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Capture/Compare 3 selection
    #[inline(always)]
    pub fn cc3s_1_0(&self) -> CC3S_1_0_R {
        CC3S_1_0_R::new((self.bits & 3) as u8)
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
    ///Bits 4:6 - Output compare 3 mode
    #[inline(always)]
    pub fn oc3m_2_0(&self) -> OC3M_2_0_R {
        OC3M_2_0_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Output compare 3 clear enable
    #[inline(always)]
    pub fn oc3ce(&self) -> OC3CE_R {
        OC3CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - Capture/Compare 4 selection
    #[inline(always)]
    pub fn cc4s_1_0(&self) -> CC4S_1_0_R {
        CC4S_1_0_R::new(((self.bits >> 8) & 3) as u8)
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
    ///Bits 12:14 - Output compare 4 mode
    #[inline(always)]
    pub fn oc4m_3_0(&self) -> OC4M_3_0_R {
        OC4M_3_0_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - Output compare 4 clear enable
    #[inline(always)]
    pub fn oc4ce(&self) -> OC4CE_R {
        OC4CE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Output compare 3 mode
    #[inline(always)]
    pub fn oc3m_3(&self) -> OC3M_3_R {
        OC3M_3_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - Output Compare 4 mode - bit 3
    #[inline(always)]
    pub fn oc4m_bit3(&self) -> OC4M_BIT3_R {
        OC4M_BIT3_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCMR2_Output")
            .field("oc4m_bit3", &self.oc4m_bit3())
            .field("oc3m_3", &self.oc3m_3())
            .field("oc4ce", &self.oc4ce())
            .field("oc4m_3_0", &self.oc4m_3_0())
            .field("oc4pe", &self.oc4pe())
            .field("oc4fe", &self.oc4fe())
            .field("cc4s_1_0", &self.cc4s_1_0())
            .field("oc3ce", &self.oc3ce())
            .field("oc3m_2_0", &self.oc3m_2_0())
            .field("oc3pe", &self.oc3pe())
            .field("oc3fe", &self.oc3fe())
            .field("cc3s_1_0", &self.cc3s_1_0())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Capture/Compare 3 selection
    #[inline(always)]
    pub fn cc3s_1_0(&mut self) -> CC3S_1_0_W<CCMR2_OUTPUTrs> {
        CC3S_1_0_W::new(self, 0)
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
    ///Bits 4:6 - Output compare 3 mode
    #[inline(always)]
    pub fn oc3m_2_0(&mut self) -> OC3M_2_0_W<CCMR2_OUTPUTrs> {
        OC3M_2_0_W::new(self, 4)
    }
    ///Bit 7 - Output compare 3 clear enable
    #[inline(always)]
    pub fn oc3ce(&mut self) -> OC3CE_W<CCMR2_OUTPUTrs> {
        OC3CE_W::new(self, 7)
    }
    ///Bits 8:9 - Capture/Compare 4 selection
    #[inline(always)]
    pub fn cc4s_1_0(&mut self) -> CC4S_1_0_W<CCMR2_OUTPUTrs> {
        CC4S_1_0_W::new(self, 8)
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
    ///Bits 12:14 - Output compare 4 mode
    #[inline(always)]
    pub fn oc4m_3_0(&mut self) -> OC4M_3_0_W<CCMR2_OUTPUTrs> {
        OC4M_3_0_W::new(self, 12)
    }
    ///Bit 15 - Output compare 4 clear enable
    #[inline(always)]
    pub fn oc4ce(&mut self) -> OC4CE_W<CCMR2_OUTPUTrs> {
        OC4CE_W::new(self, 15)
    }
    ///Bit 16 - Output compare 3 mode
    #[inline(always)]
    pub fn oc3m_3(&mut self) -> OC3M_3_W<CCMR2_OUTPUTrs> {
        OC3M_3_W::new(self, 16)
    }
    ///Bit 24 - Output Compare 4 mode - bit 3
    #[inline(always)]
    pub fn oc4m_bit3(&mut self) -> OC4M_BIT3_W<CCMR2_OUTPUTrs> {
        OC4M_BIT3_W::new(self, 24)
    }
}
/**capture/compare mode register 2 (output mode)

You can [`read`](crate::Reg::read) this register and get [`ccmr2_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#TIM1:CCMR2_Output)*/
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
///`reset()` method sets CCMR2_Output to value 0
impl crate::Resettable for CCMR2_OUTPUTrs {
    const RESET_VALUE: u32 = 0;
}
