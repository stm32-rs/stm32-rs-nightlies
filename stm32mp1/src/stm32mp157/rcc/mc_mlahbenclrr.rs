///Register `MC_MLAHBENCLRR` reader
pub type R = crate::R<MC_MLAHBENCLRRrs>;
///Register `MC_MLAHBENCLRR` writer
pub type W = crate::W<MC_MLAHBENCLRRrs>;
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
        f.debug_struct("MC_MLAHBENCLRR")
            .field("retramen", &self.retramen())
            .finish()
    }
}
impl W {
    ///Bit 4 - RETRAMEN
    #[inline(always)]
    pub fn retramen(&mut self) -> RETRAMEN_W<'_, MC_MLAHBENCLRRrs> {
        RETRAMEN_W::new(self, 4)
    }
}
/**This register is used to clear the peripheral clock enable bit

You can [`read`](crate::Reg::read) this register and get [`mc_mlahbenclrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_mlahbenclrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:MC_MLAHBENCLRR)*/
pub struct MC_MLAHBENCLRRrs;
impl crate::RegisterSpec for MC_MLAHBENCLRRrs {
    type Ux = u32;
}
///`read()` method returns [`mc_mlahbenclrr::R`](R) reader structure
impl crate::Readable for MC_MLAHBENCLRRrs {}
///`write(|w| ..)` method takes [`mc_mlahbenclrr::W`](W) writer structure
impl crate::Writable for MC_MLAHBENCLRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MC_MLAHBENCLRR to value 0x10
impl crate::Resettable for MC_MLAHBENCLRRrs {
    const RESET_VALUE: u32 = 0x10;
}
