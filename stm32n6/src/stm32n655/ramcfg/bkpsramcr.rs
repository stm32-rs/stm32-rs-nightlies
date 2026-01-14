///Register `BKPSRAMCR` reader
pub type R = crate::R<BKPSRAMCRrs>;
///Register `BKPSRAMCR` writer
pub type W = crate::W<BKPSRAMCRrs>;
///Field `ECCE` reader - ECC enable
pub type ECCE_R = crate::BitReader;
///Field `ECCE` writer - ECC enable
pub type ECCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALE` reader - Address latch enable
pub type ALE_R = crate::BitReader;
///Field `ALE` writer - Address latch enable
pub type ALE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAMER` reader - SRAM erase
pub type SRAMER_R = crate::BitReader;
///Field `SRAMER` writer - SRAM erase
pub type SRAMER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ECC enable
    #[inline(always)]
    pub fn ecce(&self) -> ECCE_R {
        ECCE_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Address latch enable
    #[inline(always)]
    pub fn ale(&self) -> ALE_R {
        ALE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - SRAM erase
    #[inline(always)]
    pub fn sramer(&self) -> SRAMER_R {
        SRAMER_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BKPSRAMCR")
            .field("ecce", &self.ecce())
            .field("ale", &self.ale())
            .field("sramer", &self.sramer())
            .finish()
    }
}
impl W {
    ///Bit 0 - ECC enable
    #[inline(always)]
    pub fn ecce(&mut self) -> ECCE_W<'_, BKPSRAMCRrs> {
        ECCE_W::new(self, 0)
    }
    ///Bit 4 - Address latch enable
    #[inline(always)]
    pub fn ale(&mut self) -> ALE_W<'_, BKPSRAMCRrs> {
        ALE_W::new(self, 4)
    }
    ///Bit 8 - SRAM erase
    #[inline(always)]
    pub fn sramer(&mut self) -> SRAMER_W<'_, BKPSRAMCRrs> {
        SRAMER_W::new(self, 8)
    }
}
/**RAMCFG BKPSRAM control register

You can [`read`](crate::Reg::read) this register and get [`bkpsramcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkpsramcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RAMCFG:BKPSRAMCR)*/
pub struct BKPSRAMCRrs;
impl crate::RegisterSpec for BKPSRAMCRrs {
    type Ux = u32;
}
///`read()` method returns [`bkpsramcr::R`](R) reader structure
impl crate::Readable for BKPSRAMCRrs {}
///`write(|w| ..)` method takes [`bkpsramcr::W`](W) writer structure
impl crate::Writable for BKPSRAMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BKPSRAMCR to value 0
impl crate::Resettable for BKPSRAMCRrs {}
