///Register `CCR` reader
pub type R = crate::R<CCRrs>;
///Register `CCR` writer
pub type W = crate::W<CCRrs>;
/**Temperature sensor and VREFINT enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREFEN {
    ///0: V_REFINT channel disabled
    Disabled = 0,
    ///1: V_REFINT channel enabled
    Enabled = 1,
}
impl From<VREFEN> for bool {
    #[inline(always)]
    fn from(variant: VREFEN) -> Self {
        variant as u8 != 0
    }
}
///Field `VREFEN` reader - Temperature sensor and VREFINT enable
pub type VREFEN_R = crate::BitReader<VREFEN>;
impl VREFEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VREFEN {
        match self.bits {
            false => VREFEN::Disabled,
            true => VREFEN::Enabled,
        }
    }
    ///V_REFINT channel disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VREFEN::Disabled
    }
    ///V_REFINT channel enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VREFEN::Enabled
    }
}
///Field `VREFEN` writer - Temperature sensor and VREFINT enable
pub type VREFEN_W<'a, REG> = crate::BitWriter<'a, REG, VREFEN>;
impl<'a, REG> VREFEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///V_REFINT channel disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VREFEN::Disabled)
    }
    ///V_REFINT channel enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VREFEN::Enabled)
    }
}
/**Temperature sensor enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSEN {
    ///0: Temperature sensor disabled
    Disabled = 0,
    ///1: Temperature sensor enabled
    Enabled = 1,
}
impl From<TSEN> for bool {
    #[inline(always)]
    fn from(variant: TSEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TSEN` reader - Temperature sensor enable
pub type TSEN_R = crate::BitReader<TSEN>;
impl TSEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSEN {
        match self.bits {
            false => TSEN::Disabled,
            true => TSEN::Enabled,
        }
    }
    ///Temperature sensor disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSEN::Disabled
    }
    ///Temperature sensor enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSEN::Enabled
    }
}
///Field `TSEN` writer - Temperature sensor enable
pub type TSEN_W<'a, REG> = crate::BitWriter<'a, REG, TSEN>;
impl<'a, REG> TSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Temperature sensor disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSEN::Disabled)
    }
    ///Temperature sensor enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSEN::Enabled)
    }
}
impl R {
    ///Bit 22 - Temperature sensor and VREFINT enable
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Temperature sensor enable
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("tsen", &self.tsen())
            .field("vrefen", &self.vrefen())
            .finish()
    }
}
impl W {
    ///Bit 22 - Temperature sensor and VREFINT enable
    #[inline(always)]
    pub fn vrefen(&mut self) -> VREFEN_W<'_, CCRrs> {
        VREFEN_W::new(self, 22)
    }
    ///Bit 23 - Temperature sensor enable
    #[inline(always)]
    pub fn tsen(&mut self) -> TSEN_W<'_, CCRrs> {
        TSEN_W::new(self, 23)
    }
}
/**common configuration register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x0.html#ADC:CCR)*/
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
///`read()` method returns [`ccr::R`](R) reader structure
impl crate::Readable for CCRrs {}
///`write(|w| ..)` method takes [`ccr::W`](W) writer structure
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCR to value 0
impl crate::Resettable for CCRrs {}
