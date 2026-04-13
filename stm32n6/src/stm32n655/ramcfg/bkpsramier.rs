///Register `BKPSRAMIER` reader
pub type R = crate::R<BKPSRAMIERrs>;
///Register `BKPSRAMIER` writer
pub type W = crate::W<BKPSRAMIERrs>;
///Field `SEIE` reader - ECC single error interrupt enable
pub type SEIE_R = crate::BitReader;
///Field `SEIE` writer - ECC single error interrupt enable
pub type SEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEIE` reader - ECC double error interrupt enable
pub type DEIE_R = crate::BitReader;
///Field `DEIE` writer - ECC double error interrupt enable
pub type DEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ECC single error interrupt enable
    #[inline(always)]
    pub fn seie(&self) -> SEIE_R {
        SEIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ECC double error interrupt enable
    #[inline(always)]
    pub fn deie(&self) -> DEIE_R {
        DEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BKPSRAMIER")
            .field("seie", &self.seie())
            .field("deie", &self.deie())
            .finish()
    }
}
impl W {
    ///Bit 0 - ECC single error interrupt enable
    #[inline(always)]
    pub fn seie(&mut self) -> SEIE_W<'_, BKPSRAMIERrs> {
        SEIE_W::new(self, 0)
    }
    ///Bit 1 - ECC double error interrupt enable
    #[inline(always)]
    pub fn deie(&mut self) -> DEIE_W<'_, BKPSRAMIERrs> {
        DEIE_W::new(self, 1)
    }
}
/**RAMCFG BKPSRAM interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`bkpsramier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkpsramier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RAMCFG:BKPSRAMIER)*/
pub struct BKPSRAMIERrs;
impl crate::RegisterSpec for BKPSRAMIERrs {
    type Ux = u32;
}
///`read()` method returns [`bkpsramier::R`](R) reader structure
impl crate::Readable for BKPSRAMIERrs {}
///`write(|w| ..)` method takes [`bkpsramier::W`](W) writer structure
impl crate::Writable for BKPSRAMIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BKPSRAMIER to value 0
impl crate::Resettable for BKPSRAMIERrs {}
