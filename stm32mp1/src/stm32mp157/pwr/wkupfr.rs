///Register `WKUPFR` reader
pub type R = crate::R<WKUPFRrs>;
///Field `WKUPF1` reader - WKUPF1
pub type WKUPF1_R = crate::BitReader;
///Field `WKUPF2` reader - WKUPF2
pub type WKUPF2_R = crate::BitReader;
///Field `WKUPF3` reader - WKUPF3
pub type WKUPF3_R = crate::BitReader;
///Field `WKUPF4` reader - WKUPF4
pub type WKUPF4_R = crate::BitReader;
///Field `WKUPF5` reader - WKUPF5
pub type WKUPF5_R = crate::BitReader;
///Field `WKUPF6` reader - WKUPF6
pub type WKUPF6_R = crate::BitReader;
impl R {
    ///Bit 0 - WKUPF1
    #[inline(always)]
    pub fn wkupf1(&self) -> WKUPF1_R {
        WKUPF1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - WKUPF2
    #[inline(always)]
    pub fn wkupf2(&self) -> WKUPF2_R {
        WKUPF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - WKUPF3
    #[inline(always)]
    pub fn wkupf3(&self) -> WKUPF3_R {
        WKUPF3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - WKUPF4
    #[inline(always)]
    pub fn wkupf4(&self) -> WKUPF4_R {
        WKUPF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - WKUPF5
    #[inline(always)]
    pub fn wkupf5(&self) -> WKUPF5_R {
        WKUPF5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - WKUPF6
    #[inline(always)]
    pub fn wkupf6(&self) -> WKUPF6_R {
        WKUPF6_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WKUPFR")
            .field("wkupf1", &self.wkupf1())
            .field("wkupf2", &self.wkupf2())
            .field("wkupf3", &self.wkupf3())
            .field("wkupf4", &self.wkupf4())
            .field("wkupf5", &self.wkupf5())
            .field("wkupf6", &self.wkupf6())
            .finish()
    }
}
/**Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...)

You can [`read`](crate::Reg::read) this register and get [`wkupfr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#PWR:WKUPFR)*/
pub struct WKUPFRrs;
impl crate::RegisterSpec for WKUPFRrs {
    type Ux = u32;
}
///`read()` method returns [`wkupfr::R`](R) reader structure
impl crate::Readable for WKUPFRrs {}
///`reset()` method sets WKUPFR to value 0
impl crate::Resettable for WKUPFRrs {}
