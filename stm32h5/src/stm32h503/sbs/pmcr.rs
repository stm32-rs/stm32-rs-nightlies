#[doc = "Register `PMCR` reader"]
pub type R = crate::R<PMCRrs>;
#[doc = "Register `PMCR` writer"]
pub type W = crate::W<PMCRrs>;
#[doc = "Field `BOOSTEN` reader - booster enable Set this bit to reduce the total harmonic distortion of the analog switch when the processor supply is below 2.7 V. The booster can be activated to guaranty AC performance on analog switch when the supply is below 2.7 V. When the booster is activated, the analog switch performances are the same as with the full voltage range."]
pub type BOOSTEN_R = crate::BitReader;
#[doc = "Field `BOOSTEN` writer - booster enable Set this bit to reduce the total harmonic distortion of the analog switch when the processor supply is below 2.7 V. The booster can be activated to guaranty AC performance on analog switch when the supply is below 2.7 V. When the booster is activated, the analog switch performances are the same as with the full voltage range."]
pub type BOOSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOSTVDDSEL` reader - booster V&lt;sub>DD&lt;/sub> selection Note: Booster must not be used when V&lt;sub>DDA&lt;/sub> &lt; 2.7 V, but V&lt;sub>DD&lt;/sub> > 2.7 V (add current consumption). Note: When both V&lt;sub>DD&lt;/sub> &lt; 2.7 V and V&lt;sub>DDA&lt;/sub> &lt; 2.7 V, booster is needed to get full AC performances from I/O analog switches."]
pub type BOOSTVDDSEL_R = crate::BitReader;
#[doc = "Field `BOOSTVDDSEL` writer - booster V&lt;sub>DD&lt;/sub> selection Note: Booster must not be used when V&lt;sub>DDA&lt;/sub> &lt; 2.7 V, but V&lt;sub>DD&lt;/sub> > 2.7 V (add current consumption). Note: When both V&lt;sub>DD&lt;/sub> &lt; 2.7 V and V&lt;sub>DDA&lt;/sub> &lt; 2.7 V, booster is needed to get full AC performances from I/O analog switches."]
pub type BOOSTVDDSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Fast-mode Plus command on PB(6)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PB6_FMP {
    #[doc = "0: Fast-mode Plus mode on PB6 disabled"]
    Disabled = 0,
    #[doc = "1: Fast-mode Plus mode on PB6 enabled"]
    Enabled = 1,
}
impl From<PB6_FMP> for bool {
    #[inline(always)]
    fn from(variant: PB6_FMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PB6_FMP` reader - Fast-mode Plus command on PB(6)"]
