///Register `MC_MLAHBENSETR` reader
pub type R = crate::R<MC_MLAHBENSETRrs>;
///Register `MC_MLAHBENSETR` writer
pub type W = crate::W<MC_MLAHBENSETRrs>;
///Field `RETRAMEN` reader - RETRAMEN
pub type RETRAMEN_R = crate::BitReader;
///Field `RETRAMEN` writer - RETRAMEN
pub type RETRAMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 4 - RETRAMEN
    #[inline(always)]
    pub fn retramen(&self) -> RETRAMEN_R {
        RETRAMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MC_MLAHBENSETR")
            .field("retramen", &self.retramen())
            .finish()
    }
}
impl W {
    ///Bit 4 - RETRAMEN
    #[inline(always)]
    pub fn retramen(&mut self) -> RETRAMEN_W<'_, MC_MLAHBENSETRrs> {
        RETRAMEN_W::new(self, 4)
    }
}
/**This register is used to set the peripheral clock enable bit

You can [`read`](crate::Reg::read) this register and get [`mc_mlahbensetr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_mlahbensetr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_MLAHBENSETR)*/
pub struct MC_MLAHBENSETRrs;
impl crate::RegisterSpec for MC_MLAHBENSETRrs {
    type Ux = u32;
}
///`read()` method returns [`mc_mlahbensetr::R`](R) reader structure
impl crate::Readable for MC_MLAHBENSETRrs {}
///`write(|w| ..)` method takes [`mc_mlahbensetr::W`](W) writer structure
impl crate::Writable for MC_MLAHBENSETRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MC_MLAHBENSETR to value 0x10
impl crate::Resettable for MC_MLAHBENSETRrs {
    const RESET_VALUE: u32 = 0x10;
}
