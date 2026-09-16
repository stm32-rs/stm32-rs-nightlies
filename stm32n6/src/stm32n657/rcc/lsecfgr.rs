///Register `LSECFGR` reader
pub type R = crate::R<LSECFGRrs>;
///Register `LSECFGR` writer
pub type W = crate::W<LSECFGRrs>;
///Field `LSECSSON` writer - LSE clock security system (CSS) enable
pub type LSECSSON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSECSSRA` reader - LSE clock security system (CSS) re-arm function
pub type LSECSSRA_R = crate::BitReader;
///Field `LSECSSRA` writer - LSE clock security system (CSS) re-arm function
pub type LSECSSRA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSECSSD` reader - LSE clock security system (CSS) failure detection
pub type LSECSSD_R = crate::BitReader;
///Field `LSEBYP` reader - LSE clock bypass
pub type LSEBYP_R = crate::BitReader;
///Field `LSEBYP` writer - LSE clock bypass
pub type LSEBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSEEXT` reader - LSE clock type in Bypass mode
pub type LSEEXT_R = crate::BitReader;
///Field `LSEEXT` writer - LSE clock type in Bypass mode
pub type LSEEXT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSEGFON` reader - LSE clock glitch filter enable
pub type LSEGFON_R = crate::BitReader;
///Field `LSEGFON` writer - LSE clock glitch filter enable
pub type LSEGFON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSEDRV` reader - LSE oscillator driving capability
pub type LSEDRV_R = crate::FieldReader;
///Field `LSEDRV` writer - LSE oscillator driving capability
pub type LSEDRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 8 - LSE clock security system (CSS) re-arm function
    #[inline(always)]
    pub fn lsecssra(&self) -> LSECSSRA_R {
        LSECSSRA_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LSE clock security system (CSS) failure detection
    #[inline(always)]
    pub fn lsecssd(&self) -> LSECSSD_R {
        LSECSSD_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 15 - LSE clock bypass
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - LSE clock type in Bypass mode
    #[inline(always)]
    pub fn lseext(&self) -> LSEEXT_R {
        LSEEXT_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - LSE clock glitch filter enable
    #[inline(always)]
    pub fn lsegfon(&self) -> LSEGFON_R {
        LSEGFON_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:19 - LSE oscillator driving capability
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LSECFGR")
            .field("lsecssra", &self.lsecssra())
            .field("lsecssd", &self.lsecssd())
            .field("lsebyp", &self.lsebyp())
            .field("lseext", &self.lseext())
            .field("lsegfon", &self.lsegfon())
            .field("lsedrv", &self.lsedrv())
            .finish()
    }
}
impl W {
    ///Bit 7 - LSE clock security system (CSS) enable
    #[inline(always)]
    pub fn lsecsson(&mut self) -> LSECSSON_W<'_, LSECFGRrs> {
        LSECSSON_W::new(self, 7)
    }
    ///Bit 8 - LSE clock security system (CSS) re-arm function
    #[inline(always)]
    pub fn lsecssra(&mut self) -> LSECSSRA_W<'_, LSECFGRrs> {
        LSECSSRA_W::new(self, 8)
    }
    ///Bit 15 - LSE clock bypass
    #[inline(always)]
    pub fn lsebyp(&mut self) -> LSEBYP_W<'_, LSECFGRrs> {
        LSEBYP_W::new(self, 15)
    }
    ///Bit 16 - LSE clock type in Bypass mode
    #[inline(always)]
    pub fn lseext(&mut self) -> LSEEXT_W<'_, LSECFGRrs> {
        LSEEXT_W::new(self, 16)
    }
    ///Bit 17 - LSE clock glitch filter enable
    #[inline(always)]
    pub fn lsegfon(&mut self) -> LSEGFON_W<'_, LSECFGRrs> {
        LSEGFON_W::new(self, 17)
    }
    ///Bits 18:19 - LSE oscillator driving capability
    #[inline(always)]
    pub fn lsedrv(&mut self) -> LSEDRV_W<'_, LSECFGRrs> {
        LSEDRV_W::new(self, 18)
    }
}
/**RCC LSE configuration register

You can [`read`](crate::Reg::read) this register and get [`lsecfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lsecfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:LSECFGR)*/
pub struct LSECFGRrs;
impl crate::RegisterSpec for LSECFGRrs {
    type Ux = u32;
}
///`read()` method returns [`lsecfgr::R`](R) reader structure
impl crate::Readable for LSECFGRrs {}
///`write(|w| ..)` method takes [`lsecfgr::W`](W) writer structure
impl crate::Writable for LSECFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LSECFGR to value 0
impl crate::Resettable for LSECFGRrs {}
