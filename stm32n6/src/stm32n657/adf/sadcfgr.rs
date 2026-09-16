///Register `SADCFGR` reader
pub type R = crate::R<SADCFGRrs>;
///Register `SADCFGR` writer
pub type W = crate::W<SADCFGRrs>;
///Field `SNTHR` reader - Signal to noise threshold
pub type SNTHR_R = crate::FieldReader;
///Field `SNTHR` writer - Signal to noise threshold
pub type SNTHR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ANSLP` reader - Ambient noise slope control
pub type ANSLP_R = crate::FieldReader;
///Field `ANSLP` writer - Ambient noise slope control
pub type ANSLP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LFRNB` reader - Number of learning frames
pub type LFRNB_R = crate::FieldReader;
///Field `LFRNB` writer - Number of learning frames
pub type LFRNB_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `HGOVR` reader - Hangover time window
pub type HGOVR_R = crate::FieldReader;
///Field `HGOVR` writer - Hangover time window
pub type HGOVR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ANMIN` reader - Minimum noise level
pub type ANMIN_R = crate::FieldReader<u16>;
///Field `ANMIN` writer - Minimum noise level
pub type ANMIN_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 0:3 - Signal to noise threshold
    #[inline(always)]
    pub fn snthr(&self) -> SNTHR_R {
        SNTHR_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:6 - Ambient noise slope control
    #[inline(always)]
    pub fn anslp(&self) -> ANSLP_R {
        ANSLP_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - Number of learning frames
    #[inline(always)]
    pub fn lfrnb(&self) -> LFRNB_R {
        LFRNB_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:14 - Hangover time window
    #[inline(always)]
    pub fn hgovr(&self) -> HGOVR_R {
        HGOVR_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 16:28 - Minimum noise level
    #[inline(always)]
    pub fn anmin(&self) -> ANMIN_R {
        ANMIN_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SADCFGR")
            .field("snthr", &self.snthr())
            .field("anslp", &self.anslp())
            .field("lfrnb", &self.lfrnb())
            .field("hgovr", &self.hgovr())
            .field("anmin", &self.anmin())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Signal to noise threshold
    #[inline(always)]
    pub fn snthr(&mut self) -> SNTHR_W<'_, SADCFGRrs> {
        SNTHR_W::new(self, 0)
    }
    ///Bits 4:6 - Ambient noise slope control
    #[inline(always)]
    pub fn anslp(&mut self) -> ANSLP_W<'_, SADCFGRrs> {
        ANSLP_W::new(self, 4)
    }
    ///Bits 8:10 - Number of learning frames
    #[inline(always)]
    pub fn lfrnb(&mut self) -> LFRNB_W<'_, SADCFGRrs> {
        LFRNB_W::new(self, 8)
    }
    ///Bits 12:14 - Hangover time window
    #[inline(always)]
    pub fn hgovr(&mut self) -> HGOVR_W<'_, SADCFGRrs> {
        HGOVR_W::new(self, 12)
    }
    ///Bits 16:28 - Minimum noise level
    #[inline(always)]
    pub fn anmin(&mut self) -> ANMIN_W<'_, SADCFGRrs> {
        ANMIN_W::new(self, 16)
    }
}
/**ADF SAD configuration register

You can [`read`](crate::Reg::read) this register and get [`sadcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sadcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADF:SADCFGR)*/
pub struct SADCFGRrs;
impl crate::RegisterSpec for SADCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`sadcfgr::R`](R) reader structure
impl crate::Readable for SADCFGRrs {}
///`write(|w| ..)` method takes [`sadcfgr::W`](W) writer structure
impl crate::Writable for SADCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SADCFGR to value 0
impl crate::Resettable for SADCFGRrs {}
