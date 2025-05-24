///Register `WPSN_CUR` reader
pub type R = crate::R<WPSN_CURrs>;
///Field `WRPSn` reader - sector write protection option status byte
pub type WRPSN_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - sector write protection option status byte
    #[inline(always)]
    pub fn wrpsn(&self) -> WRPSN_R {
        WRPSN_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WPSN_CUR")
            .field("wrpsn", &self.wrpsn())
            .finish()
    }
}
/**FLASH write sector protection for bank 1

You can [`read`](crate::Reg::read) this register and get [`wpsn_cur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FLASH:WPSN_CUR)*/
pub struct WPSN_CURrs;
impl crate::RegisterSpec for WPSN_CURrs {
    type Ux = u32;
}
///`read()` method returns [`wpsn_cur::R`](R) reader structure
impl crate::Readable for WPSN_CURrs {}
///`reset()` method sets WPSN_CUR to value 0
impl crate::Resettable for WPSN_CURrs {}