pub type PB6_FMP_R = crate::BitReader<PB6_FMP>;
impl PB6_FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PB6_FMP {
        match self.bits {
            false => PB6_FMP::Disabled,
            true => PB6_FMP::Enabled,
        }
    }
    #[doc = "Fast-mode Plus mode on PB6 disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PB6_FMP::Disabled
    }
    #[doc = "Fast-mode Plus mode on PB6 enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PB6_FMP::Enabled
    }
}
#[doc = "Field `PB6_FMP` writer - Fast-mode Plus command on PB(6)"]
pub type PB6_FMP_W<'a, REG> = crate::BitWriter<'a, REG, PB6_FMP>;
impl<'a, REG> PB6_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fast-mode Plus mode on PB6 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PB6_FMP::Disabled)
    }
    #[doc = "Fast-mode Plus mode on PB6 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PB6_FMP::Enabled)
    }
}
#[doc = "Fast-mode Plus command on PB(7)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PB7_FMP {
    #[doc = "0: Fast-mode Plus mode on PB7 disabled"]
    Disabled = 0,
    #[doc = "1: Fast-mode Plus mode on PB7 enabled"]
    Enabled = 1,
}
impl From<PB7_FMP> for bool {
    #[inline(always)]
    fn from(variant: PB7_FMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PB7_FMP` reader - Fast-mode Plus command on PB(7)"]
pub type PB7_FMP_R = crate::BitReader<PB7_FMP>;
impl PB7_FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PB7_FMP {
        match self.bits {
            false => PB7_FMP::Disabled,
            true => PB7_FMP::Enabled,
        }
    }
    #[doc = "Fast-mode Plus mode on PB7 disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PB7_FMP::Disabled
    }
    #[doc = "Fast-mode Plus mode on PB7 enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PB7_FMP::Enabled
    }
}
#[doc = "Field `PB7_FMP` writer - Fast-mode Plus command on PB(7)"]
pub type PB7_FMP_W<'a, REG> = crate::BitWriter<'a, REG, PB7_FMP>;
impl<'a, REG> PB7_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fast-mode Plus mode on PB7 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PB7_FMP::Disabled)
    }
    #[doc = "Fast-mode Plus mode on PB7 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PB7_FMP::Enabled)
    }
}
#[doc = "Fast-mode Plus command on PB(8)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PB8_FMP {
    #[doc = "0: Fast-mode Plus mode on PB8 disabled"]
    Disabled = 0,
    #[doc = "1: Fast-mode Plus mode on PB8 enabled"]
    Enabled = 1,
}
impl From<PB8_FMP> for bool {
    #[inline(always)]
    fn from(variant: PB8_FMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PB8_FMP` reader - Fast-mode Plus command on PB(8)"]
pub type PB8_FMP_R = crate::BitReader<PB8_FMP>;
impl PB8_FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PB8_FMP {
        match self.bits {
            false => PB8_FMP::Disabled,
            true => PB8_FMP::Enabled,
        }
    }
    #[doc = "Fast-mode Plus mode on PB8 disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PB8_FMP::Disabled
    }
    #[doc = "Fast-mode Plus mode on PB8 enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PB8_FMP::Enabled
    }
}
#[doc = "Field `PB8_FMP` writer - Fast-mode Plus command on PB(8)"]
pub type PB8_FMP_W<'a, REG> = crate::BitWriter<'a, REG, PB8_FMP>;
impl<'a, REG> PB8_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fast-mode Plus mode on PB8 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PB8_FMP::Disabled)
    }
    #[doc = "Fast-mode Plus mode on PB8 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PB8_FMP::Enabled)
    }
}
impl R {
    #[doc = "Bit 8 - booster enable Set this bit to reduce the total harmonic distortion of the analog switch when the processor supply is below 2.7 V. The booster can be activated to guaranty AC performance on analog switch when the supply is below 2.7 V. When the booster is activated, the analog switch performances are the same as with the full voltage range."]
    #[inline(always)]
    pub fn boosten(&self) -> BOOSTEN_R {
        BOOSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - booster V&lt;sub>DD&lt;/sub> selection Note: Booster must not be used when V&lt;sub>DDA&lt;/sub> &lt; 2.7 V, but V&lt;sub>DD&lt;/sub> > 2.7 V (add current consumption). Note: When both V&lt;sub>DD&lt;/sub> &lt; 2.7 V and V&lt;sub>DDA&lt;/sub> &lt; 2.7 V, booster is needed to get full AC performances from I/O analog switches."]
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
}
impl W {
    #[doc = "Bit 8 - booster enable Set this bit to reduce the total harmonic distortion of the analog switch when the processor supply is below 2.7 V. The booster can be activated to guaranty AC performance on analog switch when the supply is below 2.7 V. When the booster is activated, the analog switch performances are the same as with the full voltage range."]
    #[inline(always)]
    #[must_use]
    pub fn boosten(&mut self) -> BOOSTEN_W<PMCRrs> {
        BOOSTEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - booster V&lt;sub>DD&lt;/sub> selection Note: Booster must not be used when V&lt;sub>DDA&lt;/sub> &lt; 2.7 V, but V&lt;sub>DD&lt;/sub> > 2.7 V (add current consumption). Note: When both V&lt;sub>DD&lt;/sub> &lt; 2.7 V and V&lt;sub>DDA&lt;/sub> &lt; 2.7 V, booster is needed to get full AC performances from I/O analog switches."]
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
