///Register `OTP_DATA56` reader
pub type R = crate::R<OTP_DATA56rs>;
///Register `OTP_DATA56` writer
pub type W = crate::W<OTP_DATA56rs>;
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
        f.debug_struct("OTP_DATA56")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DATA
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W<OTP_DATA56rs> {
        DATA_W::new(self, 0)
    }
}
/**Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.

You can [`read`](crate::Reg::read) this register and get [`otp_data56::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_data56::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DATA56)*/
pub struct OTP_DATA56rs;
impl crate::RegisterSpec for OTP_DATA56rs {
    type Ux = u32;
}
///`read()` method returns [`otp_data56::R`](R) reader structure
impl crate::Readable for OTP_DATA56rs {}
///`write(|w| ..)` method takes [`otp_data56::W`](W) writer structure
impl crate::Writable for OTP_DATA56rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTP_DATA56 to value 0
impl crate::Resettable for OTP_DATA56rs {}
