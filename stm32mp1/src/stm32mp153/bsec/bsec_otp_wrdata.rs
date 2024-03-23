#[doc = "Register `BSEC_OTP_WRDATA` reader"]
pub type R = crate::R<BSEC_OTP_WRDATArs>;
#[doc = "Register `BSEC_OTP_WRDATA` writer"]
pub type W = crate::W<BSEC_OTP_WRDATArs>;
#[doc = "Field `WRDATA` reader - WRDATA"]
pub type WRDATA_R = crate::FieldReader<u32>;
#[doc = "Field `WRDATA` writer - WRDATA"]
pub type WRDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WRDATA"]
    #[inline(always)]
    pub fn wrdata(&self) -> WRDATA_R {
        WRDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WRDATA"]
    #[inline(always)]
    #[must_use]
    pub fn wrdata(&mut self) -> WRDATA_W<BSEC_OTP_WRDATArs> {
        WRDATA_W::new(self, 0)
    }
}
#[doc = "BSEC OTP write data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bsec_otp_wrdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bsec_otp_wrdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BSEC_OTP_WRDATArs;
impl crate::RegisterSpec for BSEC_OTP_WRDATArs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bsec_otp_wrdata::R`](R) reader structure"]
impl crate::Readable for BSEC_OTP_WRDATArs {}
#[doc = "`write(|w| ..)` method takes [`bsec_otp_wrdata::W`](W) writer structure"]
impl crate::Writable for BSEC_OTP_WRDATArs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BSEC_OTP_WRDATA to value 0"]
impl crate::Resettable for BSEC_OTP_WRDATArs {
    const RESET_VALUE: u32 = 0;
}
