///Register `BDCR2` reader
pub type R = crate::R<BDCR2rs>;
///Register `BDCR2` writer
pub type W = crate::W<BDCR2rs>;
///Field `BKPRBSEN` reader - Backup RAM backup supply enable (used to maintain BKPRAM content in Standby and Vless thansub>BATless than/sub> modes).
pub type BKPRBSEN_R = crate::BitReader;
///Field `BKPRBSEN` writer - Backup RAM backup supply enable (used to maintain BKPRAM content in Standby and Vless thansub>BATless than/sub> modes).
pub type BKPRBSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Backup RAM backup supply enable (used to maintain BKPRAM content in Standby and Vless thansub>BATless than/sub> modes).
    #[inline(always)]
    pub fn bkprbsen(&self) -> BKPRBSEN_R {
        BKPRBSEN_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BDCR2")
            .field("bkprbsen", &self.bkprbsen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Backup RAM backup supply enable (used to maintain BKPRAM content in Standby and Vless thansub>BATless than/sub> modes).
    #[inline(always)]
    pub fn bkprbsen(&mut self) -> BKPRBSEN_W<'_, BDCR2rs> {
        BKPRBSEN_W::new(self, 0)
    }
}
/**PWR backup domain control register 2

You can [`read`](crate::Reg::read) this register and get [`bdcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#PWR:BDCR2)*/
pub struct BDCR2rs;
impl crate::RegisterSpec for BDCR2rs {
    type Ux = u32;
}
///`read()` method returns [`bdcr2::R`](R) reader structure
impl crate::Readable for BDCR2rs {}
///`write(|w| ..)` method takes [`bdcr2::W`](W) writer structure
impl crate::Writable for BDCR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BDCR2 to value 0
impl crate::Resettable for BDCR2rs {}
