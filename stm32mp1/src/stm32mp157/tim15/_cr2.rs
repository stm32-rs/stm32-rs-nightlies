///Register `_CR2` reader
pub type R = crate::R<_CR2rs>;
///Register `_CR2` writer
pub type W = crate::W<_CR2rs>;
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_CR2")
            .field("ccpc", &self.ccpc())
            .field("ccus", &self.ccus())
            .field("ccds", &self.ccds())
            .field("mms", &self.mms())
            .field("ti1s", &self.ti1s())
            .field("ois1", &self.ois1())
            .field("ois1n", &self.ois1n())
            .field("ois2", &self.ois2())
            .finish()
    }
}
impl W {
    ///Bit 0 - CCPC
    #[inline(always)]
    pub fn ccpc(&mut self) -> CCPC_W<'_, _CR2rs> {
        CCPC_W::new(self, 0)
    }
    ///Bit 2 - CCUS
    #[inline(always)]
    pub fn ccus(&mut self) -> CCUS_W<'_, _CR2rs> {
        CCUS_W::new(self, 2)
    }
    ///Bit 3 - CCDS
    #[inline(always)]
    pub fn ccds(&mut self) -> CCDS_W<'_, _CR2rs> {
        CCDS_W::new(self, 3)
    }
    ///Bits 4:6 - MMS
    #[inline(always)]
    pub fn mms(&mut self) -> MMS_W<'_, _CR2rs> {
        MMS_W::new(self, 4)
    }
    ///Bit 7 - TI1S
    #[inline(always)]
    pub fn ti1s(&mut self) -> TI1S_W<'_, _CR2rs> {
        TI1S_W::new(self, 7)
    }
    ///Bit 8 - OIS1
    #[inline(always)]
    pub fn ois1(&mut self) -> OIS1_W<'_, _CR2rs> {
        OIS1_W::new(self, 8)
    }
    ///Bit 9 - OIS1N
    #[inline(always)]
    pub fn ois1n(&mut self) -> OIS1N_W<'_, _CR2rs> {
        OIS1N_W::new(self, 9)
    }
    ///Bit 10 - OIS2
    #[inline(always)]
    pub fn ois2(&mut self) -> OIS2_W<'_, _CR2rs> {
        OIS2_W::new(self, 10)
    }
}
/**TIM15 control register 2

You can [`read`](crate::Reg::read) this register and get [`_cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM15:_CR2)*/
pub struct _CR2rs;
impl crate::RegisterSpec for _CR2rs {
    type Ux = u16;
}
///`read()` method returns [`_cr2::R`](R) reader structure
impl crate::Readable for _CR2rs {}
///`write(|w| ..)` method takes [`_cr2::W`](W) writer structure
impl crate::Writable for _CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets _CR2 to value 0
impl crate::Resettable for _CR2rs {}
