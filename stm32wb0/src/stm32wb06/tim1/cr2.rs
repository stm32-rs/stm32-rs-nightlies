///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `CCPC` reader - Capture/compare preloaded control.
pub type CCPC_R = crate::BitReader;
///Field `CCPC` writer - Capture/compare preloaded control.
pub type CCPC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCUS` reader - Capture/compare control update selection.
pub type CCUS_R = crate::BitReader;
///Field `CCUS` writer - Capture/compare control update selection.
pub type CCUS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TI1S` reader - TI1S: TI1 selection 0: The TIMx_CH1 pin is connected to TI1 input. 1: The TIMx_CH1, CH2 and CH3 pins are connected to the TI1 input (XOR combination)
pub type TI1S_R = crate::BitReader;
///Field `TI1S` writer - TI1S: TI1 selection 0: The TIMx_CH1 pin is connected to TI1 input. 1: The TIMx_CH1, CH2 and CH3 pins are connected to the TI1 input (XOR combination)
pub type TI1S_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS1` reader - Output idle state 1 (OC1 output).
pub type OIS1_R = crate::BitReader;
///Field `OIS1` writer - Output idle state 1 (OC1 output).
pub type OIS1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS1N` reader - Output idle state 1 (OC1N output).
pub type OIS1N_R = crate::BitReader;
///Field `OIS1N` writer - Output idle state 1 (OC1N output).
pub type OIS1N_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS2` reader - Output idle state 2 (OC2 output). Refer to OIS1 bit.
pub type OIS2_R = crate::BitReader;
///Field `OIS2` writer - Output idle state 2 (OC2 output). Refer to OIS1 bit.
pub type OIS2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS2N` reader - Output idle state 2 (OC2N output). Refer to OIS1N bit.
pub type OIS2N_R = crate::BitReader;
///Field `OIS2N` writer - Output idle state 2 (OC2N output). Refer to OIS1N bit.
pub type OIS2N_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS3` reader - Output idle state 3 (OC3 output). Refer to OIS1 bit.
pub type OIS3_R = crate::BitReader;
///Field `OIS3` writer - Output idle state 3 (OC3 output). Refer to OIS1 bit.
pub type OIS3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS3N` reader - Output idle state 3 (OC3N output). Refer to OIS1N bit.
pub type OIS3N_R = crate::BitReader;
///Field `OIS3N` writer - Output idle state 3 (OC3N output). Refer to OIS1N bit.
pub type OIS3N_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS4` reader - Output idle state 4 (OC4 output). Refer to OIS1 bit.
pub type OIS4_R = crate::BitReader;
///Field `OIS4` writer - Output idle state 4 (OC4 output). Refer to OIS1 bit.
pub type OIS4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS5` reader - Output idle state 5 (OC5 output). Refer to OIS1 bit.
pub type OIS5_R = crate::BitReader;
///Field `OIS5` writer - Output idle state 5 (OC5 output). Refer to OIS1 bit.
pub type OIS5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS6` reader - Output idle state 6 (OC6 output). Refer to OIS1 bit.
pub type OIS6_R = crate::BitReader;
///Field `OIS6` writer - Output idle state 6 (OC6 output). Refer to OIS1 bit.
pub type OIS6_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Capture/compare preloaded control.
    #[inline(always)]
    pub fn ccpc(&self) -> CCPC_R {
        CCPC_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Capture/compare control update selection.
    #[inline(always)]
    pub fn ccus(&self) -> CCUS_R {
        CCUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 7 - TI1S: TI1 selection 0: The TIMx_CH1 pin is connected to TI1 input. 1: The TIMx_CH1, CH2 and CH3 pins are connected to the TI1 input (XOR combination)
    #[inline(always)]
    pub fn ti1s(&self) -> TI1S_R {
        TI1S_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Output idle state 1 (OC1 output).
    #[inline(always)]
    pub fn ois1(&self) -> OIS1_R {
        OIS1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Output idle state 1 (OC1N output).
    #[inline(always)]
    pub fn ois1n(&self) -> OIS1N_R {
        OIS1N_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Output idle state 2 (OC2 output). Refer to OIS1 bit.
    #[inline(always)]
    pub fn ois2(&self) -> OIS2_R {
        OIS2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Output idle state 2 (OC2N output). Refer to OIS1N bit.
    #[inline(always)]
    pub fn ois2n(&self) -> OIS2N_R {
        OIS2N_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Output idle state 3 (OC3 output). Refer to OIS1 bit.
    #[inline(always)]
    pub fn ois3(&self) -> OIS3_R {
        OIS3_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Output idle state 3 (OC3N output). Refer to OIS1N bit.
    #[inline(always)]
    pub fn ois3n(&self) -> OIS3N_R {
        OIS3N_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Output idle state 4 (OC4 output). Refer to OIS1 bit.
    #[inline(always)]
    pub fn ois4(&self) -> OIS4_R {
        OIS4_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Output idle state 5 (OC5 output). Refer to OIS1 bit.
    #[inline(always)]
    pub fn ois5(&self) -> OIS5_R {
        OIS5_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Output idle state 6 (OC6 output). Refer to OIS1 bit.
    #[inline(always)]
    pub fn ois6(&self) -> OIS6_R {
        OIS6_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("ccpc", &self.ccpc())
            .field("ccus", &self.ccus())
            .field("ti1s", &self.ti1s())
            .field("ois1", &self.ois1())
            .field("ois1n", &self.ois1n())
            .field("ois2", &self.ois2())
            .field("ois2n", &self.ois2n())
            .field("ois3", &self.ois3())
            .field("ois3n", &self.ois3n())
            .field("ois4", &self.ois4())
            .field("ois5", &self.ois5())
            .field("ois6", &self.ois6())
            .finish()
    }
}
impl W {
    ///Bit 0 - Capture/compare preloaded control.
    #[inline(always)]
    pub fn ccpc(&mut self) -> CCPC_W<CR2rs> {
        CCPC_W::new(self, 0)
    }
    ///Bit 2 - Capture/compare control update selection.
    #[inline(always)]
    pub fn ccus(&mut self) -> CCUS_W<CR2rs> {
        CCUS_W::new(self, 2)
    }
    ///Bit 7 - TI1S: TI1 selection 0: The TIMx_CH1 pin is connected to TI1 input. 1: The TIMx_CH1, CH2 and CH3 pins are connected to the TI1 input (XOR combination)
    #[inline(always)]
    pub fn ti1s(&mut self) -> TI1S_W<CR2rs> {
        TI1S_W::new(self, 7)
    }
    ///Bit 8 - Output idle state 1 (OC1 output).
    #[inline(always)]
    pub fn ois1(&mut self) -> OIS1_W<CR2rs> {
        OIS1_W::new(self, 8)
    }
    ///Bit 9 - Output idle state 1 (OC1N output).
    #[inline(always)]
    pub fn ois1n(&mut self) -> OIS1N_W<CR2rs> {
        OIS1N_W::new(self, 9)
    }
    ///Bit 10 - Output idle state 2 (OC2 output). Refer to OIS1 bit.
    #[inline(always)]
    pub fn ois2(&mut self) -> OIS2_W<CR2rs> {
        OIS2_W::new(self, 10)
    }
    ///Bit 11 - Output idle state 2 (OC2N output). Refer to OIS1N bit.
    #[inline(always)]
    pub fn ois2n(&mut self) -> OIS2N_W<CR2rs> {
        OIS2N_W::new(self, 11)
    }
    ///Bit 12 - Output idle state 3 (OC3 output). Refer to OIS1 bit.
    #[inline(always)]
    pub fn ois3(&mut self) -> OIS3_W<CR2rs> {
        OIS3_W::new(self, 12)
    }
    ///Bit 13 - Output idle state 3 (OC3N output). Refer to OIS1N bit.
    #[inline(always)]
    pub fn ois3n(&mut self) -> OIS3N_W<CR2rs> {
        OIS3N_W::new(self, 13)
    }
    ///Bit 14 - Output idle state 4 (OC4 output). Refer to OIS1 bit.
    #[inline(always)]
    pub fn ois4(&mut self) -> OIS4_W<CR2rs> {
        OIS4_W::new(self, 14)
    }
    ///Bit 16 - Output idle state 5 (OC5 output). Refer to OIS1 bit.
    #[inline(always)]
    pub fn ois5(&mut self) -> OIS5_W<CR2rs> {
        OIS5_W::new(self, 16)
    }
    ///Bit 18 - Output idle state 6 (OC6 output). Refer to OIS1 bit.
    #[inline(always)]
    pub fn ois6(&mut self) -> OIS6_W<CR2rs> {
        OIS6_W::new(self, 18)
    }
}
/**CR2 register

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#TIM1:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
