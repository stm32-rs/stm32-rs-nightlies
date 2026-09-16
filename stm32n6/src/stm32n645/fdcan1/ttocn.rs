///Register `TTOCN` reader
pub type R = crate::R<TTOCNrs>;
///Register `TTOCN` writer
pub type W = crate::W<TTOCNrs>;
///Field `SGT` reader - Set global time.
pub type SGT_R = crate::BitReader;
///Field `SGT` writer - Set global time.
pub type SGT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECS` reader - External clock synchronization.
pub type ECS_R = crate::BitReader;
///Field `ECS` writer - External clock synchronization.
pub type ECS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWP` reader - Stop watch polarity.
pub type SWP_R = crate::BitReader;
///Field `SWP` writer - Stop watch polarity.
pub type SWP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWS` reader - Stop watch source.
pub type SWS_R = crate::FieldReader;
///Field `SWS` writer - Stop watch source.
pub type SWS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RTIE` reader - Register time mark interrupt pulse enable.
pub type RTIE_R = crate::BitReader;
///Field `RTIE` writer - Register time mark interrupt pulse enable.
pub type RTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TMC` reader - Register time mark compare.
pub type TMC_R = crate::FieldReader;
///Field `TMC` writer - Register time mark compare.
pub type TMC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TTIE` reader - Trigger time mark interrupt pulse enable
pub type TTIE_R = crate::BitReader;
///Field `TTIE` writer - Trigger time mark interrupt pulse enable
pub type TTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GCS` reader - Gap control select
pub type GCS_R = crate::BitReader;
///Field `GCS` writer - Gap control select
pub type GCS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FGP` reader - Finish gap.
pub type FGP_R = crate::BitReader;
///Field `FGP` writer - Finish gap.
pub type FGP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TMG` reader - Time mark gap.
pub type TMG_R = crate::BitReader;
///Field `TMG` writer - Time mark gap.
pub type TMG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NIG` reader - Next is gap.
pub type NIG_R = crate::BitReader;
///Field `NIG` writer - Next is gap.
pub type NIG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ESCN` reader - External synchronization control
pub type ESCN_R = crate::BitReader;
///Field `ESCN` writer - External synchronization control
pub type ESCN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCKC` reader - TT operation control register locked.
pub type LCKC_R = crate::BitReader;
impl R {
    ///Bit 0 - Set global time.
    #[inline(always)]
    pub fn sgt(&self) -> SGT_R {
        SGT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - External clock synchronization.
    #[inline(always)]
    pub fn ecs(&self) -> ECS_R {
        ECS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Stop watch polarity.
    #[inline(always)]
    pub fn swp(&self) -> SWP_R {
        SWP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - Stop watch source.
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - Register time mark interrupt pulse enable.
    #[inline(always)]
    pub fn rtie(&self) -> RTIE_R {
        RTIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - Register time mark compare.
    #[inline(always)]
    pub fn tmc(&self) -> TMC_R {
        TMC_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - Trigger time mark interrupt pulse enable
    #[inline(always)]
    pub fn ttie(&self) -> TTIE_R {
        TTIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Gap control select
    #[inline(always)]
    pub fn gcs(&self) -> GCS_R {
        GCS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Finish gap.
    #[inline(always)]
    pub fn fgp(&self) -> FGP_R {
        FGP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Time mark gap.
    #[inline(always)]
    pub fn tmg(&self) -> TMG_R {
        TMG_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Next is gap.
    #[inline(always)]
    pub fn nig(&self) -> NIG_R {
        NIG_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - External synchronization control
    #[inline(always)]
    pub fn escn(&self) -> ESCN_R {
        ESCN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - TT operation control register locked.
    #[inline(always)]
    pub fn lckc(&self) -> LCKC_R {
        LCKC_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TTOCN")
            .field("sgt", &self.sgt())
            .field("ecs", &self.ecs())
            .field("swp", &self.swp())
            .field("sws", &self.sws())
            .field("rtie", &self.rtie())
            .field("tmc", &self.tmc())
            .field("ttie", &self.ttie())
            .field("gcs", &self.gcs())
            .field("fgp", &self.fgp())
            .field("tmg", &self.tmg())
            .field("nig", &self.nig())
            .field("escn", &self.escn())
            .field("lckc", &self.lckc())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set global time.
    #[inline(always)]
    pub fn sgt(&mut self) -> SGT_W<'_, TTOCNrs> {
        SGT_W::new(self, 0)
    }
    ///Bit 1 - External clock synchronization.
    #[inline(always)]
    pub fn ecs(&mut self) -> ECS_W<'_, TTOCNrs> {
        ECS_W::new(self, 1)
    }
    ///Bit 2 - Stop watch polarity.
    #[inline(always)]
    pub fn swp(&mut self) -> SWP_W<'_, TTOCNrs> {
        SWP_W::new(self, 2)
    }
    ///Bits 3:4 - Stop watch source.
    #[inline(always)]
    pub fn sws(&mut self) -> SWS_W<'_, TTOCNrs> {
        SWS_W::new(self, 3)
    }
    ///Bit 5 - Register time mark interrupt pulse enable.
    #[inline(always)]
    pub fn rtie(&mut self) -> RTIE_W<'_, TTOCNrs> {
        RTIE_W::new(self, 5)
    }
    ///Bits 6:7 - Register time mark compare.
    #[inline(always)]
    pub fn tmc(&mut self) -> TMC_W<'_, TTOCNrs> {
        TMC_W::new(self, 6)
    }
    ///Bit 8 - Trigger time mark interrupt pulse enable
    #[inline(always)]
    pub fn ttie(&mut self) -> TTIE_W<'_, TTOCNrs> {
        TTIE_W::new(self, 8)
    }
    ///Bit 9 - Gap control select
    #[inline(always)]
    pub fn gcs(&mut self) -> GCS_W<'_, TTOCNrs> {
        GCS_W::new(self, 9)
    }
    ///Bit 10 - Finish gap.
    #[inline(always)]
    pub fn fgp(&mut self) -> FGP_W<'_, TTOCNrs> {
        FGP_W::new(self, 10)
    }
    ///Bit 11 - Time mark gap.
    #[inline(always)]
    pub fn tmg(&mut self) -> TMG_W<'_, TTOCNrs> {
        TMG_W::new(self, 11)
    }
    ///Bit 12 - Next is gap.
    #[inline(always)]
    pub fn nig(&mut self) -> NIG_W<'_, TTOCNrs> {
        NIG_W::new(self, 12)
    }
    ///Bit 13 - External synchronization control
    #[inline(always)]
    pub fn escn(&mut self) -> ESCN_W<'_, TTOCNrs> {
        ESCN_W::new(self, 13)
    }
}
/**FDCAN TT operation control register

You can [`read`](crate::Reg::read) this register and get [`ttocn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttocn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#FDCAN1:TTOCN)*/
pub struct TTOCNrs;
impl crate::RegisterSpec for TTOCNrs {
    type Ux = u32;
}
///`read()` method returns [`ttocn::R`](R) reader structure
impl crate::Readable for TTOCNrs {}
///`write(|w| ..)` method takes [`ttocn::W`](W) writer structure
impl crate::Writable for TTOCNrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TTOCN to value 0
impl crate::Resettable for TTOCNrs {}
