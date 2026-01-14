///Register `MAC1USTCR` reader
pub type R = crate::R<MAC1USTCRrs>;
///Register `MAC1USTCR` writer
pub type W = crate::W<MAC1USTCRrs>;
///Field `TIC_1US_CNTR` reader - TIC_1US_CNTR
pub type TIC_1US_CNTR_R = crate::FieldReader<u16>;
///Field `TIC_1US_CNTR` writer - TIC_1US_CNTR
pub type TIC_1US_CNTR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - TIC_1US_CNTR
    #[inline(always)]
    pub fn tic_1us_cntr(&self) -> TIC_1US_CNTR_R {
        TIC_1US_CNTR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAC1USTCR")
            .field("tic_1us_cntr", &self.tic_1us_cntr())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - TIC_1US_CNTR
    #[inline(always)]
    pub fn tic_1us_cntr(&mut self) -> TIC_1US_CNTR_W<'_, MAC1USTCRrs> {
        TIC_1US_CNTR_W::new(self, 0)
    }
}
/**This register controls the generation of the Reference time (1-microsecond tick) for all the LPI timers. This timer has to be programmed by the software initially.

You can [`read`](crate::Reg::read) this register and get [`mac1ustcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac1ustcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MAC_MMC:MAC1USTCR)*/
pub struct MAC1USTCRrs;
impl crate::RegisterSpec for MAC1USTCRrs {
    type Ux = u32;
}
///`read()` method returns [`mac1ustcr::R`](R) reader structure
impl crate::Readable for MAC1USTCRrs {}
///`write(|w| ..)` method takes [`mac1ustcr::W`](W) writer structure
impl crate::Writable for MAC1USTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MAC1USTCR to value 0
impl crate::Resettable for MAC1USTCRrs {}
