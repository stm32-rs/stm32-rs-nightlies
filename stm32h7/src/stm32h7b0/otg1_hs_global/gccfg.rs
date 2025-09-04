///Register `GCCFG` reader
pub type R = crate::R<GCCFGrs>;
///Register `GCCFG` writer
pub type W = crate::W<GCCFGrs>;
///Field `DCDET` reader - Data contact detection (DCD) status
pub type DCDET_R = crate::BitReader;
///Field `DCDET` writer - Data contact detection (DCD) status
pub type DCDET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDET` reader - Primary detection (PD) status
pub type PDET_R = crate::BitReader;
///Field `PDET` writer - Primary detection (PD) status
pub type PDET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDET` reader - Secondary detection (SD) status
pub type SDET_R = crate::BitReader;
///Field `SDET` writer - Secondary detection (SD) status
pub type SDET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PS2DET` reader - DM pull-up detection status
pub type PS2DET_R = crate::BitReader;
///Field `PS2DET` writer - DM pull-up detection status
pub type PS2DET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWRDWN` reader - Power down
pub type PWRDWN_R = crate::BitReader;
///Field `PWRDWN` writer - Power down
pub type PWRDWN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BCDEN` reader - Battery charging detector (BCD) enable
pub type BCDEN_R = crate::BitReader;
///Field `BCDEN` writer - Battery charging detector (BCD) enable
pub type BCDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCDEN` reader - Data contact detection (DCD) mode enable
pub type DCDEN_R = crate::BitReader;
///Field `DCDEN` writer - Data contact detection (DCD) mode enable
pub type DCDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDEN` reader - Primary detection (PD) mode enable
pub type PDEN_R = crate::BitReader;
///Field `PDEN` writer - Primary detection (PD) mode enable
pub type PDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDEN` reader - Secondary detection (SD) mode enable
pub type SDEN_R = crate::BitReader;
///Field `SDEN` writer - Secondary detection (SD) mode enable
pub type SDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBDEN` reader - USB VBUS detection enable
pub type VBDEN_R = crate::BitReader;
///Field `VBDEN` writer - USB VBUS detection enable
pub type VBDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Data contact detection (DCD) status
    #[inline(always)]
    pub fn dcdet(&self) -> DCDET_R {
        DCDET_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Primary detection (PD) status
    #[inline(always)]
    pub fn pdet(&self) -> PDET_R {
        PDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Secondary detection (SD) status
    #[inline(always)]
    pub fn sdet(&self) -> SDET_R {
        SDET_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DM pull-up detection status
    #[inline(always)]
    pub fn ps2det(&self) -> PS2DET_R {
        PS2DET_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 16 - Power down
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Battery charging detector (BCD) enable
    #[inline(always)]
    pub fn bcden(&self) -> BCDEN_R {
        BCDEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Data contact detection (DCD) mode enable
    #[inline(always)]
    pub fn dcden(&self) -> DCDEN_R {
        DCDEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Primary detection (PD) mode enable
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Secondary detection (SD) mode enable
    #[inline(always)]
    pub fn sden(&self) -> SDEN_R {
        SDEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - USB VBUS detection enable
    #[inline(always)]
    pub fn vbden(&self) -> VBDEN_R {
        VBDEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GCCFG")
            .field("pwrdwn", &self.pwrdwn())
            .field("bcden", &self.bcden())
            .field("dcden", &self.dcden())
            .field("pden", &self.pden())
            .field("sden", &self.sden())
            .field("vbden", &self.vbden())
            .field("dcdet", &self.dcdet())
            .field("pdet", &self.pdet())
            .field("sdet", &self.sdet())
            .field("ps2det", &self.ps2det())
            .finish()
    }
}
impl W {
    ///Bit 0 - Data contact detection (DCD) status
    #[inline(always)]
    pub fn dcdet(&mut self) -> DCDET_W<GCCFGrs> {
        DCDET_W::new(self, 0)
    }
    ///Bit 1 - Primary detection (PD) status
    #[inline(always)]
    pub fn pdet(&mut self) -> PDET_W<GCCFGrs> {
        PDET_W::new(self, 1)
    }
    ///Bit 2 - Secondary detection (SD) status
    #[inline(always)]
    pub fn sdet(&mut self) -> SDET_W<GCCFGrs> {
        SDET_W::new(self, 2)
    }
    ///Bit 3 - DM pull-up detection status
    #[inline(always)]
    pub fn ps2det(&mut self) -> PS2DET_W<GCCFGrs> {
        PS2DET_W::new(self, 3)
    }
    ///Bit 16 - Power down
    #[inline(always)]
    pub fn pwrdwn(&mut self) -> PWRDWN_W<GCCFGrs> {
        PWRDWN_W::new(self, 16)
    }
    ///Bit 17 - Battery charging detector (BCD) enable
    #[inline(always)]
    pub fn bcden(&mut self) -> BCDEN_W<GCCFGrs> {
        BCDEN_W::new(self, 17)
    }
    ///Bit 18 - Data contact detection (DCD) mode enable
    #[inline(always)]
    pub fn dcden(&mut self) -> DCDEN_W<GCCFGrs> {
        DCDEN_W::new(self, 18)
    }
    ///Bit 19 - Primary detection (PD) mode enable
    #[inline(always)]
    pub fn pden(&mut self) -> PDEN_W<GCCFGrs> {
        PDEN_W::new(self, 19)
    }
    ///Bit 20 - Secondary detection (SD) mode enable
    #[inline(always)]
    pub fn sden(&mut self) -> SDEN_W<GCCFGrs> {
        SDEN_W::new(self, 20)
    }
    ///Bit 21 - USB VBUS detection enable
    #[inline(always)]
    pub fn vbden(&mut self) -> VBDEN_W<GCCFGrs> {
        VBDEN_W::new(self, 21)
    }
}
/**OTG_HS general core configuration register

You can [`read`](crate::Reg::read) this register and get [`gccfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gccfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#OTG1_HS_GLOBAL:GCCFG)*/
pub struct GCCFGrs;
impl crate::RegisterSpec for GCCFGrs {
    type Ux = u32;
}
///`read()` method returns [`gccfg::R`](R) reader structure
impl crate::Readable for GCCFGrs {}
///`write(|w| ..)` method takes [`gccfg::W`](W) writer structure
impl crate::Writable for GCCFGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GCCFG to value 0
impl crate::Resettable for GCCFGrs {}
