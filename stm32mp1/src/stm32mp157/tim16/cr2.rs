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
///Field `OIS1` reader - OIS1
pub type OIS1_R = crate::BitReader;
///Field `OIS1` writer - OIS1
pub type OIS1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS1N` reader - OIS1N
pub type OIS1N_R = crate::BitReader;
///Field `OIS1N` writer - OIS1N
pub type OIS1N_W<'a, REG> = crate::BitWriter<'a, REG>;
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("ccpc", &self.ccpc())
            .field("ccus", &self.ccus())
            .field("ccds", &self.ccds())
            .field("ois1", &self.ois1())
            .field("ois1n", &self.ois1n())
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
}
/**TIM16/TIM17 control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM16:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u16;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
