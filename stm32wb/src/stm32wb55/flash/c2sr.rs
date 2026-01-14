///Register `C2SR` reader
pub type R = crate::R<C2SRrs>;
///Register `C2SR` writer
pub type W = crate::W<C2SRrs>;
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
///Field `WRPERR` reader - write protection error
pub type WRPERR_R = crate::BitReader;
///Field `WRPERR` writer - write protection error
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
///Field `MISSERR` reader - Fast programming data miss error
pub type MISSERR_R = crate::BitReader;
///Field `MISSERR` writer - Fast programming data miss error
pub type MISSERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FASTERR` reader - Fast programming error
pub type FASTERR_R = crate::BitReader;
///Field `FASTERR` writer - Fast programming error
pub type FASTERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDERR` reader - PCROP read error
pub type RDERR_R = crate::BitReader;
///Field `RDERR` writer - PCROP read error
pub type RDERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BSY` reader - Busy
pub type BSY_R = crate::BitReader;
///Field `BSY` writer - Busy
pub type BSY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFGBSY` reader - Programming or erase configuration busy
pub type CFGBSY_R = crate::BitReader;
///Field `CFGBSY` writer - Programming or erase configuration busy
pub type CFGBSY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PESD` reader - Programming or erase operation suspended
pub type PESD_R = crate::BitReader;
///Field `PESD` writer - Programming or erase operation suspended
pub type PESD_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 4 - write protection error
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
    pub fn misserr(&self) -> MISSERR_R {
        MISSERR_R::new(((self.bits >> 8) & 1) != 0)
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
    ///Bit 16 - Busy
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Programming or erase configuration busy
    #[inline(always)]
    pub fn cfgbsy(&self) -> CFGBSY_R {
        CFGBSY_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Programming or erase operation suspended
    #[inline(always)]
    pub fn pesd(&self) -> PESD_R {
        PESD_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2SR")
            .field("eop", &self.eop())
            .field("operr", &self.operr())
            .field("progerr", &self.progerr())
            .field("wrperr", &self.wrperr())
            .field("pgaerr", &self.pgaerr())
            .field("sizerr", &self.sizerr())
            .field("pgserr", &self.pgserr())
            .field("misserr", &self.misserr())
            .field("fasterr", &self.fasterr())
            .field("rderr", &self.rderr())
            .field("bsy", &self.bsy())
            .field("cfgbsy", &self.cfgbsy())
            .field("pesd", &self.pesd())
            .finish()
    }
}
impl W {
    ///Bit 0 - End of operation
    #[inline(always)]
    pub fn eop(&mut self) -> EOP_W<'_, C2SRrs> {
        EOP_W::new(self, 0)
    }
    ///Bit 1 - Operation error
    #[inline(always)]
    pub fn operr(&mut self) -> OPERR_W<'_, C2SRrs> {
        OPERR_W::new(self, 1)
    }
    ///Bit 3 - Programming error
    #[inline(always)]
    pub fn progerr(&mut self) -> PROGERR_W<'_, C2SRrs> {
        PROGERR_W::new(self, 3)
    }
    ///Bit 4 - write protection error
    #[inline(always)]
    pub fn wrperr(&mut self) -> WRPERR_W<'_, C2SRrs> {
        WRPERR_W::new(self, 4)
    }
    ///Bit 5 - Programming alignment error
    #[inline(always)]
    pub fn pgaerr(&mut self) -> PGAERR_W<'_, C2SRrs> {
        PGAERR_W::new(self, 5)
    }
    ///Bit 6 - Size error
    #[inline(always)]
    pub fn sizerr(&mut self) -> SIZERR_W<'_, C2SRrs> {
        SIZERR_W::new(self, 6)
    }
    ///Bit 7 - Programming sequence error
    #[inline(always)]
    pub fn pgserr(&mut self) -> PGSERR_W<'_, C2SRrs> {
        PGSERR_W::new(self, 7)
    }
    ///Bit 8 - Fast programming data miss error
    #[inline(always)]
    pub fn misserr(&mut self) -> MISSERR_W<'_, C2SRrs> {
        MISSERR_W::new(self, 8)
    }
    ///Bit 9 - Fast programming error
    #[inline(always)]
    pub fn fasterr(&mut self) -> FASTERR_W<'_, C2SRrs> {
        FASTERR_W::new(self, 9)
    }
    ///Bit 14 - PCROP read error
    #[inline(always)]
    pub fn rderr(&mut self) -> RDERR_W<'_, C2SRrs> {
        RDERR_W::new(self, 14)
    }
    ///Bit 16 - Busy
    #[inline(always)]
    pub fn bsy(&mut self) -> BSY_W<'_, C2SRrs> {
        BSY_W::new(self, 16)
    }
    ///Bit 18 - Programming or erase configuration busy
    #[inline(always)]
    pub fn cfgbsy(&mut self) -> CFGBSY_W<'_, C2SRrs> {
        CFGBSY_W::new(self, 18)
    }
    ///Bit 19 - Programming or erase operation suspended
    #[inline(always)]
    pub fn pesd(&mut self) -> PESD_W<'_, C2SRrs> {
        PESD_W::new(self, 19)
    }
}
/**CPU2 cortex M0 status register

You can [`read`](crate::Reg::read) this register and get [`c2sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#Flash:C2SR)*/
pub struct C2SRrs;
impl crate::RegisterSpec for C2SRrs {
    type Ux = u32;
}
///`read()` method returns [`c2sr::R`](R) reader structure
impl crate::Readable for C2SRrs {}
///`write(|w| ..)` method takes [`c2sr::W`](W) writer structure
impl crate::Writable for C2SRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2SR to value 0
impl crate::Resettable for C2SRrs {}
