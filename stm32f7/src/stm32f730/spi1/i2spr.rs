///Register `I2SPR` reader
pub type R = crate::R<I2SPRrs>;
///Register `I2SPR` writer
pub type W = crate::W<I2SPRrs>;
///Field `I2SDIV` reader - I2S Linear prescaler
pub type I2SDIV_R = crate::FieldReader;
///Field `I2SDIV` writer - I2S Linear prescaler
pub type I2SDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
/**Odd factor for the prescaler

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ODD {
    ///0: Real divider value is I2SDIV * 2
    Even = 0,
    ///1: Real divider value is (I2SDIV * 2) + 1
    Odd = 1,
}
impl From<ODD> for bool {
    #[inline(always)]
    fn from(variant: ODD) -> Self {
        variant as u8 != 0
    }
}
///Field `ODD` reader - Odd factor for the prescaler
pub type ODD_R = crate::BitReader<ODD>;
impl ODD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ODD {
        match self.bits {
            false => ODD::Even,
            true => ODD::Odd,
        }
    }
    ///Real divider value is I2SDIV * 2
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == ODD::Even
    }
    ///Real divider value is (I2SDIV * 2) + 1
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == ODD::Odd
    }
}
///Field `ODD` writer - Odd factor for the prescaler
pub type ODD_W<'a, REG> = crate::BitWriter<'a, REG, ODD>;
impl<'a, REG> ODD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Real divider value is I2SDIV * 2
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(ODD::Even)
    }
    ///Real divider value is (I2SDIV * 2) + 1
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(ODD::Odd)
    }
}
/**Master clock output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCKOE {
    ///0: Master clock output is disabled
    Disabled = 0,
    ///1: Master clock output is enabled
    Enabled = 1,
}
impl From<MCKOE> for bool {
    #[inline(always)]
    fn from(variant: MCKOE) -> Self {
        variant as u8 != 0
    }
}
///Field `MCKOE` reader - Master clock output enable
pub type MCKOE_R = crate::BitReader<MCKOE>;
impl MCKOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MCKOE {
        match self.bits {
            false => MCKOE::Disabled,
            true => MCKOE::Enabled,
        }
    }
    ///Master clock output is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MCKOE::Disabled
    }
    ///Master clock output is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MCKOE::Enabled
    }
}
///Field `MCKOE` writer - Master clock output enable
pub type MCKOE_W<'a, REG> = crate::BitWriter<'a, REG, MCKOE>;
impl<'a, REG> MCKOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Master clock output is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MCKOE::Disabled)
    }
    ///Master clock output is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MCKOE::Enabled)
    }
}
impl R {
    ///Bits 0:7 - I2S Linear prescaler
    #[inline(always)]
    pub fn i2sdiv(&self) -> I2SDIV_R {
        I2SDIV_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - Odd factor for the prescaler
    #[inline(always)]
    pub fn odd(&self) -> ODD_R {
        ODD_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Master clock output enable
    #[inline(always)]
    pub fn mckoe(&self) -> MCKOE_R {
        MCKOE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2SPR")
            .field("mckoe", &self.mckoe())
            .field("odd", &self.odd())
            .field("i2sdiv", &self.i2sdiv())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - I2S Linear prescaler
    #[inline(always)]
    pub fn i2sdiv(&mut self) -> I2SDIV_W<'_, I2SPRrs> {
        I2SDIV_W::new(self, 0)
    }
    ///Bit 8 - Odd factor for the prescaler
    #[inline(always)]
    pub fn odd(&mut self) -> ODD_W<'_, I2SPRrs> {
        ODD_W::new(self, 8)
    }
    ///Bit 9 - Master clock output enable
    #[inline(always)]
    pub fn mckoe(&mut self) -> MCKOE_W<'_, I2SPRrs> {
        MCKOE_W::new(self, 9)
    }
}
/**I2S prescaler register

You can [`read`](crate::Reg::read) this register and get [`i2spr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2spr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F730.html#SPI1:I2SPR)*/
pub struct I2SPRrs;
impl crate::RegisterSpec for I2SPRrs {
    type Ux = u16;
}
///`read()` method returns [`i2spr::R`](R) reader structure
impl crate::Readable for I2SPRrs {}
///`write(|w| ..)` method takes [`i2spr::W`](W) writer structure
impl crate::Writable for I2SPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets I2SPR to value 0x0a
impl crate::Resettable for I2SPRrs {
    const RESET_VALUE: u16 = 0x0a;
}
