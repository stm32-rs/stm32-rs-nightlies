#[doc = "Register `PMCR` reader"]
pub type R = crate::R<PMCRrs>;
#[doc = "Register `PMCR` writer"]
pub type W = crate::W<PMCRrs>;
#[doc = "Field `BOOSTEN` reader - booster enable Set this bit to reduce the total harmonic distortion of the analog switch when the processor supply is below 2.7 V. The booster can be activated to guaranty AC performance on analog switch when the supply is below 2.7 V. When the booster is activated, the analog switch performances are the same as with the full voltage range."]
pub type BOOSTEN_R = crate::BitReader;
#[doc = "Field `BOOSTEN` writer - booster enable Set this bit to reduce the total harmonic distortion of the analog switch when the processor supply is below 2.7 V. The booster can be activated to guaranty AC performance on analog switch when the supply is below 2.7 V. When the booster is activated, the analog switch performances are the same as with the full voltage range."]
pub type BOOSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOSTVDDSEL` reader - booster VDD selection Note: Booster must not be used when VDDA &lt; 2.7 V, but VDD > 2.7 V (add current consumption). When both VDD &lt; 2.7 V and VDDA &lt; 2.7 V, booster is needed to get full AC performances from I/O analog switches."]
pub type BOOSTVDDSEL_R = crate::BitReader;
#[doc = "Field `BOOSTVDDSEL` writer - booster VDD selection Note: Booster must not be used when VDDA &lt; 2.7 V, but VDD > 2.7 V (add current consumption). When both VDD &lt; 2.7 V and VDDA &lt; 2.7 V, booster is needed to get full AC performances from I/O analog switches."]
pub type BOOSTVDDSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB6_FMP` reader - Fast-mode Plus command on PB(6)"]
pub type PB6_FMP_R = crate::BitReader;
#[doc = "Field `PB6_FMP` writer - Fast-mode Plus command on PB(6)"]
pub type PB6_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB7_FMP` reader - Fast-mode Plus command on PB(7)"]
pub type PB7_FMP_R = crate::BitReader;
#[doc = "Field `PB7_FMP` writer - Fast-mode Plus command on PB(7)"]
pub type PB7_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB8_FMP` reader - Fast-mode Plus command on PB(8)"]
pub type PB8_FMP_R = crate::BitReader;
#[doc = "Field `PB8_FMP` writer - Fast-mode Plus command on PB(8)"]
pub type PB8_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB9_FMPLUS` reader - Fast-mode Plus command on PB(9)"]
pub type PB9_FMPLUS_R = crate::BitReader;
#[doc = "Field `PB9_FMPLUS` writer - Fast-mode Plus command on PB(9)"]
pub type PB9_FMPLUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETH_SEL_PHY` reader - Ethernet PHY interface selection Other: reserved"]
pub type ETH_SEL_PHY_R = crate::FieldReader;
#[doc = "Field `ETH_SEL_PHY` writer - Ethernet PHY interface selection Other: reserved"]
pub type ETH_SEL_PHY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 8 - booster enable Set this bit to reduce the total harmonic distortion of the analog switch when the processor supply is below 2.7 V. The booster can be activated to guaranty AC performance on analog switch when the supply is below 2.7 V. When the booster is activated, the analog switch performances are the same as with the full voltage range."]
    #[inline(always)]
    pub fn boosten(&self) -> BOOSTEN_R {
        BOOSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - booster VDD selection Note: Booster must not be used when VDDA &lt; 2.7 V, but VDD > 2.7 V (add current consumption). When both VDD &lt; 2.7 V and VDDA &lt; 2.7 V, booster is needed to get full AC performances from I/O analog switches."]
    #[inline(always)]
    pub fn boostvddsel(&self) -> BOOSTVDDSEL_R {
        BOOSTVDDSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Fast-mode Plus command on PB(6)"]
    #[inline(always)]
    pub fn pb6_fmp(&self) -> PB6_FMP_R {
        PB6_FMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Fast-mode Plus command on PB(7)"]
    #[inline(always)]
    pub fn pb7_fmp(&self) -> PB7_FMP_R {
        PB7_FMP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fast-mode Plus command on PB(8)"]
    #[inline(always)]
    pub fn pb8_fmp(&self) -> PB8_FMP_R {
        PB8_FMP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Fast-mode Plus command on PB(9)"]
    #[inline(always)]
    pub fn pb9_fmplus(&self) -> PB9_FMPLUS_R {
        PB9_FMPLUS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 21:23 - Ethernet PHY interface selection Other: reserved"]
    #[inline(always)]
    pub fn eth_sel_phy(&self) -> ETH_SEL_PHY_R {
        ETH_SEL_PHY_R::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - booster enable Set this bit to reduce the total harmonic distortion of the analog switch when the processor supply is below 2.7 V. The booster can be activated to guaranty AC performance on analog switch when the supply is below 2.7 V. When the booster is activated, the analog switch performances are the same as with the full voltage range."]
    #[inline(always)]
    #[must_use]
    pub fn boosten(&mut self) -> BOOSTEN_W<PMCRrs> {
        BOOSTEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - booster VDD selection Note: Booster must not be used when VDDA &lt; 2.7 V, but VDD > 2.7 V (add current consumption). When both VDD &lt; 2.7 V and VDDA &lt; 2.7 V, booster is needed to get full AC performances from I/O analog switches."]
    #[inline(always)]
    #[must_use]
    pub fn boostvddsel(&mut self) -> BOOSTVDDSEL_W<PMCRrs> {
        BOOSTVDDSEL_W::new(self, 9)
    }
    #[doc = "Bit 16 - Fast-mode Plus command on PB(6)"]
    #[inline(always)]
    #[must_use]
    pub fn pb6_fmp(&mut self) -> PB6_FMP_W<PMCRrs> {
        PB6_FMP_W::new(self, 16)
    }
    #[doc = "Bit 17 - Fast-mode Plus command on PB(7)"]
    #[inline(always)]
    #[must_use]
    pub fn pb7_fmp(&mut self) -> PB7_FMP_W<PMCRrs> {
        PB7_FMP_W::new(self, 17)
    }
    #[doc = "Bit 18 - Fast-mode Plus command on PB(8)"]
    #[inline(always)]
    #[must_use]
    pub fn pb8_fmp(&mut self) -> PB8_FMP_W<PMCRrs> {
        PB8_FMP_W::new(self, 18)
    }
    #[doc = "Bit 19 - Fast-mode Plus command on PB(9)"]
    #[inline(always)]
    #[must_use]
    pub fn pb9_fmplus(&mut self) -> PB9_FMPLUS_W<PMCRrs> {
        PB9_FMPLUS_W::new(self, 19)
    }
    #[doc = "Bits 21:23 - Ethernet PHY interface selection Other: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn eth_sel_phy(&mut self) -> ETH_SEL_PHY_W<PMCRrs> {
        ETH_SEL_PHY_W::new(self, 21)
    }
}
#[doc = "SBS product mode and configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMCRrs;
impl crate::RegisterSpec for PMCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmcr::R`](R) reader structure"]
impl crate::Readable for PMCRrs {}
#[doc = "`write(|w| ..)` method takes [`pmcr::W`](W) writer structure"]
impl crate::Writable for PMCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMCR to value 0"]
impl crate::Resettable for PMCRrs {
    const RESET_VALUE: u32 = 0;
}
