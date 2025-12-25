///Register `GCCFG` reader
pub type R = crate::R<GCCFGrs>;
///Register `GCCFG` writer
pub type W = crate::W<GCCFGrs>;
///Field `CHGDET` reader - Charger detection, result of the current mode (primary or secondary).
pub type CHGDET_R = crate::BitReader;
///Field `FSVPLUS` reader - Single-Ended DP indicator This bit gives the voltage level on DP (also result of the comparison with V<sub>LGC</sub> threshold as defined in BC v1.2 standard).
pub type FSVPLUS_R = crate::BitReader;
///Field `FSVMINUS` reader - Single-Ended DM indicator This bit gives the voltage level on DM (also result of the comparison with V<sub>LGC</sub> threshold as defined in BC v1.2 standard).
pub type FSVMINUS_R = crate::BitReader;
///Field `SESSVLD` reader - VBUS session indicator Indicates if VBUS is above VBUS session threshold.
pub type SESSVLD_R = crate::BitReader;
///Field `HCDPEN` reader - Host CDP behavior enable
pub type HCDPEN_R = crate::BitReader;
///Field `HCDPEN` writer - Host CDP behavior enable
pub type HCDPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HCDPDETEN` reader - Host CDP port voltage detector enable on DP
pub type HCDPDETEN_R = crate::BitReader;
///Field `HCDPDETEN` writer - Host CDP port voltage detector enable on DP
pub type HCDPDETEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HVDMSRCEN` reader - Host CDP port Voltage source enable on DM
pub type HVDMSRCEN_R = crate::BitReader;
///Field `HVDMSRCEN` writer - Host CDP port Voltage source enable on DM
pub type HVDMSRCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCDEN` reader - Data Contact Detection enable
pub type DCDEN_R = crate::BitReader;
///Field `DCDEN` writer - Data Contact Detection enable
pub type DCDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDEN` reader - Primary detection enable
pub type PDEN_R = crate::BitReader;
///Field `PDEN` writer - Primary detection enable
pub type PDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBDEN` reader - VBUS detection enable Enables VBUS Sensing Comparators in order to detect VBUS presence and/or perform OTG operation.
pub type VBDEN_R = crate::BitReader;
///Field `VBDEN` writer - VBUS detection enable Enables VBUS Sensing Comparators in order to detect VBUS presence and/or perform OTG operation.
pub type VBDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDEN` reader - Secondary detection enable
pub type SDEN_R = crate::BitReader;
///Field `SDEN` writer - Secondary detection enable
pub type SDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBVALOVAL` reader - Software override value of the VBUS B-session detection
pub type VBVALOVAL_R = crate::BitReader;
///Field `VBVALOVAL` writer - Software override value of the VBUS B-session detection
pub type VBVALOVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBVALOVEN` reader - Enables a software override of the VBUS B-session detection.
pub type VBVALOVEN_R = crate::BitReader;
///Field `VBVALOVEN` writer - Enables a software override of the VBUS B-session detection.
pub type VBVALOVEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FORCEHOSTPD` reader - Force host mode pull-downs If the ID pin functions are enabled, the host mode pull-downs on DP and DM activate automatically. However, whenever that is not the case, yet host mode is required, this bit must be used to force the pull-downs active.
pub type FORCEHOSTPD_R = crate::BitReader;
///Field `FORCEHOSTPD` writer - Force host mode pull-downs If the ID pin functions are enabled, the host mode pull-downs on DP and DM activate automatically. However, whenever that is not the case, yet host mode is required, this bit must be used to force the pull-downs active.
pub type FORCEHOSTPD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Charger detection, result of the current mode (primary or secondary).
    #[inline(always)]
    pub fn chgdet(&self) -> CHGDET_R {
        CHGDET_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Single-Ended DP indicator This bit gives the voltage level on DP (also result of the comparison with V<sub>LGC</sub> threshold as defined in BC v1.2 standard).
    #[inline(always)]
    pub fn fsvplus(&self) -> FSVPLUS_R {
        FSVPLUS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Single-Ended DM indicator This bit gives the voltage level on DM (also result of the comparison with V<sub>LGC</sub> threshold as defined in BC v1.2 standard).
    #[inline(always)]
    pub fn fsvminus(&self) -> FSVMINUS_R {
        FSVMINUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - VBUS session indicator Indicates if VBUS is above VBUS session threshold.
    #[inline(always)]
    pub fn sessvld(&self) -> SESSVLD_R {
        SESSVLD_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 16 - Host CDP behavior enable
    #[inline(always)]
    pub fn hcdpen(&self) -> HCDPEN_R {
        HCDPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Host CDP port voltage detector enable on DP
    #[inline(always)]
    pub fn hcdpdeten(&self) -> HCDPDETEN_R {
        HCDPDETEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Host CDP port Voltage source enable on DM
    #[inline(always)]
    pub fn hvdmsrcen(&self) -> HVDMSRCEN_R {
        HVDMSRCEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Data Contact Detection enable
    #[inline(always)]
    pub fn dcden(&self) -> DCDEN_R {
        DCDEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Primary detection enable
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - VBUS detection enable Enables VBUS Sensing Comparators in order to detect VBUS presence and/or perform OTG operation.
    #[inline(always)]
    pub fn vbden(&self) -> VBDEN_R {
        VBDEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Secondary detection enable
    #[inline(always)]
    pub fn sden(&self) -> SDEN_R {
        SDEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Software override value of the VBUS B-session detection
    #[inline(always)]
    pub fn vbvaloval(&self) -> VBVALOVAL_R {
        VBVALOVAL_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Enables a software override of the VBUS B-session detection.
    #[inline(always)]
    pub fn vbvaloven(&self) -> VBVALOVEN_R {
        VBVALOVEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Force host mode pull-downs If the ID pin functions are enabled, the host mode pull-downs on DP and DM activate automatically. However, whenever that is not the case, yet host mode is required, this bit must be used to force the pull-downs active.
    #[inline(always)]
    pub fn forcehostpd(&self) -> FORCEHOSTPD_R {
        FORCEHOSTPD_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GCCFG")
            .field("chgdet", &self.chgdet())
            .field("fsvplus", &self.fsvplus())
            .field("fsvminus", &self.fsvminus())
            .field("sessvld", &self.sessvld())
            .field("hcdpen", &self.hcdpen())
            .field("hcdpdeten", &self.hcdpdeten())
            .field("hvdmsrcen", &self.hvdmsrcen())
            .field("dcden", &self.dcden())
            .field("pden", &self.pden())
            .field("vbden", &self.vbden())
            .field("sden", &self.sden())
            .field("vbvaloval", &self.vbvaloval())
            .field("vbvaloven", &self.vbvaloven())
            .field("forcehostpd", &self.forcehostpd())
            .finish()
    }
}
impl W {
    ///Bit 16 - Host CDP behavior enable
    #[inline(always)]
    pub fn hcdpen(&mut self) -> HCDPEN_W<'_, GCCFGrs> {
        HCDPEN_W::new(self, 16)
    }
    ///Bit 17 - Host CDP port voltage detector enable on DP
    #[inline(always)]
    pub fn hcdpdeten(&mut self) -> HCDPDETEN_W<'_, GCCFGrs> {
        HCDPDETEN_W::new(self, 17)
    }
    ///Bit 18 - Host CDP port Voltage source enable on DM
    #[inline(always)]
    pub fn hvdmsrcen(&mut self) -> HVDMSRCEN_W<'_, GCCFGrs> {
        HVDMSRCEN_W::new(self, 18)
    }
    ///Bit 19 - Data Contact Detection enable
    #[inline(always)]
    pub fn dcden(&mut self) -> DCDEN_W<'_, GCCFGrs> {
        DCDEN_W::new(self, 19)
    }
    ///Bit 20 - Primary detection enable
    #[inline(always)]
    pub fn pden(&mut self) -> PDEN_W<'_, GCCFGrs> {
        PDEN_W::new(self, 20)
    }
    ///Bit 21 - VBUS detection enable Enables VBUS Sensing Comparators in order to detect VBUS presence and/or perform OTG operation.
    #[inline(always)]
    pub fn vbden(&mut self) -> VBDEN_W<'_, GCCFGrs> {
        VBDEN_W::new(self, 21)
    }
    ///Bit 22 - Secondary detection enable
    #[inline(always)]
    pub fn sden(&mut self) -> SDEN_W<'_, GCCFGrs> {
        SDEN_W::new(self, 22)
    }
    ///Bit 23 - Software override value of the VBUS B-session detection
    #[inline(always)]
    pub fn vbvaloval(&mut self) -> VBVALOVAL_W<'_, GCCFGrs> {
        VBVALOVAL_W::new(self, 23)
    }
    ///Bit 24 - Enables a software override of the VBUS B-session detection.
    #[inline(always)]
    pub fn vbvaloven(&mut self) -> VBVALOVEN_W<'_, GCCFGrs> {
        VBVALOVEN_W::new(self, 24)
    }
    ///Bit 25 - Force host mode pull-downs If the ID pin functions are enabled, the host mode pull-downs on DP and DM activate automatically. However, whenever that is not the case, yet host mode is required, this bit must be used to force the pull-downs active.
    #[inline(always)]
    pub fn forcehostpd(&mut self) -> FORCEHOSTPD_W<'_, GCCFGrs> {
        FORCEHOSTPD_W::new(self, 25)
    }
}
/**OTG general core configuration register

You can [`read`](crate::Reg::read) this register and get [`gccfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gccfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#OTG_HS:GCCFG)*/
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
