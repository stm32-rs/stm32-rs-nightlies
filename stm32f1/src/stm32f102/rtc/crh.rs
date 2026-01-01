///Register `CRH` reader
pub type R = crate::R<CRHrs>;
///Register `CRH` writer
pub type W = crate::W<CRHrs>;
/**Second interrupt Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECIE {
    ///0: Second interrupt is masked
    Disabled = 0,
    ///1: Second interrupt is enabled
    Enabled = 1,
}
impl From<SECIE> for bool {
    #[inline(always)]
    fn from(variant: SECIE) -> Self {
        variant as u8 != 0
    }
}
///Field `SECIE` reader - Second interrupt Enable
pub type SECIE_R = crate::BitReader<SECIE>;
impl SECIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SECIE {
        match self.bits {
            false => SECIE::Disabled,
            true => SECIE::Enabled,
        }
    }
    ///Second interrupt is masked
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SECIE::Disabled
    }
    ///Second interrupt is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SECIE::Enabled
    }
}
///Field `SECIE` writer - Second interrupt Enable
pub type SECIE_W<'a, REG> = crate::BitWriter<'a, REG, SECIE>;
impl<'a, REG> SECIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Second interrupt is masked
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SECIE::Disabled)
    }
    ///Second interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SECIE::Enabled)
    }
}
/**Alarm interrupt Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRIE {
    ///0: Alarm interrupt is masked
    Disabled = 0,
    ///1: Alarm interrupt is enabled
    Enabled = 1,
}
impl From<ALRIE> for bool {
    #[inline(always)]
    fn from(variant: ALRIE) -> Self {
        variant as u8 != 0
    }
}
///Field `ALRIE` reader - Alarm interrupt Enable
pub type ALRIE_R = crate::BitReader<ALRIE>;
impl ALRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ALRIE {
        match self.bits {
            false => ALRIE::Disabled,
            true => ALRIE::Enabled,
        }
    }
    ///Alarm interrupt is masked
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ALRIE::Disabled
    }
    ///Alarm interrupt is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ALRIE::Enabled
    }
}
///Field `ALRIE` writer - Alarm interrupt Enable
pub type ALRIE_W<'a, REG> = crate::BitWriter<'a, REG, ALRIE>;
impl<'a, REG> ALRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Alarm interrupt is masked
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ALRIE::Disabled)
    }
    ///Alarm interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ALRIE::Enabled)
    }
}
/**Overflow interrupt Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OWIE {
    ///0: Overflow interrupt is masked
    Disabled = 0,
    ///1: Overflow interrupt is enabled
    Enabled = 1,
}
impl From<OWIE> for bool {
    #[inline(always)]
    fn from(variant: OWIE) -> Self {
        variant as u8 != 0
    }
}
///Field `OWIE` reader - Overflow interrupt Enable
pub type OWIE_R = crate::BitReader<OWIE>;
impl OWIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OWIE {
        match self.bits {
            false => OWIE::Disabled,
            true => OWIE::Enabled,
        }
    }
    ///Overflow interrupt is masked
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OWIE::Disabled
    }
    ///Overflow interrupt is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OWIE::Enabled
    }
}
///Field `OWIE` writer - Overflow interrupt Enable
pub type OWIE_W<'a, REG> = crate::BitWriter<'a, REG, OWIE>;
impl<'a, REG> OWIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Overflow interrupt is masked
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OWIE::Disabled)
    }
    ///Overflow interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OWIE::Enabled)
    }
}
impl R {
    ///Bit 0 - Second interrupt Enable
    #[inline(always)]
    pub fn secie(&self) -> SECIE_R {
        SECIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Alarm interrupt Enable
    #[inline(always)]
    pub fn alrie(&self) -> ALRIE_R {
        ALRIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Overflow interrupt Enable
    #[inline(always)]
    pub fn owie(&self) -> OWIE_R {
        OWIE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRH")
            .field("secie", &self.secie())
            .field("alrie", &self.alrie())
            .field("owie", &self.owie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Second interrupt Enable
    #[inline(always)]
    pub fn secie(&mut self) -> SECIE_W<'_, CRHrs> {
        SECIE_W::new(self, 0)
    }
    ///Bit 1 - Alarm interrupt Enable
    #[inline(always)]
    pub fn alrie(&mut self) -> ALRIE_W<'_, CRHrs> {
        ALRIE_W::new(self, 1)
    }
    ///Bit 2 - Overflow interrupt Enable
    #[inline(always)]
    pub fn owie(&mut self) -> OWIE_W<'_, CRHrs> {
        OWIE_W::new(self, 2)
    }
}
/**RTC Control Register High

You can [`read`](crate::Reg::read) this register and get [`crh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F102.html#RTC:CRH)*/
pub struct CRHrs;
impl crate::RegisterSpec for CRHrs {
    type Ux = u32;
}
///`read()` method returns [`crh::R`](R) reader structure
impl crate::Readable for CRHrs {}
///`write(|w| ..)` method takes [`crh::W`](W) writer structure
impl crate::Writable for CRHrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRH to value 0
impl crate::Resettable for CRHrs {}
