///Register `WKUPFR` reader
pub type R = crate::R<WKUPFRrs>;
///Field `WKUPF1` reader - Wakeup pin WKUP1 flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPC1 bit in the PWR wakeup clear register (PWR_WKUPCR).
pub type WKUPF1_R = crate::BitReader;
///Field `WKUPF2` reader - Wakeup pin WKUP2 flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPC2 bit in the PWR wakeup clear register (PWR_WKUPCR).
pub type WKUPF2_R = crate::BitReader;
///Field `WKUPF3` reader - Wakeup pin WKUP3 flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPC3 bit in the PWR wakeup clear register (PWR_WKUPCR).
pub type WKUPF3_R = crate::BitReader;
///Field `WKUPF4` reader - Wakeup pin WKUP4 flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPC4 bit in the PWR wakeup clear register (PWR_WKUPCR).
pub type WKUPF4_R = crate::BitReader;
impl R {
    ///Bit 0 - Wakeup pin WKUP1 flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPC1 bit in the PWR wakeup clear register (PWR_WKUPCR).
    #[inline(always)]
    pub fn wkupf1(&self) -> WKUPF1_R {
        WKUPF1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Wakeup pin WKUP2 flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPC2 bit in the PWR wakeup clear register (PWR_WKUPCR).
    #[inline(always)]
    pub fn wkupf2(&self) -> WKUPF2_R {
        WKUPF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup pin WKUP3 flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPC3 bit in the PWR wakeup clear register (PWR_WKUPCR).
    #[inline(always)]
    pub fn wkupf3(&self) -> WKUPF3_R {
        WKUPF3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Wakeup pin WKUP4 flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPC4 bit in the PWR wakeup clear register (PWR_WKUPCR).
    #[inline(always)]
    pub fn wkupf4(&self) -> WKUPF4_R {
        WKUPF4_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WKUPFR")
            .field("wkupf1", &self.wkupf1())
            .field("wkupf2", &self.wkupf2())
            .field("wkupf3", &self.wkupf3())
            .field("wkupf4", &self.wkupf4())
            .finish()
    }
}
/**PWR wakeup flag register

You can [`read`](crate::Reg::read) this register and get [`wkupfr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#PWR:WKUPFR)*/
pub struct WKUPFRrs;
impl crate::RegisterSpec for WKUPFRrs {
    type Ux = u32;
}
///`read()` method returns [`wkupfr::R`](R) reader structure
impl crate::Readable for WKUPFRrs {}
///`reset()` method sets WKUPFR to value 0
impl crate::Resettable for WKUPFRrs {}
