///Register `BCDR` reader
pub type R = crate::R<BCDRrs>;
///Register `BCDR` writer
pub type W = crate::W<BCDRrs>;
///Field `BCDEN` reader - Battery charging detector mode enable
pub type BCDEN_R = crate::BitReader;
///Field `BCDEN` writer - Battery charging detector mode enable
pub type BCDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCDEN` reader - Data contact detection mode enable
pub type DCDEN_R = crate::BitReader;
///Field `DCDEN` writer - Data contact detection mode enable
pub type DCDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDEN` reader - Primary detection mode enable
pub type PDEN_R = crate::BitReader;
///Field `PDEN` writer - Primary detection mode enable
pub type PDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDEN` reader - Secondary detection mode enable
pub type SDEN_R = crate::BitReader;
///Field `SDEN` writer - Secondary detection mode enable
pub type SDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCDET` reader - Data contact detection status
pub type DCDET_R = crate::BitReader;
///Field `DCDET` writer - Data contact detection status
pub type DCDET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDET` reader - Primary detection status
pub type PDET_R = crate::BitReader;
///Field `PDET` writer - Primary detection status
pub type PDET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDET` reader - Secondary detection status
pub type SDET_R = crate::BitReader;
///Field `SDET` writer - Secondary detection status
pub type SDET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PS2DET` reader - DM pull-up detection status
pub type PS2DET_R = crate::BitReader;
///Field `PS2DET` writer - DM pull-up detection status
pub type PS2DET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DPPU` reader - DP pull-up control
pub type DPPU_R = crate::BitReader;
///Field `DPPU` writer - DP pull-up control
pub type DPPU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Battery charging detector mode enable
    #[inline(always)]
    pub fn bcden(&self) -> BCDEN_R {
        BCDEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Data contact detection mode enable
    #[inline(always)]
    pub fn dcden(&self) -> DCDEN_R {
        DCDEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Primary detection mode enable
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Secondary detection mode enable
    #[inline(always)]
    pub fn sden(&self) -> SDEN_R {
        SDEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Data contact detection status
    #[inline(always)]
    pub fn dcdet(&self) -> DCDET_R {
        DCDET_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Primary detection status
    #[inline(always)]
    pub fn pdet(&self) -> PDET_R {
        PDET_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Secondary detection status
    #[inline(always)]
    pub fn sdet(&self) -> SDET_R {
        SDET_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - DM pull-up detection status
    #[inline(always)]
    pub fn ps2det(&self) -> PS2DET_R {
        PS2DET_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 15 - DP pull-up control
    #[inline(always)]
    pub fn dppu(&self) -> DPPU_R {
        DPPU_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCDR")
            .field("dppu", &self.dppu())
            .field("ps2det", &self.ps2det())
            .field("sdet", &self.sdet())
            .field("pdet", &self.pdet())
            .field("dcdet", &self.dcdet())
            .field("sden", &self.sden())
            .field("pden", &self.pden())
            .field("dcden", &self.dcden())
            .field("bcden", &self.bcden())
            .finish()
    }
}
impl W {
    ///Bit 0 - Battery charging detector mode enable
    #[inline(always)]
    pub fn bcden(&mut self) -> BCDEN_W<BCDRrs> {
        BCDEN_W::new(self, 0)
    }
    ///Bit 1 - Data contact detection mode enable
    #[inline(always)]
    pub fn dcden(&mut self) -> DCDEN_W<BCDRrs> {
        DCDEN_W::new(self, 1)
    }
    ///Bit 2 - Primary detection mode enable
    #[inline(always)]
    pub fn pden(&mut self) -> PDEN_W<BCDRrs> {
        PDEN_W::new(self, 2)
    }
    ///Bit 3 - Secondary detection mode enable
    #[inline(always)]
    pub fn sden(&mut self) -> SDEN_W<BCDRrs> {
        SDEN_W::new(self, 3)
    }
    ///Bit 4 - Data contact detection status
    #[inline(always)]
    pub fn dcdet(&mut self) -> DCDET_W<BCDRrs> {
        DCDET_W::new(self, 4)
    }
    ///Bit 5 - Primary detection status
    #[inline(always)]
    pub fn pdet(&mut self) -> PDET_W<BCDRrs> {
        PDET_W::new(self, 5)
    }
    ///Bit 6 - Secondary detection status
    #[inline(always)]
    pub fn sdet(&mut self) -> SDET_W<BCDRrs> {
        SDET_W::new(self, 6)
    }
    ///Bit 7 - DM pull-up detection status
    #[inline(always)]
    pub fn ps2det(&mut self) -> PS2DET_W<BCDRrs> {
        PS2DET_W::new(self, 7)
    }
    ///Bit 15 - DP pull-up control
    #[inline(always)]
    pub fn dppu(&mut self) -> DPPU_W<BCDRrs> {
        DPPU_W::new(self, 15)
    }
}
/**Battery Charging Detector

You can [`read`](crate::Reg::read) this register and get [`bcdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G431.html#USB:BCDR)*/
pub struct BCDRrs;
impl crate::RegisterSpec for BCDRrs {
    type Ux = u32;
}
///`read()` method returns [`bcdr::R`](R) reader structure
impl crate::Readable for BCDRrs {}
///`write(|w| ..)` method takes [`bcdr::W`](W) writer structure
impl crate::Writable for BCDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BCDR to value 0
impl crate::Resettable for BCDRrs {
    const RESET_VALUE: u32 = 0;
}
