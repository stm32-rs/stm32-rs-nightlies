///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `CCPC` reader - Capture/compare preloaded control
pub type CCPC_R = crate::BitReader;
///Field `CCPC` writer - Capture/compare preloaded control
pub type CCPC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCUS` reader - Capture/compare control update selection
pub type CCUS_R = crate::BitReader;
///Field `CCUS` writer - Capture/compare control update selection
pub type CCUS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCDS` reader - Capture/compare DMA selection
pub type CCDS_R = crate::BitReader;
///Field `CCDS` writer - Capture/compare DMA selection
pub type CCDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MMS` reader - Master mode selection
pub type MMS_R = crate::FieldReader;
///Field `MMS` writer - Master mode selection
pub type MMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TI1S` reader - TI1 selection
pub type TI1S_R = crate::BitReader;
///Field `TI1S` writer - TI1 selection
pub type TI1S_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS1` reader - Output Idle state 1
pub type OIS1_R = crate::BitReader;
///Field `OIS1` writer - Output Idle state 1
pub type OIS1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS1N` reader - Output Idle state 1
pub type OIS1N_R = crate::BitReader;
///Field `OIS1N` writer - Output Idle state 1
pub type OIS1N_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS2` reader - Output Idle state 2
pub type OIS2_R = crate::BitReader;
///Field `OIS2` writer - Output Idle state 2
pub type OIS2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS2N` reader - Output Idle state 2
pub type OIS2N_R = crate::BitReader;
///Field `OIS2N` writer - Output Idle state 2
pub type OIS2N_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS3` reader - Output Idle state 3
pub type OIS3_R = crate::BitReader;
///Field `OIS3` writer - Output Idle state 3
pub type OIS3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS3N` reader - Output Idle state 3
pub type OIS3N_R = crate::BitReader;
///Field `OIS3N` writer - Output Idle state 3
pub type OIS3N_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS4` reader - Output Idle state 4
pub type OIS4_R = crate::BitReader;
///Field `OIS4` writer - Output Idle state 4
pub type OIS4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Capture/compare preloaded control
    #[inline(always)]
    pub fn ccpc(&self) -> CCPC_R {
        CCPC_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Capture/compare control update selection
    #[inline(always)]
    pub fn ccus(&self) -> CCUS_R {
        CCUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Capture/compare DMA selection
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - Master mode selection
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - TI1 selection
    #[inline(always)]
    pub fn ti1s(&self) -> TI1S_R {
        TI1S_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Output Idle state 1
    #[inline(always)]
    pub fn ois1(&self) -> OIS1_R {
        OIS1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Output Idle state 1
    #[inline(always)]
    pub fn ois1n(&self) -> OIS1N_R {
        OIS1N_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Output Idle state 2
    #[inline(always)]
    pub fn ois2(&self) -> OIS2_R {
        OIS2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Output Idle state 2
    #[inline(always)]
    pub fn ois2n(&self) -> OIS2N_R {
        OIS2N_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Output Idle state 3
    #[inline(always)]
    pub fn ois3(&self) -> OIS3_R {
        OIS3_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Output Idle state 3
    #[inline(always)]
    pub fn ois3n(&self) -> OIS3N_R {
        OIS3N_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Output Idle state 4
    #[inline(always)]
    pub fn ois4(&self) -> OIS4_R {
        OIS4_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("ois4", &self.ois4())
            .field("ois3n", &self.ois3n())
            .field("ois3", &self.ois3())
            .field("ois2n", &self.ois2n())
            .field("ois2", &self.ois2())
            .field("ois1n", &self.ois1n())
            .field("ois1", &self.ois1())
            .field("ti1s", &self.ti1s())
            .field("mms", &self.mms())
            .field("ccds", &self.ccds())
            .field("ccus", &self.ccus())
            .field("ccpc", &self.ccpc())
            .finish()
    }
}
impl W {
    ///Bit 0 - Capture/compare preloaded control
    #[inline(always)]
    pub fn ccpc(&mut self) -> CCPC_W<'_, CR2rs> {
        CCPC_W::new(self, 0)
    }
    ///Bit 2 - Capture/compare control update selection
    #[inline(always)]
    pub fn ccus(&mut self) -> CCUS_W<'_, CR2rs> {
        CCUS_W::new(self, 2)
    }
    ///Bit 3 - Capture/compare DMA selection
    #[inline(always)]
    pub fn ccds(&mut self) -> CCDS_W<'_, CR2rs> {
        CCDS_W::new(self, 3)
    }
    ///Bits 4:6 - Master mode selection
    #[inline(always)]
    pub fn mms(&mut self) -> MMS_W<'_, CR2rs> {
        MMS_W::new(self, 4)
    }
    ///Bit 7 - TI1 selection
    #[inline(always)]
    pub fn ti1s(&mut self) -> TI1S_W<'_, CR2rs> {
        TI1S_W::new(self, 7)
    }
    ///Bit 8 - Output Idle state 1
    #[inline(always)]
    pub fn ois1(&mut self) -> OIS1_W<'_, CR2rs> {
        OIS1_W::new(self, 8)
    }
    ///Bit 9 - Output Idle state 1
    #[inline(always)]
    pub fn ois1n(&mut self) -> OIS1N_W<'_, CR2rs> {
        OIS1N_W::new(self, 9)
    }
    ///Bit 10 - Output Idle state 2
    #[inline(always)]
    pub fn ois2(&mut self) -> OIS2_W<'_, CR2rs> {
        OIS2_W::new(self, 10)
    }
    ///Bit 11 - Output Idle state 2
    #[inline(always)]
    pub fn ois2n(&mut self) -> OIS2N_W<'_, CR2rs> {
        OIS2N_W::new(self, 11)
    }
    ///Bit 12 - Output Idle state 3
    #[inline(always)]
    pub fn ois3(&mut self) -> OIS3_W<'_, CR2rs> {
        OIS3_W::new(self, 12)
    }
    ///Bit 13 - Output Idle state 3
    #[inline(always)]
    pub fn ois3n(&mut self) -> OIS3N_W<'_, CR2rs> {
        OIS3N_W::new(self, 13)
    }
    ///Bit 14 - Output Idle state 4
    #[inline(always)]
    pub fn ois4(&mut self) -> OIS4_W<'_, CR2rs> {
        OIS4_W::new(self, 14)
    }
}
/**control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#TIM1:CR2)*/
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
