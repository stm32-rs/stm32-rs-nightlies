///Register `BSEC_OTP_DATA55` reader
pub type R = crate::R<BSEC_OTP_DATA55rs>;
///Register `BSEC_OTP_DATA55` writer
pub type W = crate::W<BSEC_OTP_DATA55rs>;
///Field `DATA` reader - DATA
pub type DATA_R = crate::FieldReader<u32>;
///Field `DATA` writer - DATA
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - DATA
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BSEC_OTP_DATA55")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DATA
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<BSEC_OTP_DATA55rs> {
        DATA_W::new(self, 0)
    }
}
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_data55::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_data55::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#BSEC:BSEC_OTP_DATA55)*/
pub struct BSEC_OTP_DATA55rs;
impl crate::RegisterSpec for BSEC_OTP_DATA55rs {
    type Ux = u32;
}
///`read()` method returns [`bsec_otp_data55::R`](R) reader structure
impl crate::Readable for BSEC_OTP_DATA55rs {}
///`write(|w| ..)` method takes [`bsec_otp_data55::W`](W) writer structure
impl crate::Writable for BSEC_OTP_DATA55rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BSEC_OTP_DATA55 to value 0
impl crate::Resettable for BSEC_OTP_DATA55rs {
    const RESET_VALUE: u32 = 0;
}
