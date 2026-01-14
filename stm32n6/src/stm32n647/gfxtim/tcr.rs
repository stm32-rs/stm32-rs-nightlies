///Register `TCR` reader
pub type R = crate::R<TCRrs>;
///Register `TCR` writer
pub type W = crate::W<TCRrs>;
///Field `AFCEN` writer - absolute frame counter enable
pub type AFCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FAFCR` writer - force absolute frame counter reset
pub type FAFCR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALCEN` writer - absolute line counter enable
pub type ALCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FALCR` writer - force absolute line counter reset
pub type FALCR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFC1EN` writer - relative frame counter 1 enable
pub type RFC1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFC1CM` reader - relative frame counter 1 continuous mode
pub type RFC1CM_R = crate::BitReader;
///Field `RFC1CM` writer - relative frame counter 1 continuous mode
pub type RFC1CM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRFC1R` writer - force relative frame counter 1 reload
pub type FRFC1R_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFC2EN` writer - relative frame counter 2 enable
pub type RFC2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFC2CM` reader - relative frame counter 2 continuous mode
pub type RFC2CM_R = crate::BitReader;
///Field `RFC2CM` writer - relative frame counter 2 continuous mode
pub type RFC2CM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRFC2R` writer - force relative frame counter 2 reload
pub type FRFC2R_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 17 - relative frame counter 1 continuous mode
    #[inline(always)]
    pub fn rfc1cm(&self) -> RFC1CM_R {
        RFC1CM_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 21 - relative frame counter 2 continuous mode
    #[inline(always)]
    pub fn rfc2cm(&self) -> RFC2CM_R {
        RFC2CM_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCR")
            .field("rfc1cm", &self.rfc1cm())
            .field("rfc2cm", &self.rfc2cm())
            .finish()
    }
}
impl W {
    ///Bit 0 - absolute frame counter enable
    #[inline(always)]
    pub fn afcen(&mut self) -> AFCEN_W<'_, TCRrs> {
        AFCEN_W::new(self, 0)
    }
    ///Bit 1 - force absolute frame counter reset
    #[inline(always)]
    pub fn fafcr(&mut self) -> FAFCR_W<'_, TCRrs> {
        FAFCR_W::new(self, 1)
    }
    ///Bit 4 - absolute line counter enable
    #[inline(always)]
    pub fn alcen(&mut self) -> ALCEN_W<'_, TCRrs> {
        ALCEN_W::new(self, 4)
    }
    ///Bit 5 - force absolute line counter reset
    #[inline(always)]
    pub fn falcr(&mut self) -> FALCR_W<'_, TCRrs> {
        FALCR_W::new(self, 5)
    }
    ///Bit 16 - relative frame counter 1 enable
    #[inline(always)]
    pub fn rfc1en(&mut self) -> RFC1EN_W<'_, TCRrs> {
        RFC1EN_W::new(self, 16)
    }
    ///Bit 17 - relative frame counter 1 continuous mode
    #[inline(always)]
    pub fn rfc1cm(&mut self) -> RFC1CM_W<'_, TCRrs> {
        RFC1CM_W::new(self, 17)
    }
    ///Bit 18 - force relative frame counter 1 reload
    #[inline(always)]
    pub fn frfc1r(&mut self) -> FRFC1R_W<'_, TCRrs> {
        FRFC1R_W::new(self, 18)
    }
    ///Bit 20 - relative frame counter 2 enable
    #[inline(always)]
    pub fn rfc2en(&mut self) -> RFC2EN_W<'_, TCRrs> {
        RFC2EN_W::new(self, 20)
    }
    ///Bit 21 - relative frame counter 2 continuous mode
    #[inline(always)]
    pub fn rfc2cm(&mut self) -> RFC2CM_W<'_, TCRrs> {
        RFC2CM_W::new(self, 21)
    }
    ///Bit 22 - force relative frame counter 2 reload
    #[inline(always)]
    pub fn frfc2r(&mut self) -> FRFC2R_W<'_, TCRrs> {
        FRFC2R_W::new(self, 22)
    }
}
/**GFXTIM timers configuration register

You can [`read`](crate::Reg::read) this register and get [`tcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#GFXTIM:TCR)*/
pub struct TCRrs;
impl crate::RegisterSpec for TCRrs {
    type Ux = u32;
}
///`read()` method returns [`tcr::R`](R) reader structure
impl crate::Readable for TCRrs {}
///`write(|w| ..)` method takes [`tcr::W`](W) writer structure
impl crate::Writable for TCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TCR to value 0
impl crate::Resettable for TCRrs {}
