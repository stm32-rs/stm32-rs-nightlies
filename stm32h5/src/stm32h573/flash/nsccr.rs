///Register `NSCCR` writer
pub type W = crate::W<NSCCRrs>;
///Field `CLR_EOP` writer - EOP flag clear bit Setting this bit to 1 resets to 0 EOP flag in FLASH_NSSR register.
pub type CLR_EOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLR_WRPERR` writer - WRPERR flag clear bit Setting this bit to 1 resets to 0 WRPERR flag in FLASH_NSSR register.
pub type CLR_WRPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLR_PGSERR` writer - PGSERR flag clear bit Setting this bit to 1 resets to 0 PGSERR flag in FLASH_NSSR register.
pub type CLR_PGSERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLR_STRBERR` writer - STRBERR flag clear bit Setting this bit to 1 resets to 0 STRBERR flag in FLASH_NSSR register.
pub type CLR_STRBERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLR_INCERR` writer - INCERR flag clear bit Setting this bit to 1 resets to 0 INCERR flag in FLASH_NSSR register.
pub type CLR_INCERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLR_OBKERR` writer - OBKERR flag clear bit. Setting this bit to 1 resets to 0 OBKERR flag in FLASH_NSSR register.
pub type CLR_OBKERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLR_OBKWERR` writer - OBKWERR flag clear bit. Setting this bit to 1 resets to 0 OBKWERR flag in FLASH_NSSR register.
pub type CLR_OBKWERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLR_OPTCHANGEERR` writer - Clear the flag corresponding flag in FLASH_NSSR by writing this bit.
pub type CLR_OPTCHANGEERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<NSCCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 16 - EOP flag clear bit Setting this bit to 1 resets to 0 EOP flag in FLASH_NSSR register.
    #[inline(always)]
    pub fn clr_eop(&mut self) -> CLR_EOP_W<NSCCRrs> {
        CLR_EOP_W::new(self, 16)
    }
    ///Bit 17 - WRPERR flag clear bit Setting this bit to 1 resets to 0 WRPERR flag in FLASH_NSSR register.
    #[inline(always)]
    pub fn clr_wrperr(&mut self) -> CLR_WRPERR_W<NSCCRrs> {
        CLR_WRPERR_W::new(self, 17)
    }
    ///Bit 18 - PGSERR flag clear bit Setting this bit to 1 resets to 0 PGSERR flag in FLASH_NSSR register.
    #[inline(always)]
    pub fn clr_pgserr(&mut self) -> CLR_PGSERR_W<NSCCRrs> {
        CLR_PGSERR_W::new(self, 18)
    }
    ///Bit 19 - STRBERR flag clear bit Setting this bit to 1 resets to 0 STRBERR flag in FLASH_NSSR register.
    #[inline(always)]
    pub fn clr_strberr(&mut self) -> CLR_STRBERR_W<NSCCRrs> {
        CLR_STRBERR_W::new(self, 19)
    }
    ///Bit 20 - INCERR flag clear bit Setting this bit to 1 resets to 0 INCERR flag in FLASH_NSSR register.
    #[inline(always)]
    pub fn clr_incerr(&mut self) -> CLR_INCERR_W<NSCCRrs> {
        CLR_INCERR_W::new(self, 20)
    }
    ///Bit 21 - OBKERR flag clear bit. Setting this bit to 1 resets to 0 OBKERR flag in FLASH_NSSR register.
    #[inline(always)]
    pub fn clr_obkerr(&mut self) -> CLR_OBKERR_W<NSCCRrs> {
        CLR_OBKERR_W::new(self, 21)
    }
    ///Bit 22 - OBKWERR flag clear bit. Setting this bit to 1 resets to 0 OBKWERR flag in FLASH_NSSR register.
    #[inline(always)]
    pub fn clr_obkwerr(&mut self) -> CLR_OBKWERR_W<NSCCRrs> {
        CLR_OBKWERR_W::new(self, 22)
    }
    ///Bit 23 - Clear the flag corresponding flag in FLASH_NSSR by writing this bit.
    #[inline(always)]
    pub fn clr_optchangeerr(&mut self) -> CLR_OPTCHANGEERR_W<NSCCRrs> {
        CLR_OPTCHANGEERR_W::new(self, 23)
    }
}
/**FLASH non-secure clear control register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsccr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#FLASH:NSCCR)*/
pub struct NSCCRrs;
impl crate::RegisterSpec for NSCCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`nsccr::W`](W) writer structure
impl crate::Writable for NSCCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NSCCR to value 0
impl crate::Resettable for NSCCRrs {}
