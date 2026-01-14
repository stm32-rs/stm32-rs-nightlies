///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
///Field `EOP` reader - End of operation
pub type EOP_R = crate::BitReader;
///Field `EOP` writer - End of operation
pub type EOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPERR` reader - Operation error
pub type OPERR_R = crate::BitReader;
///Field `OPERR` writer - Operation error
pub type OPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PROGERR` reader - Programming error
pub type PROGERR_R = crate::BitReader;
///Field `PROGERR` writer - Programming error
pub type PROGERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WRPERR` reader - Write protected error
pub type WRPERR_R = crate::BitReader;
///Field `WRPERR` writer - Write protected error
pub type WRPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PGAERR` reader - Programming alignment error
pub type PGAERR_R = crate::BitReader;
///Field `PGAERR` writer - Programming alignment error
pub type PGAERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SIZERR` reader - Size error
pub type SIZERR_R = crate::BitReader;
///Field `SIZERR` writer - Size error
pub type SIZERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PGSERR` reader - Programming sequence error
pub type PGSERR_R = crate::BitReader;
///Field `PGSERR` writer - Programming sequence error
pub type PGSERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MISERR` reader - Fast programming data miss error
pub type MISERR_R = crate::BitReader;
///Field `MISERR` writer - Fast programming data miss error
pub type MISERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FASTERR` reader - Fast programming error
pub type FASTERR_R = crate::BitReader;
///Field `FASTERR` writer - Fast programming error
pub type FASTERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDERR` reader - PCROP read error
pub type RDERR_R = crate::BitReader;
///Field `RDERR` writer - PCROP read error
pub type RDERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPTVERR` reader - Option validity error
pub type OPTVERR_R = crate::BitReader;
///Field `OPTVERR` writer - Option validity error
pub type OPTVERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BSY` reader - Busy
pub type BSY_R = crate::BitReader;
impl R {
    ///Bit 0 - End of operation
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Operation error
    #[inline(always)]
    pub fn operr(&self) -> OPERR_R {
        OPERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Programming error
    #[inline(always)]
    pub fn progerr(&self) -> PROGERR_R {
        PROGERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Write protected error
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Programming alignment error
    #[inline(always)]
    pub fn pgaerr(&self) -> PGAERR_R {
        PGAERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Size error
    #[inline(always)]
    pub fn sizerr(&self) -> SIZERR_R {
        SIZERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Programming sequence error
    #[inline(always)]
    pub fn pgserr(&self) -> PGSERR_R {
        PGSERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Fast programming data miss error
    #[inline(always)]
    pub fn miserr(&self) -> MISERR_R {
        MISERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Fast programming error
    #[inline(always)]
    pub fn fasterr(&self) -> FASTERR_R {
        FASTERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 14 - PCROP read error
    #[inline(always)]
    pub fn rderr(&self) -> RDERR_R {
        RDERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Option validity error
    #[inline(always)]
    pub fn optverr(&self) -> OPTVERR_R {
        OPTVERR_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Busy
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("eop", &self.eop())
            .field("operr", &self.operr())
            .field("progerr", &self.progerr())
            .field("wrperr", &self.wrperr())
            .field("pgaerr", &self.pgaerr())
            .field("sizerr", &self.sizerr())
            .field("pgserr", &self.pgserr())
            .field("miserr", &self.miserr())
            .field("fasterr", &self.fasterr())
            .field("rderr", &self.rderr())
            .field("optverr", &self.optverr())
            .field("bsy", &self.bsy())
            .finish()
    }
}
impl W {
    ///Bit 0 - End of operation
    #[inline(always)]
    pub fn eop(&mut self) -> EOP_W<'_, SRrs> {
        EOP_W::new(self, 0)
    }
    ///Bit 1 - Operation error
    #[inline(always)]
    pub fn operr(&mut self) -> OPERR_W<'_, SRrs> {
        OPERR_W::new(self, 1)
    }
    ///Bit 3 - Programming error
    #[inline(always)]
    pub fn progerr(&mut self) -> PROGERR_W<'_, SRrs> {
        PROGERR_W::new(self, 3)
    }
    ///Bit 4 - Write protected error
    #[inline(always)]
    pub fn wrperr(&mut self) -> WRPERR_W<'_, SRrs> {
        WRPERR_W::new(self, 4)
    }
    ///Bit 5 - Programming alignment error
    #[inline(always)]
    pub fn pgaerr(&mut self) -> PGAERR_W<'_, SRrs> {
        PGAERR_W::new(self, 5)
    }
    ///Bit 6 - Size error
    #[inline(always)]
    pub fn sizerr(&mut self) -> SIZERR_W<'_, SRrs> {
        SIZERR_W::new(self, 6)
    }
    ///Bit 7 - Programming sequence error
    #[inline(always)]
    pub fn pgserr(&mut self) -> PGSERR_W<'_, SRrs> {
        PGSERR_W::new(self, 7)
    }
    ///Bit 8 - Fast programming data miss error
    #[inline(always)]
    pub fn miserr(&mut self) -> MISERR_W<'_, SRrs> {
        MISERR_W::new(self, 8)
    }
    ///Bit 9 - Fast programming error
    #[inline(always)]
    pub fn fasterr(&mut self) -> FASTERR_W<'_, SRrs> {
        FASTERR_W::new(self, 9)
    }
    ///Bit 14 - PCROP read error
    #[inline(always)]
    pub fn rderr(&mut self) -> RDERR_W<'_, SRrs> {
        RDERR_W::new(self, 14)
    }
    ///Bit 15 - Option validity error
    #[inline(always)]
    pub fn optverr(&mut self) -> OPTVERR_W<'_, SRrs> {
        OPTVERR_W::new(self, 15)
    }
}
/**Status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x3.html#FLASH:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
