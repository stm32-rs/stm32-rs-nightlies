///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
///Field `BSY` reader - Write/erase operations in progress
pub type BSY_R = crate::BitReader;
///Field `EOP` reader - End of operation
pub type EOP_R = crate::BitReader;
///Field `ENDHV` reader - End of high voltage
pub type ENDHV_R = crate::BitReader;
///Field `READY` reader - Flash memory module ready after low power mode
pub type READY_R = crate::BitReader;
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
///Field `OPTVERR` reader - Option validity error
pub type OPTVERR_R = crate::BitReader;
///Field `OPTVERR` writer - Option validity error
pub type OPTVERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPTVERRUSR` reader - Option UserValidity Error
pub type OPTVERRUSR_R = crate::BitReader;
///Field `OPTVERRUSR` writer - Option UserValidity Error
pub type OPTVERRUSR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Write/erase operations in progress
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End of operation
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - End of high voltage
    #[inline(always)]
    pub fn endhv(&self) -> ENDHV_R {
        ENDHV_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Flash memory module ready after low power mode
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - Write protected error
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Programming alignment error
    #[inline(always)]
    pub fn pgaerr(&self) -> PGAERR_R {
        PGAERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Size error
    #[inline(always)]
    pub fn sizerr(&self) -> SIZERR_R {
        SIZERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Option validity error
    #[inline(always)]
    pub fn optverr(&self) -> OPTVERR_R {
        OPTVERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Option UserValidity Error
    #[inline(always)]
    pub fn optverrusr(&self) -> OPTVERRUSR_R {
        OPTVERRUSR_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("bsy", &self.bsy())
            .field("eop", &self.eop())
            .field("endhv", &self.endhv())
            .field("ready", &self.ready())
            .field("wrperr", &self.wrperr())
            .field("pgaerr", &self.pgaerr())
            .field("sizerr", &self.sizerr())
            .field("optverr", &self.optverr())
            .field("optverrusr", &self.optverrusr())
            .finish()
    }
}
impl W {
    ///Bit 8 - Write protected error
    #[inline(always)]
    pub fn wrperr(&mut self) -> WRPERR_W<'_, SRrs> {
        WRPERR_W::new(self, 8)
    }
    ///Bit 9 - Programming alignment error
    #[inline(always)]
    pub fn pgaerr(&mut self) -> PGAERR_W<'_, SRrs> {
        PGAERR_W::new(self, 9)
    }
    ///Bit 10 - Size error
    #[inline(always)]
    pub fn sizerr(&mut self) -> SIZERR_W<'_, SRrs> {
        SIZERR_W::new(self, 10)
    }
    ///Bit 11 - Option validity error
    #[inline(always)]
    pub fn optverr(&mut self) -> OPTVERR_W<'_, SRrs> {
        OPTVERR_W::new(self, 11)
    }
    ///Bit 12 - Option UserValidity Error
    #[inline(always)]
    pub fn optverrusr(&mut self) -> OPTVERRUSR_W<'_, SRrs> {
        OPTVERRUSR_W::new(self, 12)
    }
}
/**Status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L162.html#Flash:SR)*/
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
///`reset()` method sets SR to value 0x04
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x04;
}
