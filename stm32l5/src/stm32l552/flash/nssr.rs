///Register `NSSR` reader
pub type R = crate::R<NSSRrs>;
///Register `NSSR` writer
pub type W = crate::W<NSSRrs>;
///Field `NSEOP` reader - NSEOP
pub type NSEOP_R = crate::BitReader;
///Field `NSEOP` writer - NSEOP
pub type NSEOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NSOPERR` reader - NSOPERR
pub type NSOPERR_R = crate::BitReader;
///Field `NSOPERR` writer - NSOPERR
pub type NSOPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NSPROGERR` reader - NSPROGERR
pub type NSPROGERR_R = crate::BitReader;
///Field `NSPROGERR` writer - NSPROGERR
pub type NSPROGERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NSWRPERR` reader - NSWRPERR
pub type NSWRPERR_R = crate::BitReader;
///Field `NSWRPERR` writer - NSWRPERR
pub type NSWRPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NSPGAERR` reader - NSPGAERR
pub type NSPGAERR_R = crate::BitReader;
///Field `NSPGAERR` writer - NSPGAERR
pub type NSPGAERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NSSIZERR` reader - NSSIZERR
pub type NSSIZERR_R = crate::BitReader;
///Field `NSSIZERR` writer - NSSIZERR
pub type NSSIZERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NSPGSERR` reader - NSPGSERR
pub type NSPGSERR_R = crate::BitReader;
///Field `NSPGSERR` writer - NSPGSERR
pub type NSPGSERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPTWERR` reader - OPTWERR
pub type OPTWERR_R = crate::BitReader;
///Field `OPTWERR` writer - OPTWERR
pub type OPTWERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPTVERR` reader - OPTVERR
pub type OPTVERR_R = crate::BitReader;
///Field `OPTVERR` writer - OPTVERR
pub type OPTVERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NSBSY` reader - NSBusy
pub type NSBSY_R = crate::BitReader;
impl R {
    ///Bit 0 - NSEOP
    #[inline(always)]
    pub fn nseop(&self) -> NSEOP_R {
        NSEOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - NSOPERR
    #[inline(always)]
    pub fn nsoperr(&self) -> NSOPERR_R {
        NSOPERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - NSPROGERR
    #[inline(always)]
    pub fn nsprogerr(&self) -> NSPROGERR_R {
        NSPROGERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - NSWRPERR
    #[inline(always)]
    pub fn nswrperr(&self) -> NSWRPERR_R {
        NSWRPERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - NSPGAERR
    #[inline(always)]
    pub fn nspgaerr(&self) -> NSPGAERR_R {
        NSPGAERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - NSSIZERR
    #[inline(always)]
    pub fn nssizerr(&self) -> NSSIZERR_R {
        NSSIZERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - NSPGSERR
    #[inline(always)]
    pub fn nspgserr(&self) -> NSPGSERR_R {
        NSPGSERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 13 - OPTWERR
    #[inline(always)]
    pub fn optwerr(&self) -> OPTWERR_R {
        OPTWERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - OPTVERR
    #[inline(always)]
    pub fn optverr(&self) -> OPTVERR_R {
        OPTVERR_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - NSBusy
    #[inline(always)]
    pub fn nsbsy(&self) -> NSBSY_R {
        NSBSY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NSSR")
            .field("nseop", &self.nseop())
            .field("nsoperr", &self.nsoperr())
            .field("nsprogerr", &self.nsprogerr())
            .field("nswrperr", &self.nswrperr())
            .field("nspgaerr", &self.nspgaerr())
            .field("nssizerr", &self.nssizerr())
            .field("nspgserr", &self.nspgserr())
            .field("optwerr", &self.optwerr())
            .field("optverr", &self.optverr())
            .field("nsbsy", &self.nsbsy())
            .finish()
    }
}
impl W {
    ///Bit 0 - NSEOP
    #[inline(always)]
    pub fn nseop(&mut self) -> NSEOP_W<'_, NSSRrs> {
        NSEOP_W::new(self, 0)
    }
    ///Bit 1 - NSOPERR
    #[inline(always)]
    pub fn nsoperr(&mut self) -> NSOPERR_W<'_, NSSRrs> {
        NSOPERR_W::new(self, 1)
    }
    ///Bit 3 - NSPROGERR
    #[inline(always)]
    pub fn nsprogerr(&mut self) -> NSPROGERR_W<'_, NSSRrs> {
        NSPROGERR_W::new(self, 3)
    }
    ///Bit 4 - NSWRPERR
    #[inline(always)]
    pub fn nswrperr(&mut self) -> NSWRPERR_W<'_, NSSRrs> {
        NSWRPERR_W::new(self, 4)
    }
    ///Bit 5 - NSPGAERR
    #[inline(always)]
    pub fn nspgaerr(&mut self) -> NSPGAERR_W<'_, NSSRrs> {
        NSPGAERR_W::new(self, 5)
    }
    ///Bit 6 - NSSIZERR
    #[inline(always)]
    pub fn nssizerr(&mut self) -> NSSIZERR_W<'_, NSSRrs> {
        NSSIZERR_W::new(self, 6)
    }
    ///Bit 7 - NSPGSERR
    #[inline(always)]
    pub fn nspgserr(&mut self) -> NSPGSERR_W<'_, NSSRrs> {
        NSPGSERR_W::new(self, 7)
    }
    ///Bit 13 - OPTWERR
    #[inline(always)]
    pub fn optwerr(&mut self) -> OPTWERR_W<'_, NSSRrs> {
        OPTWERR_W::new(self, 13)
    }
    ///Bit 15 - OPTVERR
    #[inline(always)]
    pub fn optverr(&mut self) -> OPTVERR_W<'_, NSSRrs> {
        OPTVERR_W::new(self, 15)
    }
}
/**Flash status register

You can [`read`](crate::Reg::read) this register and get [`nssr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nssr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:NSSR)*/
pub struct NSSRrs;
impl crate::RegisterSpec for NSSRrs {
    type Ux = u32;
}
///`read()` method returns [`nssr::R`](R) reader structure
impl crate::Readable for NSSRrs {}
///`write(|w| ..)` method takes [`nssr::W`](W) writer structure
impl crate::Writable for NSSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NSSR to value 0
impl crate::Resettable for NSSRrs {}
