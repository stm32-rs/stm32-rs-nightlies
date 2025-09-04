///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**Random number generator enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNGEN {
    ///0: Random number generator is disabled
    Disabled = 0,
    ///1: Random number generator is enabled
    Enabled = 1,
}
impl From<RNGEN> for bool {
    #[inline(always)]
    fn from(variant: RNGEN) -> Self {
        variant as u8 != 0
    }
}
///Field `RNGEN` reader - Random number generator enable
pub type RNGEN_R = crate::BitReader<RNGEN>;
impl RNGEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RNGEN {
        match self.bits {
            false => RNGEN::Disabled,
            true => RNGEN::Enabled,
        }
    }
    ///Random number generator is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RNGEN::Disabled
    }
    ///Random number generator is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RNGEN::Enabled
    }
}
///Field `RNGEN` writer - Random number generator enable
pub type RNGEN_W<'a, REG> = crate::BitWriter<'a, REG, RNGEN>;
impl<'a, REG> RNGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Random number generator is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RNGEN::Disabled)
    }
    ///Random number generator is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RNGEN::Enabled)
    }
}
/**Interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IE {
    ///0: RNG interrupt is disabled
    Disabled = 0,
    ///1: RNG interrupt is enabled
    Enabled = 1,
}
impl From<IE> for bool {
    #[inline(always)]
    fn from(variant: IE) -> Self {
        variant as u8 != 0
    }
}
///Field `IE` reader - Interrupt enable
pub type IE_R = crate::BitReader<IE>;
impl IE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IE {
        match self.bits {
            false => IE::Disabled,
            true => IE::Enabled,
        }
    }
    ///RNG interrupt is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IE::Disabled
    }
    ///RNG interrupt is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IE::Enabled
    }
}
///Field `IE` writer - Interrupt enable
pub type IE_W<'a, REG> = crate::BitWriter<'a, REG, IE>;
impl<'a, REG> IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RNG interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IE::Disabled)
    }
    ///RNG interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IE::Enabled)
    }
}
/**Clock error detection Note: The clock error detection can be used only when ck_rc48 or ck_pll1_q (ck_pll1_q = 48MHz) source is selected otherwise, CED bit must be equal to 1. The clock error detection cannot be enabled nor disabled on the fly when RNG peripheral is enabled, to enable or disable CED the RNG must be disabled.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CED {
    ///0: Clock error detection is enabled
    Enabled = 0,
    ///1: Clock error detection is disabled
    Disabled = 1,
}
impl From<CED> for bool {
    #[inline(always)]
    fn from(variant: CED) -> Self {
        variant as u8 != 0
    }
}
///Field `CED` reader - Clock error detection Note: The clock error detection can be used only when ck_rc48 or ck_pll1_q (ck_pll1_q = 48MHz) source is selected otherwise, CED bit must be equal to 1. The clock error detection cannot be enabled nor disabled on the fly when RNG peripheral is enabled, to enable or disable CED the RNG must be disabled.
pub type CED_R = crate::BitReader<CED>;
impl CED_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CED {
        match self.bits {
            false => CED::Enabled,
            true => CED::Disabled,
        }
    }
    ///Clock error detection is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CED::Enabled
    }
    ///Clock error detection is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CED::Disabled
    }
}
///Field `CED` writer - Clock error detection Note: The clock error detection can be used only when ck_rc48 or ck_pll1_q (ck_pll1_q = 48MHz) source is selected otherwise, CED bit must be equal to 1. The clock error detection cannot be enabled nor disabled on the fly when RNG peripheral is enabled, to enable or disable CED the RNG must be disabled.
pub type CED_W<'a, REG> = crate::BitWriter<'a, REG, CED>;
impl<'a, REG> CED_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock error detection is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CED::Enabled)
    }
    ///Clock error detection is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CED::Disabled)
    }
}
impl R {
    ///Bit 2 - Random number generator enable
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Interrupt enable
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - Clock error detection Note: The clock error detection can be used only when ck_rc48 or ck_pll1_q (ck_pll1_q = 48MHz) source is selected otherwise, CED bit must be equal to 1. The clock error detection cannot be enabled nor disabled on the fly when RNG peripheral is enabled, to enable or disable CED the RNG must be disabled.
    #[inline(always)]
    pub fn ced(&self) -> CED_R {
        CED_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("rngen", &self.rngen())
            .field("ie", &self.ie())
            .field("ced", &self.ced())
            .finish()
    }
}
impl W {
    ///Bit 2 - Random number generator enable
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W<CRrs> {
        RNGEN_W::new(self, 2)
    }
    ///Bit 3 - Interrupt enable
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W<CRrs> {
        IE_W::new(self, 3)
    }
    ///Bit 5 - Clock error detection Note: The clock error detection can be used only when ck_rc48 or ck_pll1_q (ck_pll1_q = 48MHz) source is selected otherwise, CED bit must be equal to 1. The clock error detection cannot be enabled nor disabled on the fly when RNG peripheral is enabled, to enable or disable CED the RNG must be disabled.
    #[inline(always)]
    pub fn ced(&mut self) -> CED_W<CRrs> {
        CED_W::new(self, 5)
    }
}
/**RNG control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#RNG:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
