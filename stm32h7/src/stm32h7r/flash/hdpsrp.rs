///Register `HDPSRP` reader
pub type R = crate::R<HDPSRPrs>;
///Register `HDPSRP` writer
pub type W = crate::W<HDPSRPrs>;
///Field `HDP_AREA_START` reader - Hide protection user Flash area start programming Write to change corresponding option byte bits in FLASH_HDPSR. If HDP_AREA_END=HDP_AREA_START all the sectors are protected. If HDP_AREA_END<HDP_AREA_START no sectors are protected.
pub type HDP_AREA_START_R = crate::FieldReader<u16>;
///Field `HDP_AREA_START` writer - Hide protection user Flash area start programming Write to change corresponding option byte bits in FLASH_HDPSR. If HDP_AREA_END=HDP_AREA_START all the sectors are protected. If HDP_AREA_END<HDP_AREA_START no sectors are protected.
pub type HDP_AREA_START_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `HDP_AREA_END` reader - Hide protection user Flash area end programming Write to change corresponding option byte bits in FLASH_HDPSR. If HDP_AREA_END=HDP_AREA_START all the sectors are protected. If HDP_AREA_END<HDP_AREA_START no sectors are protected.
pub type HDP_AREA_END_R = crate::FieldReader<u16>;
///Field `HDP_AREA_END` writer - Hide protection user Flash area end programming Write to change corresponding option byte bits in FLASH_HDPSR. If HDP_AREA_END=HDP_AREA_START all the sectors are protected. If HDP_AREA_END<HDP_AREA_START no sectors are protected.
pub type HDP_AREA_END_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Hide protection user Flash area start programming Write to change corresponding option byte bits in FLASH_HDPSR. If HDP_AREA_END=HDP_AREA_START all the sectors are protected. If HDP_AREA_END<HDP_AREA_START no sectors are protected.
    #[inline(always)]
    pub fn hdp_area_start(&self) -> HDP_AREA_START_R {
        HDP_AREA_START_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Hide protection user Flash area end programming Write to change corresponding option byte bits in FLASH_HDPSR. If HDP_AREA_END=HDP_AREA_START all the sectors are protected. If HDP_AREA_END<HDP_AREA_START no sectors are protected.
    #[inline(always)]
    pub fn hdp_area_end(&self) -> HDP_AREA_END_R {
        HDP_AREA_END_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HDPSRP")
            .field("hdp_area_start", &self.hdp_area_start())
            .field("hdp_area_end", &self.hdp_area_end())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Hide protection user Flash area start programming Write to change corresponding option byte bits in FLASH_HDPSR. If HDP_AREA_END=HDP_AREA_START all the sectors are protected. If HDP_AREA_END<HDP_AREA_START no sectors are protected.
    #[inline(always)]
    pub fn hdp_area_start(&mut self) -> HDP_AREA_START_W<'_, HDPSRPrs> {
        HDP_AREA_START_W::new(self, 0)
    }
    ///Bits 16:24 - Hide protection user Flash area end programming Write to change corresponding option byte bits in FLASH_HDPSR. If HDP_AREA_END=HDP_AREA_START all the sectors are protected. If HDP_AREA_END<HDP_AREA_START no sectors are protected.
    #[inline(always)]
    pub fn hdp_area_end(&mut self) -> HDP_AREA_END_W<'_, HDPSRPrs> {
        HDP_AREA_END_W::new(self, 16)
    }
}
/**FLASH hide protection status register programming

You can [`read`](crate::Reg::read) this register and get [`hdpsrp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdpsrp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:HDPSRP)*/
pub struct HDPSRPrs;
impl crate::RegisterSpec for HDPSRPrs {
    type Ux = u32;
}
///`read()` method returns [`hdpsrp::R`](R) reader structure
impl crate::Readable for HDPSRPrs {}
///`write(|w| ..)` method takes [`hdpsrp::W`](W) writer structure
impl crate::Writable for HDPSRPrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HDPSRP to value 0
impl crate::Resettable for HDPSRPrs {}
