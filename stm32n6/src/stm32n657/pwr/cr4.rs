///Register `CR4` reader
pub type R = crate::R<CR4rs>;
///Register `CR4` writer
pub type W = crate::W<CR4rs>;
///Field `TCMRBSEN` reader - I-TCM and D-TCM RAMs backup supply enable (used to maintain TCM RAMs content in Standby mode)
pub type TCMRBSEN_R = crate::BitReader;
///Field `TCMRBSEN` writer - I-TCM and D-TCM RAMs backup supply enable (used to maintain TCM RAMs content in Standby mode)
pub type TCMRBSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCMFLXRBSEN` reader - I-TCM FLEXMEM backup supply enable (used to maintain I-TCM FLEX MEM content in Standby mode)
pub type TCMFLXRBSEN_R = crate::BitReader;
///Field `TCMFLXRBSEN` writer - I-TCM FLEXMEM backup supply enable (used to maintain I-TCM FLEX MEM content in Standby mode)
pub type TCMFLXRBSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - I-TCM and D-TCM RAMs backup supply enable (used to maintain TCM RAMs content in Standby mode)
    #[inline(always)]
    pub fn tcmrbsen(&self) -> TCMRBSEN_R {
        TCMRBSEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - I-TCM FLEXMEM backup supply enable (used to maintain I-TCM FLEX MEM content in Standby mode)
    #[inline(always)]
    pub fn tcmflxrbsen(&self) -> TCMFLXRBSEN_R {
        TCMFLXRBSEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR4")
            .field("tcmrbsen", &self.tcmrbsen())
            .field("tcmflxrbsen", &self.tcmflxrbsen())
            .finish()
    }
}
impl W {
    ///Bit 0 - I-TCM and D-TCM RAMs backup supply enable (used to maintain TCM RAMs content in Standby mode)
    #[inline(always)]
    pub fn tcmrbsen(&mut self) -> TCMRBSEN_W<'_, CR4rs> {
        TCMRBSEN_W::new(self, 0)
    }
    ///Bit 4 - I-TCM FLEXMEM backup supply enable (used to maintain I-TCM FLEX MEM content in Standby mode)
    #[inline(always)]
    pub fn tcmflxrbsen(&mut self) -> TCMFLXRBSEN_W<'_, CR4rs> {
        TCMFLXRBSEN_W::new(self, 4)
    }
}
/**PWR control register 4

You can [`read`](crate::Reg::read) this register and get [`cr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#PWR:CR4)*/
pub struct CR4rs;
impl crate::RegisterSpec for CR4rs {
    type Ux = u32;
}
///`read()` method returns [`cr4::R`](R) reader structure
impl crate::Readable for CR4rs {}
///`write(|w| ..)` method takes [`cr4::W`](W) writer structure
impl crate::Writable for CR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR4 to value 0
impl crate::Resettable for CR4rs {}
