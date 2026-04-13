///Register `SECSR` reader
pub type R = crate::R<SECSRrs>;
///Register `SECSR` writer
pub type W = crate::W<SECSRrs>;
///Field `SECEOP` reader - SECEOP
pub type SECEOP_R = crate::BitReader;
///Field `SECEOP` writer - SECEOP
pub type SECEOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SECOPERR` reader - SECOPERR
pub type SECOPERR_R = crate::BitReader;
///Field `SECOPERR` writer - SECOPERR
pub type SECOPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SECPROGERR` reader - SECPROGERR
pub type SECPROGERR_R = crate::BitReader;
///Field `SECPROGERR` writer - SECPROGERR
pub type SECPROGERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SECWRPERR` reader - SECWRPERR
pub type SECWRPERR_R = crate::BitReader;
///Field `SECWRPERR` writer - SECWRPERR
pub type SECWRPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SECPGAERR` reader - SECPGAERR
pub type SECPGAERR_R = crate::BitReader;
///Field `SECPGAERR` writer - SECPGAERR
pub type SECPGAERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SECSIZERR` reader - SECSIZERR
pub type SECSIZERR_R = crate::BitReader;
///Field `SECSIZERR` writer - SECSIZERR
pub type SECSIZERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SECPGSERR` reader - SECPGSERR
pub type SECPGSERR_R = crate::BitReader;
///Field `SECPGSERR` writer - SECPGSERR
pub type SECPGSERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SECRDERR` reader - Secure read protection error
pub type SECRDERR_R = crate::BitReader;
///Field `SECRDERR` writer - Secure read protection error
pub type SECRDERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SECBSY` reader - SECBusy
pub type SECBSY_R = crate::BitReader;
impl R {
    ///Bit 0 - SECEOP
    #[inline(always)]
    pub fn seceop(&self) -> SECEOP_R {
        SECEOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SECOPERR
    #[inline(always)]
    pub fn secoperr(&self) -> SECOPERR_R {
        SECOPERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - SECPROGERR
    #[inline(always)]
    pub fn secprogerr(&self) -> SECPROGERR_R {
        SECPROGERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SECWRPERR
    #[inline(always)]
    pub fn secwrperr(&self) -> SECWRPERR_R {
        SECWRPERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SECPGAERR
    #[inline(always)]
    pub fn secpgaerr(&self) -> SECPGAERR_R {
        SECPGAERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SECSIZERR
    #[inline(always)]
    pub fn secsizerr(&self) -> SECSIZERR_R {
        SECSIZERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SECPGSERR
    #[inline(always)]
    pub fn secpgserr(&self) -> SECPGSERR_R {
        SECPGSERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 14 - Secure read protection error
    #[inline(always)]
    pub fn secrderr(&self) -> SECRDERR_R {
        SECRDERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - SECBusy
    #[inline(always)]
    pub fn secbsy(&self) -> SECBSY_R {
        SECBSY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECSR")
            .field("seceop", &self.seceop())
            .field("secoperr", &self.secoperr())
            .field("secprogerr", &self.secprogerr())
            .field("secwrperr", &self.secwrperr())
            .field("secpgaerr", &self.secpgaerr())
            .field("secsizerr", &self.secsizerr())
            .field("secpgserr", &self.secpgserr())
            .field("secrderr", &self.secrderr())
            .field("secbsy", &self.secbsy())
            .finish()
    }
}
impl W {
    ///Bit 0 - SECEOP
    #[inline(always)]
    pub fn seceop(&mut self) -> SECEOP_W<'_, SECSRrs> {
        SECEOP_W::new(self, 0)
    }
    ///Bit 1 - SECOPERR
    #[inline(always)]
    pub fn secoperr(&mut self) -> SECOPERR_W<'_, SECSRrs> {
        SECOPERR_W::new(self, 1)
    }
    ///Bit 3 - SECPROGERR
    #[inline(always)]
    pub fn secprogerr(&mut self) -> SECPROGERR_W<'_, SECSRrs> {
        SECPROGERR_W::new(self, 3)
    }
    ///Bit 4 - SECWRPERR
    #[inline(always)]
    pub fn secwrperr(&mut self) -> SECWRPERR_W<'_, SECSRrs> {
        SECWRPERR_W::new(self, 4)
    }
    ///Bit 5 - SECPGAERR
    #[inline(always)]
    pub fn secpgaerr(&mut self) -> SECPGAERR_W<'_, SECSRrs> {
        SECPGAERR_W::new(self, 5)
    }
    ///Bit 6 - SECSIZERR
    #[inline(always)]
    pub fn secsizerr(&mut self) -> SECSIZERR_W<'_, SECSRrs> {
        SECSIZERR_W::new(self, 6)
    }
    ///Bit 7 - SECPGSERR
    #[inline(always)]
    pub fn secpgserr(&mut self) -> SECPGSERR_W<'_, SECSRrs> {
        SECPGSERR_W::new(self, 7)
    }
    ///Bit 14 - Secure read protection error
    #[inline(always)]
    pub fn secrderr(&mut self) -> SECRDERR_W<'_, SECSRrs> {
        SECRDERR_W::new(self, 14)
    }
}
/**Flash status register

You can [`read`](crate::Reg::read) this register and get [`secsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:SECSR)*/
pub struct SECSRrs;
impl crate::RegisterSpec for SECSRrs {
    type Ux = u32;
}
///`read()` method returns [`secsr::R`](R) reader structure
impl crate::Readable for SECSRrs {}
///`write(|w| ..)` method takes [`secsr::W`](W) writer structure
impl crate::Writable for SECSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECSR to value 0
impl crate::Resettable for SECSRrs {}
