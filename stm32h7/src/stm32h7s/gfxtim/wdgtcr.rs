///Register `WDGTCR` reader
pub type R = crate::R<WDGTCRrs>;
///Register `WDGTCR` writer
pub type W = crate::W<WDGTCRrs>;
///Field `WDGEN` writer - watchdog enable This bit enables the graphic watchdog.
pub type WDGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDGDIS` writer - watchdog disable This bit disables the graphic watchdog.
pub type WDGDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDGS` reader - watchdog status This bit returns the status of the graphic watchdog.
pub type WDGS_R = crate::BitReader;
///Field `WDGHRC` reader - watchdog hardware reload configuration This field configures the watchdog hardware reload.
pub type WDGHRC_R = crate::FieldReader;
///Field `WDGHRC` writer - watchdog hardware reload configuration This field configures the watchdog hardware reload.
pub type WDGHRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WDGCS` reader - watchdog clock source This field selects the watchdog clock source. others: reserved
pub type WDGCS_R = crate::FieldReader;
///Field `WDGCS` writer - watchdog clock source This field selects the watchdog clock source. others: reserved
pub type WDGCS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `FWDGR` writer - force watchdog reload This bit forces the reload of the graphic watchdog.
pub type FWDGR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - watchdog status This bit returns the status of the graphic watchdog.
    #[inline(always)]
    pub fn wdgs(&self) -> WDGS_R {
        WDGS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:5 - watchdog hardware reload configuration This field configures the watchdog hardware reload.
    #[inline(always)]
    pub fn wdghrc(&self) -> WDGHRC_R {
        WDGHRC_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:11 - watchdog clock source This field selects the watchdog clock source. others: reserved
    #[inline(always)]
    pub fn wdgcs(&self) -> WDGCS_R {
        WDGCS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDGTCR")
            .field("wdgs", &self.wdgs())
            .field("wdghrc", &self.wdghrc())
            .field("wdgcs", &self.wdgcs())
            .finish()
    }
}
impl W {
    ///Bit 0 - watchdog enable This bit enables the graphic watchdog.
    #[inline(always)]
    pub fn wdgen(&mut self) -> WDGEN_W<'_, WDGTCRrs> {
        WDGEN_W::new(self, 0)
    }
    ///Bit 1 - watchdog disable This bit disables the graphic watchdog.
    #[inline(always)]
    pub fn wdgdis(&mut self) -> WDGDIS_W<'_, WDGTCRrs> {
        WDGDIS_W::new(self, 1)
    }
    ///Bits 4:5 - watchdog hardware reload configuration This field configures the watchdog hardware reload.
    #[inline(always)]
    pub fn wdghrc(&mut self) -> WDGHRC_W<'_, WDGTCRrs> {
        WDGHRC_W::new(self, 4)
    }
    ///Bits 8:11 - watchdog clock source This field selects the watchdog clock source. others: reserved
    #[inline(always)]
    pub fn wdgcs(&mut self) -> WDGCS_W<'_, WDGTCRrs> {
        WDGCS_W::new(self, 8)
    }
    ///Bit 16 - force watchdog reload This bit forces the reload of the graphic watchdog.
    #[inline(always)]
    pub fn fwdgr(&mut self) -> FWDGR_W<'_, WDGTCRrs> {
        FWDGR_W::new(self, 16)
    }
}
/**GFXTIM watchdog timer configuration register

You can [`read`](crate::Reg::read) this register and get [`wdgtcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdgtcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#GFXTIM:WDGTCR)*/
pub struct WDGTCRrs;
impl crate::RegisterSpec for WDGTCRrs {
    type Ux = u32;
}
///`read()` method returns [`wdgtcr::R`](R) reader structure
impl crate::Readable for WDGTCRrs {}
///`write(|w| ..)` method takes [`wdgtcr::W`](W) writer structure
impl crate::Writable for WDGTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WDGTCR to value 0
impl crate::Resettable for WDGTCRrs {}
