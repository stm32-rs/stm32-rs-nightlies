#[doc = "Register `BSEC_OTP_DATA47` reader"]
pub type R = crate::R<BSEC_OTP_DATA47rs>;
#[doc = "Register `BSEC_OTP_DATA47` writer"]
pub type W = crate::W<BSEC_OTP_DATA47rs>;
#[doc = "Field `DATA` reader - DATA"]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - DATA"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DATA"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DATA"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<BSEC_OTP_DATA47rs> {
        DATA_W::new(self, 0)
    }
}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bsec_otp_data47::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bsec_otp_data47::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BSEC_OTP_DATA47rs;
impl crate::RegisterSpec for BSEC_OTP_DATA47rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bsec_otp_data47::R`](R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA47rs {}
#[doc = "`write(|w| ..)` method takes [`bsec_otp_data47::W`](W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA47rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BSEC_OTP_DATA47 to value 0"]
impl crate::Resettable for BSEC_OTP_DATA47rs {
    const RESET_VALUE: u32 = 0;
}
