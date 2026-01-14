///Register `LCR` reader
pub type R = crate::R<LCRrs>;
///Field `LNBR` reader - LNBR
pub type LNBR_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - LNBR
    #[inline(always)]
    pub fn lnbr(&self) -> LNBR_R {
        LNBR_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCR").field("lnbr", &self.lnbr()).finish()
    }
}
/**LDTC layer count register

You can [`read`](crate::Reg::read) this register and get [`lcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LCR)*/
pub struct LCRrs;
impl crate::RegisterSpec for LCRrs {
    type Ux = u32;
}
///`read()` method returns [`lcr::R`](R) reader structure
impl crate::Readable for LCRrs {}
///`reset()` method sets LCR to value 0x02
impl crate::Resettable for LCRrs {
    const RESET_VALUE: u32 = 0x02;
}
