///Register `PMCR` reader
pub type R = crate::R<PMCRrs>;
///Register `PMCR` writer
pub type W = crate::W<PMCRrs>;
///Field `BOOSTEN` reader - booster enable Set this bit to reduce the total harmonic distortion of the analog switch when the processor supply is below 2.7 V. The booster can be activated to guaranty AC performance on analog switch when the supply is below 2.7 V. When the booster is activated, the analog switch performances are the same as with the full voltage range.
pub type BOOSTEN_R = crate::BitReader;
///Field `BOOSTEN` writer - booster enable Set this bit to reduce the total harmonic distortion of the analog switch when the processor supply is below 2.7 V. The booster can be activated to guaranty AC performance on analog switch when the supply is below 2.7 V. When the booster is activated, the analog switch performances are the same as with the full voltage range.
pub type BOOSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOOSTVDDSEL` reader - booster V DD selection Note: Booster must not be used when V DDA 2.7 V, but V DD 2.7 V (add current consumption). Note: When both V DD 2.7 V and V DDA 2.7 V, booster is needed to get full AC performances from I/O analog switches.
pub type BOOSTVDDSEL_R = crate::BitReader;
///Field `BOOSTVDDSEL` writer - booster V DD selection Note: Booster must not be used when V DDA 2.7 V, but V DD 2.7 V (add current consumption). Note: When both V DD 2.7 V and V DDA 2.7 V, booster is needed to get full AC performances from I/O analog switches.
pub type BOOSTVDDSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Fast-mode Plus command on PB(6)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PB6_FMP {
    ///0: Fast-mode Plus mode on PB6 disabled
    Disabled = 0,
    ///1: Fast-mode Plus mode on PB6 enabled
    Enabled = 1,
}
impl From<PB6_FMP> for bool {
    #[inline(always)]
    fn from(variant: PB6_FMP) -> Self {
        variant as u8 != 0
    }
}
///Field `PB6_FMP` reader - Fast-mode Plus command on PB(6)
pub type PB6_FMP_R = crate::BitReader<PB6_FMP>;
impl PB6_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PB6_FMP {
        match self.bits {
            false => PB6_FMP::Disabled,
            true => PB6_FMP::Enabled,
        }
    }
    ///Fast-mode Plus mode on PB6 disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PB6_FMP::Disabled
    }
    ///Fast-mode Plus mode on PB6 enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PB6_FMP::Enabled
    }
}
///Field `PB6_FMP` writer - Fast-mode Plus command on PB(6)
pub type PB6_FMP_W<'a, REG> = crate::BitWriter<'a, REG, PB6_FMP>;
impl<'a, REG> PB6_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Fast-mode Plus mode on PB6 disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PB6_FMP::Disabled)
    }
    ///Fast-mode Plus mode on PB6 enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PB6_FMP::Enabled)
    }
}
/**Fast-mode Plus command on PB(7)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PB7_FMP {
    ///0: Fast-mode Plus mode on PB7 disabled
    Disabled = 0,
    ///1: Fast-mode Plus mode on PB7 enabled
    Enabled = 1,
}
impl From<PB7_FMP> for bool {
    #[inline(always)]
    fn from(variant: PB7_FMP) -> Self {
        variant as u8 != 0
    }
}
///Field `PB7_FMP` reader - Fast-mode Plus command on PB(7)
pub type PB7_FMP_R = crate::BitReader<PB7_FMP>;
impl PB7_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PB7_FMP {
        match self.bits {
            false => PB7_FMP::Disabled,
            true => PB7_FMP::Enabled,
        }
    }
    ///Fast-mode Plus mode on PB7 disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PB7_FMP::Disabled
    }
    ///Fast-mode Plus mode on PB7 enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PB7_FMP::Enabled
    }
}
///Field `PB7_FMP` writer - Fast-mode Plus command on PB(7)
pub type PB7_FMP_W<'a, REG> = crate::BitWriter<'a, REG, PB7_FMP>;
impl<'a, REG> PB7_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Fast-mode Plus mode on PB7 disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PB7_FMP::Disabled)
    }
    ///Fast-mode Plus mode on PB7 enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PB7_FMP::Enabled)
    }
}
/**Fast-mode Plus command on PB(8)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PB8_FMP {
    ///0: Fast-mode Plus mode on PB8 disabled
    Disabled = 0,
    ///1: Fast-mode Plus mode on PB8 enabled
    Enabled = 1,
}
impl From<PB8_FMP> for bool {
    #[inline(always)]
    fn from(variant: PB8_FMP) -> Self {
        variant as u8 != 0
    }
}
///Field `PB8_FMP` reader - Fast-mode Plus command on PB(8)
pub type PB8_FMP_R = crate::BitReader<PB8_FMP>;
impl PB8_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PB8_FMP {
        match self.bits {
            false => PB8_FMP::Disabled,
            true => PB8_FMP::Enabled,
        }
    }
    ///Fast-mode Plus mode on PB8 disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PB8_FMP::Disabled
    }
    ///Fast-mode Plus mode on PB8 enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PB8_FMP::Enabled
    }
}
///Field `PB8_FMP` writer - Fast-mode Plus command on PB(8)
pub type PB8_FMP_W<'a, REG> = crate::BitWriter<'a, REG, PB8_FMP>;
impl<'a, REG> PB8_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Fast-mode Plus mode on PB8 disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PB8_FMP::Disabled)
    }
    ///Fast-mode Plus mode on PB8 enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PB8_FMP::Enabled)
    }
}
impl R {
    ///Bit 8 - booster enable Set this bit to reduce the total harmonic distortion of the analog switch when the processor supply is below 2.7 V. The booster can be activated to guaranty AC performance on analog switch when the supply is below 2.7 V. When the booster is activated, the analog switch performances are the same as with the full voltage range.
    #[inline(always)]
    pub fn boosten(&self) -> BOOSTEN_R {
        BOOSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - booster V DD selection Note: Booster must not be used when V DDA 2.7 V, but V DD 2.7 V (add current consumption). Note: When both V DD 2.7 V and V DDA 2.7 V, booster is needed to get full AC performances from I/O analog switches.
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMCR")
            .field("boosten", &self.boosten())
            .field("boostvddsel", &self.boostvddsel())
            .field("pb6_fmp", &self.pb6_fmp())
            .field("pb7_fmp", &self.pb7_fmp())
            .field("pb8_fmp", &self.pb8_fmp())
            .finish()
    }
}
impl W {
    ///Bit 8 - booster enable Set this bit to reduce the total harmonic distortion of the analog switch when the processor supply is below 2.7 V. The booster can be activated to guaranty AC performance on analog switch when the supply is below 2.7 V. When the booster is activated, the analog switch performances are the same as with the full voltage range.
    #[inline(always)]
    pub fn boosten(&mut self) -> BOOSTEN_W<'_, PMCRrs> {
        BOOSTEN_W::new(self, 8)
    }
    ///Bit 9 - booster V DD selection Note: Booster must not be used when V DDA 2.7 V, but V DD 2.7 V (add current consumption). Note: When both V DD 2.7 V and V DDA 2.7 V, booster is needed to get full AC performances from I/O analog switches.
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
}
/**SBS product mode and configuration register

You can [`read`](crate::Reg::read) this register and get [`pmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#SBS:PMCR)*/
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
