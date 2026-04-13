///Register `OTP_CONTROL` reader
pub type R = crate::R<OTP_CONTROLrs>;
///Register `OTP_CONTROL` writer
pub type W = crate::W<OTP_CONTROLrs>;
///Field `ADDR` reader - ADDR
pub type ADDR_R = crate::FieldReader;
///Field `ADDR` writer - ADDR
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `PROG` reader - PROG
pub type PROG_R = crate::BitReader;
///Field `PROG` writer - PROG
pub type PROG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOCK` reader - LOCK
pub type LOCK_R = crate::BitReader;
///Field `LOCK` writer - LOCK
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:6 - ADDR
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 8 - PROG
    #[inline(always)]
    pub fn prog(&self) -> PROG_R {
        PROG_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LOCK
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTP_CONTROL")
            .field("addr", &self.addr())
            .field("prog", &self.prog())
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - ADDR
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W<'_, OTP_CONTROLrs> {
        ADDR_W::new(self, 0)
    }
    ///Bit 8 - PROG
    #[inline(always)]
    pub fn prog(&mut self) -> PROG_W<'_, OTP_CONTROLrs> {
        PROG_W::new(self, 8)
    }
    ///Bit 9 - LOCK
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, OTP_CONTROLrs> {
        LOCK_W::new(self, 9)
    }
}
/**BSEC OTP control register

You can [`read`](crate::Reg::read) this register and get [`otp_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_CONTROL)*/
pub struct OTP_CONTROLrs;
impl crate::RegisterSpec for OTP_CONTROLrs {
    type Ux = u32;
}
///`read()` method returns [`otp_control::R`](R) reader structure
impl crate::Readable for OTP_CONTROLrs {}
///`write(|w| ..)` method takes [`otp_control::W`](W) writer structure
impl crate::Writable for OTP_CONTROLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTP_CONTROL to value 0
impl crate::Resettable for OTP_CONTROLrs {}
