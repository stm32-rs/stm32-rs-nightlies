///Register `BCDR` reader
pub type R = crate::R<BCDRrs>;
///Register `BCDR` writer
pub type W = crate::W<BCDRrs>;
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
///Field `DCDET` reader - Data contact detection (DCD) status
pub type DCDET_R = crate::BitReader;
///Field `PDET` reader - Primary detection (PD) status
pub type PDET_R = crate::BitReader;
///Field `SDET` reader - Secondary detection (SD) status
pub type SDET_R = crate::BitReader;
///Field `PS2DET` reader - DM pull-up detection status
pub type PS2DET_R = crate::BitReader;
///Field `DPPU_DPD` reader - DP pull-up / DPDM pull-down
pub type DPPU_DPD_R = crate::BitReader;
///Field `DPPU_DPD` writer - DP pull-up / DPDM pull-down
pub type DPPU_DPD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Battery charging detector (BCD) enable
    #[inline(always)]
    pub fn bcden(&self) -> BCDEN_R {
        BCDEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Data contact detection (DCD) mode enable
    #[inline(always)]
    pub fn dcden(&self) -> DCDEN_R {
        DCDEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Primary detection (PD) mode enable
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Secondary detection (SD) mode enable
    #[inline(always)]
    pub fn sden(&self) -> SDEN_R {
        SDEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Data contact detection (DCD) status
    #[inline(always)]
    pub fn dcdet(&self) -> DCDET_R {
        DCDET_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Primary detection (PD) status
    #[inline(always)]
    pub fn pdet(&self) -> PDET_R {
        PDET_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Secondary detection (SD) status
    #[inline(always)]
    pub fn sdet(&self) -> SDET_R {
        SDET_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - DM pull-up detection status
    #[inline(always)]
    pub fn ps2det(&self) -> PS2DET_R {
        PS2DET_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 15 - DP pull-up / DPDM pull-down
    #[inline(always)]
    pub fn dppu_dpd(&self) -> DPPU_DPD_R {
        DPPU_DPD_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCDR")
            .field("bcden", &self.bcden())
            .field("dcden", &self.dcden())
            .field("pden", &self.pden())
            .field("sden", &self.sden())
            .field("dcdet", &self.dcdet())
            .field("pdet", &self.pdet())
            .field("sdet", &self.sdet())
            .field("ps2det", &self.ps2det())
            .field("dppu_dpd", &self.dppu_dpd())
            .finish()
    }
}
impl W {
    ///Bit 0 - Battery charging detector (BCD) enable
    #[inline(always)]
    pub fn bcden(&mut self) -> BCDEN_W<'_, BCDRrs> {
        BCDEN_W::new(self, 0)
    }
    ///Bit 1 - Data contact detection (DCD) mode enable
    #[inline(always)]
    pub fn dcden(&mut self) -> DCDEN_W<'_, BCDRrs> {
        DCDEN_W::new(self, 1)
    }
    ///Bit 2 - Primary detection (PD) mode enable
    #[inline(always)]
    pub fn pden(&mut self) -> PDEN_W<'_, BCDRrs> {
        PDEN_W::new(self, 2)
    }
    ///Bit 3 - Secondary detection (SD) mode enable
    #[inline(always)]
    pub fn sden(&mut self) -> SDEN_W<'_, BCDRrs> {
        SDEN_W::new(self, 3)
    }
    ///Bit 15 - DP pull-up / DPDM pull-down
    #[inline(always)]
    pub fn dppu_dpd(&mut self) -> DPPU_DPD_W<'_, BCDRrs> {
        DPPU_DPD_W::new(self, 15)
    }
}
/**Battery charging detector

You can [`read`](crate::Reg::read) this register and get [`bcdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#USB:BCDR)*/
pub struct BCDRrs;
impl crate::RegisterSpec for BCDRrs {
    type Ux = u32;
}
///`read()` method returns [`bcdr::R`](R) reader structure
impl crate::Readable for BCDRrs {}
///`write(|w| ..)` method takes [`bcdr::W`](W) writer structure
impl crate::Writable for BCDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCDR to value 0
impl crate::Resettable for BCDRrs {}
