///Register `HDPSR` reader
pub type R = crate::R<HDPSRrs>;
///Field `HDP_AREA_START` reader - Hide protection user Flash area start This option sets the start address that contains the first 256-byte block of the hide protection (HDP) area in user Flash area. If HDP_AREA_END=HDP_AREA_START all the sectors are protected. If HDP_AREA_END<HDP_AREA_START no sectors are protected.
pub type HDP_AREA_START_R = crate::FieldReader<u16>;
///Field `HDP_AREA_END` reader - Hide protection user Flash area end This option sets the end address that contains the last 256-byte block of the hide protection (HDP) area in user Flash area. If HDP_AREA_END=HDP_AREA_START all the sectors are protected. If HDP_AREA_END<HDP_AREA_START no sectors are protected.
pub type HDP_AREA_END_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:8 - Hide protection user Flash area start This option sets the start address that contains the first 256-byte block of the hide protection (HDP) area in user Flash area. If HDP_AREA_END=HDP_AREA_START all the sectors are protected. If HDP_AREA_END<HDP_AREA_START no sectors are protected.
    #[inline(always)]
    pub fn hdp_area_start(&self) -> HDP_AREA_START_R {
        HDP_AREA_START_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Hide protection user Flash area end This option sets the end address that contains the last 256-byte block of the hide protection (HDP) area in user Flash area. If HDP_AREA_END=HDP_AREA_START all the sectors are protected. If HDP_AREA_END<HDP_AREA_START no sectors are protected.
    #[inline(always)]
    pub fn hdp_area_end(&self) -> HDP_AREA_END_R {
        HDP_AREA_END_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HDPSR")
            .field("hdp_area_start", &self.hdp_area_start())
            .field("hdp_area_end", &self.hdp_area_end())
            .finish()
    }
}
/**FLASH hide protection status register

You can [`read`](crate::Reg::read) this register and get [`hdpsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#FLASH:HDPSR)*/
pub struct HDPSRrs;
impl crate::RegisterSpec for HDPSRrs {
    type Ux = u32;
}
///`read()` method returns [`hdpsr::R`](R) reader structure
impl crate::Readable for HDPSRrs {}
///`reset()` method sets HDPSR to value 0
impl crate::Resettable for HDPSRrs {}
