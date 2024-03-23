#[doc = "Register `I2SPR` reader"]
pub type R = crate::R<I2SPRrs>;
#[doc = "Register `I2SPR` writer"]
pub type W = crate::W<I2SPRrs>;
#[doc = "Field `I2SDIV` reader - I2S Linear prescaler"]
pub type I2SDIV_R = crate::FieldReader;
#[doc = "Field `I2SDIV` writer - I2S Linear prescaler"]
pub type I2SDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Odd factor for the prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ODD {
    #[doc = "0: Real divider value is I2SDIV * 2"]
    Even = 0,
    #[doc = "1: Real divider value is (I2SDIV * 2) + 1"]
    Odd = 1,
}
impl From<ODD> for bool {
    #[inline(always)]
    fn from(variant: ODD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ODD` reader - Odd factor for the prescaler"]
pub type ODD_R = crate::BitReader<ODD>;
impl ODD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ODD {
        match self.bits {
            false => ODD::Even,
            true => ODD::Odd,
        }
    }
    #[doc = "Real divider value is I2SDIV * 2"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == ODD::Even
    }
    #[doc = "Real divider value is (I2SDIV * 2) + 1"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == ODD::Odd
    }
}
#[doc = "Field `ODD` writer - Odd factor for the prescaler"]
pub type ODD_W<'a, REG> = crate::BitWriter<'a, REG, ODD>;
impl<'a, REG> ODD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Real divider value is I2SDIV * 2"]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(ODD::Even)
    }
    #[doc = "Real divider value is (I2SDIV * 2) + 1"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(ODD::Odd)
    }
}
#[doc = "Master clock output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCKOE {
    #[doc = "0: Master clock output is disabled"]
    Disabled = 0,
    #[doc = "1: Master clock output is enabled"]
    Enabled = 1,
}
impl From<MCKOE> for bool {
    #[inline(always)]
    fn from(variant: MCKOE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCKOE` reader - Master clock output enable"]
pub type MCKOE_R = crate::BitReader<MCKOE>;
impl MCKOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MCKOE {
        match self.bits {
            false => MCKOE::Disabled,
            true => MCKOE::Enabled,
        }
    }
    #[doc = "Master clock output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MCKOE::Disabled
    }
    #[doc = "Master clock output is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MCKOE::Enabled
    }
}
#[doc = "Field `MCKOE` writer - Master clock output enable"]
pub type MCKOE_W<'a, REG> = crate::BitWriter<'a, REG, MCKOE>;
impl<'a, REG> MCKOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master clock output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MCKOE::Disabled)
    }
    #[doc = "Master clock output is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MCKOE::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:7 - I2S Linear prescaler"]
    #[inline(always)]
    pub fn i2sdiv(&self) -> I2SDIV_R {
        I2SDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Odd factor for the prescaler"]
    #[inline(always)]
    pub fn odd(&self) -> ODD_R {
        ODD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Master clock output enable"]
    #[inline(always)]
    pub fn mckoe(&self) -> MCKOE_R {
        MCKOE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2S Linear prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn i2sdiv(&mut self) -> I2SDIV_W<I2SPRrs> {
        I2SDIV_W::new(self, 0)
    }
    #[doc = "Bit 8 - Odd factor for the prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn odd(&mut self) -> ODD_W<I2SPRrs> {
        ODD_W::new(self, 8)
    }
    #[doc = "Bit 9 - Master clock output enable"]
    #[inline(always)]
    #[must_use]
    pub fn mckoe(&mut self) -> MCKOE_W<I2SPRrs> {
        MCKOE_W::new(self, 9)
    }
}
#[doc = "I2S prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2spr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2spr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2SPRrs;
impl crate::RegisterSpec for I2SPRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2spr::R`](R) reader structure"]
impl crate::Readable for I2SPRrs {}
#[doc = "`write(|w| ..)` method takes [`i2spr::W`](W) writer structure"]
impl crate::Writable for I2SPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2SPR to value 0x02"]
impl crate::Resettable for I2SPRrs {
    const RESET_VALUE: u32 = 0x02;
}
