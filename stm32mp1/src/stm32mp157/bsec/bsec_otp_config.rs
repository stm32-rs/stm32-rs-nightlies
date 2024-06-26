///Register `BSEC_OTP_CONFIG` reader
pub type R = crate::R<BSEC_OTP_CONFIGrs>;
///Register `BSEC_OTP_CONFIG` writer
pub type W = crate::W<BSEC_OTP_CONFIGrs>;
///Field `PWRUP` reader - PWRUP
pub type PWRUP_R = crate::BitReader;
///Field `PWRUP` writer - PWRUP
pub type PWRUP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRC` reader - FRC
pub type FRC_R = crate::FieldReader;
///Field `FRC` writer - FRC
pub type FRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PRGWIDTH` reader - PRGWIDTH
pub type PRGWIDTH_R = crate::FieldReader;
///Field `PRGWIDTH` writer - PRGWIDTH
pub type PRGWIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TREAD` reader - TREAD
pub type TREAD_R = crate::FieldReader;
///Field `TREAD` writer - TREAD
pub type TREAD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - PWRUP
    #[inline(always)]
    pub fn pwrup(&self) -> PWRUP_R {
        PWRUP_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - FRC
    #[inline(always)]
    pub fn frc(&self) -> FRC_R {
        FRC_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 3:6 - PRGWIDTH
    #[inline(always)]
    pub fn prgwidth(&self) -> PRGWIDTH_R {
        PRGWIDTH_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    ///Bits 7:8 - TREAD
    #[inline(always)]
    pub fn tread(&self) -> TREAD_R {
        TREAD_R::new(((self.bits >> 7) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BSEC_OTP_CONFIG")
            .field("pwrup", &self.pwrup())
            .field("frc", &self.frc())
            .field("prgwidth", &self.prgwidth())
            .field("tread", &self.tread())
            .finish()
    }
}
impl W {
    ///Bit 0 - PWRUP
    #[inline(always)]
    #[must_use]
    pub fn pwrup(&mut self) -> PWRUP_W<BSEC_OTP_CONFIGrs> {
        PWRUP_W::new(self, 0)
    }
    ///Bits 1:2 - FRC
    #[inline(always)]
    #[must_use]
    pub fn frc(&mut self) -> FRC_W<BSEC_OTP_CONFIGrs> {
        FRC_W::new(self, 1)
    }
    ///Bits 3:6 - PRGWIDTH
    #[inline(always)]
    #[must_use]
    pub fn prgwidth(&mut self) -> PRGWIDTH_W<BSEC_OTP_CONFIGrs> {
        PRGWIDTH_W::new(self, 3)
    }
    ///Bits 7:8 - TREAD
    #[inline(always)]
    #[must_use]
    pub fn tread(&mut self) -> TREAD_W<BSEC_OTP_CONFIGrs> {
        TREAD_W::new(self, 7)
    }
}
/**BSEC OTP configuration register

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#BSEC:BSEC_OTP_CONFIG)*/
pub struct BSEC_OTP_CONFIGrs;
impl crate::RegisterSpec for BSEC_OTP_CONFIGrs {
    type Ux = u32;
}
///`read()` method returns [`bsec_otp_config::R`](R) reader structure
impl crate::Readable for BSEC_OTP_CONFIGrs {}
///`write(|w| ..)` method takes [`bsec_otp_config::W`](W) writer structure
impl crate::Writable for BSEC_OTP_CONFIGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BSEC_OTP_CONFIG to value 0x0e
impl crate::Resettable for BSEC_OTP_CONFIGrs {
    const RESET_VALUE: u32 = 0x0e;
}
