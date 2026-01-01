///Register `IEWU` reader
pub type R = crate::R<IEWUrs>;
///Register `IEWU` writer
pub type W = crate::W<IEWUrs>;
///Field `EIWL0` reader - EWL0 Enable Internal WakeUp line LPUART When this bit is set the internal wakeup line is enabled and a rising edge will trigger a CPU wakeup event. - 0: wakeup disabled. - 1: wakeup enabled.
pub type EIWL0_R = crate::BitReader;
///Field `EIWL0` writer - EWL0 Enable Internal WakeUp line LPUART When this bit is set the internal wakeup line is enabled and a rising edge will trigger a CPU wakeup event. - 0: wakeup disabled. - 1: wakeup enabled.
pub type EIWL0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EIWL1` reader - EIWL1 Enable Internal WakeUp line RTC When this bit is set the internal wakeup line is enabled and a rising edge will trigger a CPU wakeup event. - 0: wakeup disabled. - 1: wakeup enabled.
pub type EIWL1_R = crate::BitReader;
///Field `EIWL1` writer - EIWL1 Enable Internal WakeUp line RTC When this bit is set the internal wakeup line is enabled and a rising edge will trigger a CPU wakeup event. - 0: wakeup disabled. - 1: wakeup enabled.
pub type EIWL1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EIWL2` reader - EIWL2 Enable Internal WakeUp line LCD When this bit is set the internal wakeup line is enabled and a rising edge will trigger a CPU wakeup event. - 0: wakeup disabled. - 1: wakeup enabled.
pub type EIWL2_R = crate::BitReader;
///Field `EIWL2` writer - EIWL2 Enable Internal WakeUp line LCD When this bit is set the internal wakeup line is enabled and a rising edge will trigger a CPU wakeup event. - 0: wakeup disabled. - 1: wakeup enabled.
pub type EIWL2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EIWL3` reader - EIWL3 Enable Internal Wakeup line COMP When this bit is set the COMP wakeup is enabled and an edge will trigger a COMP wakeup event - 0: wakeup disabled. - 1: wakeup enabled.
pub type EIWL3_R = crate::BitReader;
///Field `EIWL3` writer - EIWL3 Enable Internal Wakeup line COMP When this bit is set the COMP wakeup is enabled and an edge will trigger a COMP wakeup event - 0: wakeup disabled. - 1: wakeup enabled.
pub type EIWL3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EIWL4` reader - EIWL4 Enable Internal Wakeup line LCSC When this bit is set the LCSC wakeup is enabled and an edge will trigger a LCSC wakeup event - 0: wakeup disabled. - 1: wakeup enabled.
pub type EIWL4_R = crate::BitReader;
///Field `EIWL4` writer - EIWL4 Enable Internal Wakeup line LCSC When this bit is set the LCSC wakeup is enabled and an edge will trigger a LCSC wakeup event - 0: wakeup disabled. - 1: wakeup enabled.
pub type EIWL4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWMRSUBG` reader - EWMRSUB Wakeup MRSUBG Enable When this bit is set the MRSUBG wakeup is enabled and a rising edge will trigger a MRSUBG wakeup event - 0: MRSUBG wakeup disabled. - 1: MRSUBG wakeup enabled.
pub type EWMRSUBG_R = crate::BitReader;
///Field `EWMRSUBG` writer - EWMRSUB Wakeup MRSUBG Enable When this bit is set the MRSUBG wakeup is enabled and a rising edge will trigger a MRSUBG wakeup event - 0: MRSUBG wakeup disabled. - 1: MRSUBG wakeup enabled.
pub type EWMRSUBG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWMRSUBGHCPU` reader - EWMRSUBGHCPU Wakeup MRSUBG Host CPU Enable When this bit is set the MRSUBG HOST CPU wakeup is enabled and a rising edge will trigger a MRSUBG Host CPU wakeup event - 0: MRSUBG Host CPU wakeup disabled. - 1: MRSUBG Host CPU wakeup enabled.
pub type EWMRSUBGHCPU_R = crate::BitReader;
///Field `EWMRSUBGHCPU` writer - EWMRSUBGHCPU Wakeup MRSUBG Host CPU Enable When this bit is set the MRSUBG HOST CPU wakeup is enabled and a rising edge will trigger a MRSUBG Host CPU wakeup event - 0: MRSUBG Host CPU wakeup disabled. - 1: MRSUBG Host CPU wakeup enabled.
pub type EWMRSUBGHCPU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWLPAWUR` reader - EWLPAWUR: Wakeup Bubble Enable When this bit is set the Bubble wakeup is enabled and a rising edge will trigger a LPAWUR wakeup event - 0: LPAWUR wakeup disabled. - 1: LPAWUR wakeup enabled.
pub type EWLPAWUR_R = crate::BitReader;
///Field `EWLPAWUR` writer - EWLPAWUR: Wakeup Bubble Enable When this bit is set the Bubble wakeup is enabled and a rising edge will trigger a LPAWUR wakeup event - 0: LPAWUR wakeup disabled. - 1: LPAWUR wakeup enabled.
pub type EWLPAWUR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - EWL0 Enable Internal WakeUp line LPUART When this bit is set the internal wakeup line is enabled and a rising edge will trigger a CPU wakeup event. - 0: wakeup disabled. - 1: wakeup enabled.
    #[inline(always)]
    pub fn eiwl0(&self) -> EIWL0_R {
        EIWL0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EIWL1 Enable Internal WakeUp line RTC When this bit is set the internal wakeup line is enabled and a rising edge will trigger a CPU wakeup event. - 0: wakeup disabled. - 1: wakeup enabled.
    #[inline(always)]
    pub fn eiwl1(&self) -> EIWL1_R {
        EIWL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - EIWL2 Enable Internal WakeUp line LCD When this bit is set the internal wakeup line is enabled and a rising edge will trigger a CPU wakeup event. - 0: wakeup disabled. - 1: wakeup enabled.
    #[inline(always)]
    pub fn eiwl2(&self) -> EIWL2_R {
        EIWL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - EIWL3 Enable Internal Wakeup line COMP When this bit is set the COMP wakeup is enabled and an edge will trigger a COMP wakeup event - 0: wakeup disabled. - 1: wakeup enabled.
    #[inline(always)]
    pub fn eiwl3(&self) -> EIWL3_R {
        EIWL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - EIWL4 Enable Internal Wakeup line LCSC When this bit is set the LCSC wakeup is enabled and an edge will trigger a LCSC wakeup event - 0: wakeup disabled. - 1: wakeup enabled.
    #[inline(always)]
    pub fn eiwl4(&self) -> EIWL4_R {
        EIWL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - EWMRSUB Wakeup MRSUBG Enable When this bit is set the MRSUBG wakeup is enabled and a rising edge will trigger a MRSUBG wakeup event - 0: MRSUBG wakeup disabled. - 1: MRSUBG wakeup enabled.
    #[inline(always)]
    pub fn ewmrsubg(&self) -> EWMRSUBG_R {
        EWMRSUBG_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - EWMRSUBGHCPU Wakeup MRSUBG Host CPU Enable When this bit is set the MRSUBG HOST CPU wakeup is enabled and a rising edge will trigger a MRSUBG Host CPU wakeup event - 0: MRSUBG Host CPU wakeup disabled. - 1: MRSUBG Host CPU wakeup enabled.
    #[inline(always)]
    pub fn ewmrsubghcpu(&self) -> EWMRSUBGHCPU_R {
        EWMRSUBGHCPU_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - EWLPAWUR: Wakeup Bubble Enable When this bit is set the Bubble wakeup is enabled and a rising edge will trigger a LPAWUR wakeup event - 0: LPAWUR wakeup disabled. - 1: LPAWUR wakeup enabled.
    #[inline(always)]
    pub fn ewlpawur(&self) -> EWLPAWUR_R {
        EWLPAWUR_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IEWU")
            .field("eiwl0", &self.eiwl0())
            .field("eiwl1", &self.eiwl1())
            .field("eiwl2", &self.eiwl2())
            .field("eiwl3", &self.eiwl3())
            .field("eiwl4", &self.eiwl4())
            .field("ewmrsubg", &self.ewmrsubg())
            .field("ewmrsubghcpu", &self.ewmrsubghcpu())
            .field("ewlpawur", &self.ewlpawur())
            .finish()
    }
}
impl W {
    ///Bit 0 - EWL0 Enable Internal WakeUp line LPUART When this bit is set the internal wakeup line is enabled and a rising edge will trigger a CPU wakeup event. - 0: wakeup disabled. - 1: wakeup enabled.
    #[inline(always)]
    pub fn eiwl0(&mut self) -> EIWL0_W<'_, IEWUrs> {
        EIWL0_W::new(self, 0)
    }
    ///Bit 1 - EIWL1 Enable Internal WakeUp line RTC When this bit is set the internal wakeup line is enabled and a rising edge will trigger a CPU wakeup event. - 0: wakeup disabled. - 1: wakeup enabled.
    #[inline(always)]
    pub fn eiwl1(&mut self) -> EIWL1_W<'_, IEWUrs> {
        EIWL1_W::new(self, 1)
    }
    ///Bit 2 - EIWL2 Enable Internal WakeUp line LCD When this bit is set the internal wakeup line is enabled and a rising edge will trigger a CPU wakeup event. - 0: wakeup disabled. - 1: wakeup enabled.
    #[inline(always)]
    pub fn eiwl2(&mut self) -> EIWL2_W<'_, IEWUrs> {
        EIWL2_W::new(self, 2)
    }
    ///Bit 3 - EIWL3 Enable Internal Wakeup line COMP When this bit is set the COMP wakeup is enabled and an edge will trigger a COMP wakeup event - 0: wakeup disabled. - 1: wakeup enabled.
    #[inline(always)]
    pub fn eiwl3(&mut self) -> EIWL3_W<'_, IEWUrs> {
        EIWL3_W::new(self, 3)
    }
    ///Bit 4 - EIWL4 Enable Internal Wakeup line LCSC When this bit is set the LCSC wakeup is enabled and an edge will trigger a LCSC wakeup event - 0: wakeup disabled. - 1: wakeup enabled.
    #[inline(always)]
    pub fn eiwl4(&mut self) -> EIWL4_W<'_, IEWUrs> {
        EIWL4_W::new(self, 4)
    }
    ///Bit 8 - EWMRSUB Wakeup MRSUBG Enable When this bit is set the MRSUBG wakeup is enabled and a rising edge will trigger a MRSUBG wakeup event - 0: MRSUBG wakeup disabled. - 1: MRSUBG wakeup enabled.
    #[inline(always)]
    pub fn ewmrsubg(&mut self) -> EWMRSUBG_W<'_, IEWUrs> {
        EWMRSUBG_W::new(self, 8)
    }
    ///Bit 9 - EWMRSUBGHCPU Wakeup MRSUBG Host CPU Enable When this bit is set the MRSUBG HOST CPU wakeup is enabled and a rising edge will trigger a MRSUBG Host CPU wakeup event - 0: MRSUBG Host CPU wakeup disabled. - 1: MRSUBG Host CPU wakeup enabled.
    #[inline(always)]
    pub fn ewmrsubghcpu(&mut self) -> EWMRSUBGHCPU_W<'_, IEWUrs> {
        EWMRSUBGHCPU_W::new(self, 9)
    }
    ///Bit 10 - EWLPAWUR: Wakeup Bubble Enable When this bit is set the Bubble wakeup is enabled and a rising edge will trigger a LPAWUR wakeup event - 0: LPAWUR wakeup disabled. - 1: LPAWUR wakeup enabled.
    #[inline(always)]
    pub fn ewlpawur(&mut self) -> EWLPAWUR_W<'_, IEWUrs> {
        EWLPAWUR_W::new(self, 10)
    }
}
/**IEWU register

You can [`read`](crate::Reg::read) this register and get [`iewu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iewu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:IEWU)*/
pub struct IEWUrs;
impl crate::RegisterSpec for IEWUrs {
    type Ux = u32;
}
///`read()` method returns [`iewu::R`](R) reader structure
impl crate::Readable for IEWUrs {}
///`write(|w| ..)` method takes [`iewu::W`](W) writer structure
impl crate::Writable for IEWUrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IEWU to value 0
impl crate::Resettable for IEWUrs {}
