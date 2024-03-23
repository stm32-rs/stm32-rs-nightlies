#[doc = "Register `NSSR` reader"]
pub type R = crate::R<NSSRrs>;
#[doc = "Register `NSSR` writer"]
pub type W = crate::W<NSSRrs>;
#[doc = "Field `NSEOP` reader - NSEOP"]
pub type NSEOP_R = crate::BitReader;
#[doc = "Field `NSEOP` writer - NSEOP"]
pub type NSEOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSOPERR` reader - NSOPERR"]
pub type NSOPERR_R = crate::BitReader;
#[doc = "Field `NSOPERR` writer - NSOPERR"]
pub type NSOPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSPROGERR` reader - NSPROGERR"]
pub type NSPROGERR_R = crate::BitReader;
#[doc = "Field `NSPROGERR` writer - NSPROGERR"]
pub type NSPROGERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSWRPERR` reader - NSWRPERR"]
pub type NSWRPERR_R = crate::BitReader;
#[doc = "Field `NSWRPERR` writer - NSWRPERR"]
pub type NSWRPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSPGAERR` reader - NSPGAERR"]
pub type NSPGAERR_R = crate::BitReader;
#[doc = "Field `NSPGAERR` writer - NSPGAERR"]
pub type NSPGAERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSSIZERR` reader - NSSIZERR"]
pub type NSSIZERR_R = crate::BitReader;
#[doc = "Field `NSSIZERR` writer - NSSIZERR"]
pub type NSSIZERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSPGSERR` reader - NSPGSERR"]
pub type NSPGSERR_R = crate::BitReader;
#[doc = "Field `NSPGSERR` writer - NSPGSERR"]
pub type NSPGSERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTWERR` reader - OPTWERR"]
pub type OPTWERR_R = crate::BitReader;
#[doc = "Field `OPTWERR` writer - OPTWERR"]
pub type OPTWERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTVERR` reader - OPTVERR"]
pub type OPTVERR_R = crate::BitReader;
#[doc = "Field `OPTVERR` writer - OPTVERR"]
pub type OPTVERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSBSY` reader - NSBusy"]
pub type NSBSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NSEOP"]
    #[inline(always)]
    pub fn nseop(&self) -> NSEOP_R {
        NSEOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NSOPERR"]
    #[inline(always)]
    pub fn nsoperr(&self) -> NSOPERR_R {
        NSOPERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - NSPROGERR"]
    #[inline(always)]
    pub fn nsprogerr(&self) -> NSPROGERR_R {
        NSPROGERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NSWRPERR"]
    #[inline(always)]
    pub fn nswrperr(&self) -> NSWRPERR_R {
        NSWRPERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NSPGAERR"]
    #[inline(always)]
    pub fn nspgaerr(&self) -> NSPGAERR_R {
        NSPGAERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NSSIZERR"]
    #[inline(always)]
    pub fn nssizerr(&self) -> NSSIZERR_R {
        NSSIZERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NSPGSERR"]
    #[inline(always)]
    pub fn nspgserr(&self) -> NSPGSERR_R {
        NSPGSERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 13 - OPTWERR"]
    #[inline(always)]
    pub fn optwerr(&self) -> OPTWERR_R {
        OPTWERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - OPTVERR"]
    #[inline(always)]
    pub fn optverr(&self) -> OPTVERR_R {
        OPTVERR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - NSBusy"]
    #[inline(always)]
    pub fn nsbsy(&self) -> NSBSY_R {
        NSBSY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NSEOP"]
    #[inline(always)]
    #[must_use]
    pub fn nseop(&mut self) -> NSEOP_W<NSSRrs> {
        NSEOP_W::new(self, 0)
    }
    #[doc = "Bit 1 - NSOPERR"]
    #[inline(always)]
    #[must_use]
    pub fn nsoperr(&mut self) -> NSOPERR_W<NSSRrs> {
        NSOPERR_W::new(self, 1)
    }
    #[doc = "Bit 3 - NSPROGERR"]
    #[inline(always)]
    #[must_use]
    pub fn nsprogerr(&mut self) -> NSPROGERR_W<NSSRrs> {
        NSPROGERR_W::new(self, 3)
    }
    #[doc = "Bit 4 - NSWRPERR"]
    #[inline(always)]
    #[must_use]
    pub fn nswrperr(&mut self) -> NSWRPERR_W<NSSRrs> {
        NSWRPERR_W::new(self, 4)
    }
    #[doc = "Bit 5 - NSPGAERR"]
    #[inline(always)]
    #[must_use]
    pub fn nspgaerr(&mut self) -> NSPGAERR_W<NSSRrs> {
        NSPGAERR_W::new(self, 5)
    }
    #[doc = "Bit 6 - NSSIZERR"]
    #[inline(always)]
    #[must_use]
    pub fn nssizerr(&mut self) -> NSSIZERR_W<NSSRrs> {
        NSSIZERR_W::new(self, 6)
    }
    #[doc = "Bit 7 - NSPGSERR"]
    #[inline(always)]
    #[must_use]
    pub fn nspgserr(&mut self) -> NSPGSERR_W<NSSRrs> {
        NSPGSERR_W::new(self, 7)
    }
    #[doc = "Bit 13 - OPTWERR"]
    #[inline(always)]
    #[must_use]
    pub fn optwerr(&mut self) -> OPTWERR_W<NSSRrs> {
        OPTWERR_W::new(self, 13)
    }
    #[doc = "Bit 15 - OPTVERR"]
    #[inline(always)]
    #[must_use]
    pub fn optverr(&mut self) -> OPTVERR_W<NSSRrs> {
        OPTVERR_W::new(self, 15)
    }
}
#[doc = "Flash status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nssr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nssr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NSSRrs;
impl crate::RegisterSpec for NSSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nssr::R`](R) reader structure"]
impl crate::Readable for NSSRrs {}
#[doc = "`write(|w| ..)` method takes [`nssr::W`](W) writer structure"]
impl crate::Writable for NSSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NSSR to value 0"]
impl crate::Resettable for NSSRrs {
    const RESET_VALUE: u32 = 0;
}
