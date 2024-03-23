#[doc = "Register `NSCCR` writer"]
pub type W = crate::W<NSCCRrs>;
#[doc = "Field `CLR_EOP` writer - EOP flag clear bit Setting this bit to 1 resets to 0 EOP flag in FLASH_NSSR register."]
pub type CLR_EOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_WRPERR` writer - WRPERR flag clear bit Setting this bit to 1 resets to 0 WRPERR flag in FLASH_NSSR register."]
pub type CLR_WRPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_PGSERR` writer - PGSERR flag clear bit Setting this bit to 1 resets to 0 PGSERR flag in FLASH_NSSR register."]
pub type CLR_PGSERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_STRBERR` writer - STRBERR flag clear bit Setting this bit to 1 resets to 0 STRBERR flag in FLASH_NSSR register."]
pub type CLR_STRBERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_INCERR` writer - INCERR flag clear bit Setting this bit to 1 resets to 0 INCERR flag in FLASH_NSSR register."]
pub type CLR_INCERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_OPTCHANGEERR` writer - Clear the flag corresponding flag in FLASH_NSSR by writing this bit."]
pub type CLR_OPTCHANGEERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 16 - EOP flag clear bit Setting this bit to 1 resets to 0 EOP flag in FLASH_NSSR register."]
    #[inline(always)]
    #[must_use]
    pub fn clr_eop(&mut self) -> CLR_EOP_W<NSCCRrs> {
        CLR_EOP_W::new(self, 16)
    }
    #[doc = "Bit 17 - WRPERR flag clear bit Setting this bit to 1 resets to 0 WRPERR flag in FLASH_NSSR register."]
    #[inline(always)]
    #[must_use]
    pub fn clr_wrperr(&mut self) -> CLR_WRPERR_W<NSCCRrs> {
        CLR_WRPERR_W::new(self, 17)
    }
    #[doc = "Bit 18 - PGSERR flag clear bit Setting this bit to 1 resets to 0 PGSERR flag in FLASH_NSSR register."]
    #[inline(always)]
    #[must_use]
    pub fn clr_pgserr(&mut self) -> CLR_PGSERR_W<NSCCRrs> {
        CLR_PGSERR_W::new(self, 18)
    }
    #[doc = "Bit 19 - STRBERR flag clear bit Setting this bit to 1 resets to 0 STRBERR flag in FLASH_NSSR register."]
    #[inline(always)]
    #[must_use]
    pub fn clr_strberr(&mut self) -> CLR_STRBERR_W<NSCCRrs> {
        CLR_STRBERR_W::new(self, 19)
    }
    #[doc = "Bit 20 - INCERR flag clear bit Setting this bit to 1 resets to 0 INCERR flag in FLASH_NSSR register."]
    #[inline(always)]
    #[must_use]
    pub fn clr_incerr(&mut self) -> CLR_INCERR_W<NSCCRrs> {
        CLR_INCERR_W::new(self, 20)
    }
    #[doc = "Bit 23 - Clear the flag corresponding flag in FLASH_NSSR by writing this bit."]
    #[inline(always)]
    #[must_use]
    pub fn clr_optchangeerr(&mut self) -> CLR_OPTCHANGEERR_W<NSCCRrs> {
        CLR_OPTCHANGEERR_W::new(self, 23)
    }
}
#[doc = "FLASH non-secure clear control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nsccr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NSCCRrs;
impl crate::RegisterSpec for NSCCRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`nsccr::W`](W) writer structure"]
impl crate::Writable for NSCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NSCCR to value 0"]
impl crate::Resettable for NSCCRrs {
    const RESET_VALUE: u32 = 0;
}
