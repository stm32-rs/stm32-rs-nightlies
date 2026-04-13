///Register `PMCR` reader
pub type R = crate::R<PMCRrs>;
///Register `PMCR` writer
pub type W = crate::W<PMCRrs>;
///Field `BOOSTEN` reader - booster enable Set this bit to reduce the total harmonic distortion of the analog switch when the processor supply is below 2.7 V. The booster can be activated to guaranty AC performance on analog switch when the supply is below 2.7 V. When the booster is activated, the analog switch performances are the same as with the full voltage range.
pub type BOOSTEN_R = crate::BitReader;
///Field `BOOSTEN` writer - booster enable Set this bit to reduce the total harmonic distortion of the analog switch when the processor supply is below 2.7 V. The booster can be activated to guaranty AC performance on analog switch when the supply is below 2.7 V. When the booster is activated, the analog switch performances are the same as with the full voltage range.
pub type BOOSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOOSTVDDSEL` reader - booster VDD selection Note: Booster must not be used when VDDA 2.7 V, but VDD 2.7 V (add current consumption). When both VDD 2.7 V and VDDA 2.7 V, booster is needed to get full AC performances from I/O analog switches.
pub type BOOSTVDDSEL_R = crate::BitReader;
///Field `BOOSTVDDSEL` writer - booster VDD selection Note: Booster must not be used when VDDA 2.7 V, but VDD 2.7 V (add current consumption). When both VDD 2.7 V and VDDA 2.7 V, booster is needed to get full AC performances from I/O analog switches.
pub type BOOSTVDDSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PB6_FMP` reader - Fast-mode Plus command on PB(6)
pub type PB6_FMP_R = crate::BitReader;
///Field `PB6_FMP` writer - Fast-mode Plus command on PB(6)
pub type PB6_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PB7_FMP` reader - Fast-mode Plus command on PB(7)
pub type PB7_FMP_R = crate::BitReader;
///Field `PB7_FMP` writer - Fast-mode Plus command on PB(7)
pub type PB7_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PB8_FMP` reader - Fast-mode Plus command on PB(8)
pub type PB8_FMP_R = crate::BitReader;
///Field `PB8_FMP` writer - Fast-mode Plus command on PB(8)
pub type PB8_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PB9_FMPLUS` reader - Fast-mode Plus command on PB(9)
pub type PB9_FMPLUS_R = crate::BitReader;
///Field `PB9_FMPLUS` writer - Fast-mode Plus command on PB(9)
pub type PB9_FMPLUS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 8 - booster enable Set this bit to reduce the total harmonic distortion of the analog switch when the processor supply is below 2.7 V. The booster can be activated to guaranty AC performance on analog switch when the supply is below 2.7 V. When the booster is activated, the analog switch performances are the same as with the full voltage range.
    #[inline(always)]
    pub fn boosten(&self) -> BOOSTEN_R {
        BOOSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - booster VDD selection Note: Booster must not be used when VDDA 2.7 V, but VDD 2.7 V (add current consumption). When both VDD 2.7 V and VDDA 2.7 V, booster is needed to get full AC performances from I/O analog switches.
    #[inline(always)]
    pub fn boostvddsel(&self) -> BOOSTVDDSEL_R {
        BOOSTVDDSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - Fast-mode Plus command on PB(6)
    #[inline(always)]
    pub fn pb6_fmp(&self) -> PB6_FMP_R {
        PB6_FMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Fast-mode Plus command on PB(7)
    #[inline(always)]
    pub fn pb7_fmp(&self) -> PB7_FMP_R {
        PB7_FMP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Fast-mode Plus command on PB(8)
    #[inline(always)]
    pub fn pb8_fmp(&self) -> PB8_FMP_R {
        PB8_FMP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Fast-mode Plus command on PB(9)
    #[inline(always)]
    pub fn pb9_fmplus(&self) -> PB9_FMPLUS_R {
        PB9_FMPLUS_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMCR")
            .field("boosten", &self.boosten())
            .field("boostvddsel", &self.boostvddsel())
            .field("pb6_fmp", &self.pb6_fmp())
            .field("pb7_fmp", &self.pb7_fmp())
            .field("pb8_fmp", &self.pb8_fmp())
            .field("pb9_fmplus", &self.pb9_fmplus())
            .finish()
    }
}
impl W {
    ///Bit 8 - booster enable Set this bit to reduce the total harmonic distortion of the analog switch when the processor supply is below 2.7 V. The booster can be activated to guaranty AC performance on analog switch when the supply is below 2.7 V. When the booster is activated, the analog switch performances are the same as with the full voltage range.
    #[inline(always)]
    pub fn boosten(&mut self) -> BOOSTEN_W<'_, PMCRrs> {
        BOOSTEN_W::new(self, 8)
    }
    ///Bit 9 - booster VDD selection Note: Booster must not be used when VDDA 2.7 V, but VDD 2.7 V (add current consumption). When both VDD 2.7 V and VDDA 2.7 V, booster is needed to get full AC performances from I/O analog switches.
    #[inline(always)]
    pub fn boostvddsel(&mut self) -> BOOSTVDDSEL_W<'_, PMCRrs> {
        BOOSTVDDSEL_W::new(self, 9)
    }
    ///Bit 16 - Fast-mode Plus command on PB(6)
    #[inline(always)]
    pub fn pb6_fmp(&mut self) -> PB6_FMP_W<'_, PMCRrs> {
        PB6_FMP_W::new(self, 16)
    }
    ///Bit 17 - Fast-mode Plus command on PB(7)
    #[inline(always)]
    pub fn pb7_fmp(&mut self) -> PB7_FMP_W<'_, PMCRrs> {
        PB7_FMP_W::new(self, 17)
    }
    ///Bit 18 - Fast-mode Plus command on PB(8)
    #[inline(always)]
    pub fn pb8_fmp(&mut self) -> PB8_FMP_W<'_, PMCRrs> {
        PB8_FMP_W::new(self, 18)
    }
    ///Bit 19 - Fast-mode Plus command on PB(9)
    #[inline(always)]
    pub fn pb9_fmplus(&mut self) -> PB9_FMPLUS_W<'_, PMCRrs> {
        PB9_FMPLUS_W::new(self, 19)
    }
}
/**SBS product mode and configuration register

You can [`read`](crate::Reg::read) this register and get [`pmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#SBS:PMCR)*/
pub struct PMCRrs;
impl crate::RegisterSpec for PMCRrs {
    type Ux = u32;
}
///`read()` method returns [`pmcr::R`](R) reader structure
impl crate::Readable for PMCRrs {}
///`write(|w| ..)` method takes [`pmcr::W`](W) writer structure
impl crate::Writable for PMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PMCR to value 0
impl crate::Resettable for PMCRrs {}
