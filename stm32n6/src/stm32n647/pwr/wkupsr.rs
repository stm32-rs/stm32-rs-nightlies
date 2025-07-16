///Register `WKUPSR` reader
pub type R = crate::R<WKUPSRrs>;
///Field `WKUPF1` reader - Wake-up flag for WKUP1 pin before enable
pub type WKUPF1_R = crate::BitReader;
///Field `WKUPF2` reader - Wake-up flag for WKUP2 pin before enable
pub type WKUPF2_R = crate::BitReader;
///Field `WKUPF3` reader - Wake-up flag for WKUP3 pin before enable
pub type WKUPF3_R = crate::BitReader;
///Field `WKUPF4` reader - Wake-up flag for WKUP4 pin before enable
pub type WKUPF4_R = crate::BitReader;
impl R {
    ///Bit 0 - Wake-up flag for WKUP1 pin before enable
    #[inline(always)]
    pub fn wkupf1(&self) -> WKUPF1_R {
        WKUPF1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Wake-up flag for WKUP2 pin before enable
    #[inline(always)]
    pub fn wkupf2(&self) -> WKUPF2_R {
        WKUPF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wake-up flag for WKUP3 pin before enable
    #[inline(always)]
    pub fn wkupf3(&self) -> WKUPF3_R {
        WKUPF3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Wake-up flag for WKUP4 pin before enable
    #[inline(always)]
    pub fn wkupf4(&self) -> WKUPF4_R {
        WKUPF4_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WKUPSR")
            .field("wkupf1", &self.wkupf1())
            .field("wkupf2", &self.wkupf2())
            .field("wkupf3", &self.wkupf3())
            .field("wkupf4", &self.wkupf4())
            .finish()
    }
}
/**PWR wake-up status register

You can [`read`](crate::Reg::read) this register and get [`wkupsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#PWR:WKUPSR)*/
pub struct WKUPSRrs;
impl crate::RegisterSpec for WKUPSRrs {
    type Ux = u32;
}
///`read()` method returns [`wkupsr::R`](R) reader structure
impl crate::Readable for WKUPSRrs {}
///`reset()` method sets WKUPSR to value 0
impl crate::Resettable for WKUPSRrs {}
