///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `CCPC` reader - CCPC
pub type CCPC_R = crate::BitReader;
///Field `CCPC` writer - CCPC
pub type CCPC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCUS` reader - CCUS
pub type CCUS_R = crate::BitReader;
///Field `CCUS` writer - CCUS
pub type CCUS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCDS` reader - CCDS
pub type CCDS_R = crate::BitReader;
///Field `CCDS` writer - CCDS
pub type CCDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MMS` reader - MMS
pub type MMS_R = crate::FieldReader;
///Field `MMS` writer - MMS
pub type MMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TI1S` reader - TI1S
pub type TI1S_R = crate::BitReader;
///Field `TI1S` writer - TI1S
pub type TI1S_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS1` reader - OIS1
pub type OIS1_R = crate::BitReader;
///Field `OIS1` writer - OIS1
pub type OIS1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS1N` reader - OIS1N
pub type OIS1N_R = crate::BitReader;
///Field `OIS1N` writer - OIS1N
pub type OIS1N_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS2` reader - OIS2
pub type OIS2_R = crate::BitReader;
///Field `OIS2` writer - OIS2
pub type OIS2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS2N` reader - OIS2N
pub type OIS2N_R = crate::BitReader;
///Field `OIS2N` writer - OIS2N
pub type OIS2N_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS3` reader - OIS3
pub type OIS3_R = crate::BitReader;
///Field `OIS3` writer - OIS3
pub type OIS3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS3N` reader - OIS3N
pub type OIS3N_R = crate::BitReader;
///Field `OIS3N` writer - OIS3N
pub type OIS3N_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS4` reader - OIS4
pub type OIS4_R = crate::BitReader;
///Field `OIS4` writer - OIS4
pub type OIS4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS5` reader - OIS5
pub type OIS5_R = crate::BitReader;
///Field `OIS5` writer - OIS5
pub type OIS5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS6` reader - OIS6
pub type OIS6_R = crate::BitReader;
///Field `OIS6` writer - OIS6
pub type OIS6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MMS2` reader - MMS2
pub type MMS2_R = crate::FieldReader;
///Field `MMS2` writer - MMS2
pub type MMS2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - CCPC
    #[inline(always)]
    pub fn ccpc(&self) -> CCPC_R {
        CCPC_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - CCUS
    #[inline(always)]
    pub fn ccus(&self) -> CCUS_R {
        CCUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CCDS
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - MMS
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - TI1S
    #[inline(always)]
    pub fn ti1s(&self) -> TI1S_R {
        TI1S_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - OIS1
    #[inline(always)]
    pub fn ois1(&self) -> OIS1_R {
        OIS1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - OIS1N
    #[inline(always)]
    pub fn ois1n(&self) -> OIS1N_R {
        OIS1N_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - OIS2
    #[inline(always)]
    pub fn ois2(&self) -> OIS2_R {
        OIS2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - OIS2N
    #[inline(always)]
    pub fn ois2n(&self) -> OIS2N_R {
        OIS2N_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - OIS3
    #[inline(always)]
    pub fn ois3(&self) -> OIS3_R {
        OIS3_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - OIS3N
    #[inline(always)]
    pub fn ois3n(&self) -> OIS3N_R {
        OIS3N_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - OIS4
    #[inline(always)]
    pub fn ois4(&self) -> OIS4_R {
        OIS4_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - OIS5
    #[inline(always)]
    pub fn ois5(&self) -> OIS5_R {
        OIS5_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - OIS6
    #[inline(always)]
    pub fn ois6(&self) -> OIS6_R {
        OIS6_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 20:23 - MMS2
    #[inline(always)]
    pub fn mms2(&self) -> MMS2_R {
        MMS2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("ccpc", &self.ccpc())
            .field("ccus", &self.ccus())
            .field("ccds", &self.ccds())
            .field("mms", &self.mms())
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
            .field("mms2", &self.mms2())
            .finish()
    }
}
impl W {
    ///Bit 0 - CCPC
    #[inline(always)]
    pub fn ccpc(&mut self) -> CCPC_W<'_, CR2rs> {
        CCPC_W::new(self, 0)
    }
    ///Bit 2 - CCUS
    #[inline(always)]
    pub fn ccus(&mut self) -> CCUS_W<'_, CR2rs> {
        CCUS_W::new(self, 2)
    }
    ///Bit 3 - CCDS
    #[inline(always)]
    pub fn ccds(&mut self) -> CCDS_W<'_, CR2rs> {
        CCDS_W::new(self, 3)
    }
    ///Bits 4:6 - MMS
    #[inline(always)]
    pub fn mms(&mut self) -> MMS_W<'_, CR2rs> {
        MMS_W::new(self, 4)
    }
    ///Bit 7 - TI1S
    #[inline(always)]
    pub fn ti1s(&mut self) -> TI1S_W<'_, CR2rs> {
        TI1S_W::new(self, 7)
    }
    ///Bit 8 - OIS1
    #[inline(always)]
    pub fn ois1(&mut self) -> OIS1_W<'_, CR2rs> {
        OIS1_W::new(self, 8)
    }
    ///Bit 9 - OIS1N
    #[inline(always)]
    pub fn ois1n(&mut self) -> OIS1N_W<'_, CR2rs> {
        OIS1N_W::new(self, 9)
    }
    ///Bit 10 - OIS2
    #[inline(always)]
    pub fn ois2(&mut self) -> OIS2_W<'_, CR2rs> {
        OIS2_W::new(self, 10)
    }
    ///Bit 11 - OIS2N
    #[inline(always)]
    pub fn ois2n(&mut self) -> OIS2N_W<'_, CR2rs> {
        OIS2N_W::new(self, 11)
    }
    ///Bit 12 - OIS3
    #[inline(always)]
    pub fn ois3(&mut self) -> OIS3_W<'_, CR2rs> {
        OIS3_W::new(self, 12)
    }
    ///Bit 13 - OIS3N
    #[inline(always)]
    pub fn ois3n(&mut self) -> OIS3N_W<'_, CR2rs> {
        OIS3N_W::new(self, 13)
    }
    ///Bit 14 - OIS4
    #[inline(always)]
    pub fn ois4(&mut self) -> OIS4_W<'_, CR2rs> {
        OIS4_W::new(self, 14)
    }
    ///Bit 16 - OIS5
    #[inline(always)]
    pub fn ois5(&mut self) -> OIS5_W<'_, CR2rs> {
        OIS5_W::new(self, 16)
    }
    ///Bit 18 - OIS6
    #[inline(always)]
    pub fn ois6(&mut self) -> OIS6_W<'_, CR2rs> {
        OIS6_W::new(self, 18)
    }
    ///Bits 20:23 - MMS2
    #[inline(always)]
    pub fn mms2(&mut self) -> MMS2_W<'_, CR2rs> {
        MMS2_W::new(self, 20)
    }
}
/**TIM2 control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM2:CR2)*/
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
