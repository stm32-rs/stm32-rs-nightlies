///Register `BSEC_OTP_WRDATA` reader
pub type R = crate::R<BSEC_OTP_WRDATArs>;
///Register `BSEC_OTP_WRDATA` writer
pub type W = crate::W<BSEC_OTP_WRDATArs>;
///Field `WRDATA` reader - WRDATA
pub type WRDATA_R = crate::FieldReader<u32>;
///Field `WRDATA` writer - WRDATA
pub type WRDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - WRDATA
    #[inline(always)]
    pub fn wrdata(&self) -> WRDATA_R {
        WRDATA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BSEC_OTP_WRDATA")
            .field("wrdata", &self.wrdata())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - WRDATA
    #[inline(always)]
    #[must_use]
    pub fn wrdata(&mut self) -> WRDATA_W<BSEC_OTP_WRDATArs> {
        WRDATA_W::new(self, 0)
    }
}
/**BSEC OTP write data register

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_wrdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_wrdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_WRDATA)*/
pub struct BSEC_OTP_WRDATArs;
impl crate::RegisterSpec for BSEC_OTP_WRDATArs {
    type Ux = u32;
}
///`read()` method returns [`bsec_otp_wrdata::R`](R) reader structure
impl crate::Readable for BSEC_OTP_WRDATArs {}
///`write(|w| ..)` method takes [`bsec_otp_wrdata::W`](W) writer structure
impl crate::Writable for BSEC_OTP_WRDATArs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BSEC_OTP_WRDATA to value 0
impl crate::Resettable for BSEC_OTP_WRDATArs {
    const RESET_VALUE: u32 = 0;
}
