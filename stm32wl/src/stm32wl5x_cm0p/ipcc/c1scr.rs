#[doc = "Register `C1SCR` reader"]
pub type R = crate::R<C1SCRrs>;
#[doc = "Register `C1SCR` writer"]
pub type W = crate::W<C1SCRrs>;
#[doc = "CH1C\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1C {
    #[doc = "0: No action"]
    NoAction = 0,
    #[doc = "1: Processor receive channel n status bit clear"]
    Clear = 1,
}
impl From<CH1C> for bool {
    #[inline(always)]
    fn from(variant: CH1C) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1C` reader - CH1C"]
pub type CH1C_R = crate::BitReader<CH1C>;
impl CH1C_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH1C {
        match self.bits {
            false => CH1C::NoAction,
            true => CH1C::Clear,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == CH1C::NoAction
    }
    #[doc = "Processor receive channel n status bit clear"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CH1C::Clear
    }
}
#[doc = "Field `CH1C` writer - CH1C"]
pub type CH1C_W<'a, REG> = crate::BitWriter<'a, REG, CH1C>;
impl<'a, REG> CH1C_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(CH1C::NoAction)
    }
    #[doc = "Processor receive channel n status bit clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CH1C::Clear)
    }
}
#[doc = "Field `CH2C` reader - CH2C"]
pub use CH1C_R as CH2C_R;
#[doc = "Field `CH3C` reader - CH3C"]
pub use CH1C_R as CH3C_R;
#[doc = "Field `CH4C` reader - CH4C"]
pub use CH1C_R as CH4C_R;
#[doc = "Field `CH5C` reader - CH5C"]
pub use CH1C_R as CH5C_R;
#[doc = "Field `CH6C` reader - CH6C"]
pub use CH1C_R as CH6C_R;
#[doc = "Field `CH2C` writer - CH2C"]
pub use CH1C_W as CH2C_W;
#[doc = "Field `CH3C` writer - CH3C"]
pub use CH1C_W as CH3C_W;
#[doc = "Field `CH4C` writer - CH4C"]
pub use CH1C_W as CH4C_W;
#[doc = "Field `CH5C` writer - CH5C"]
pub use CH1C_W as CH5C_W;
#[doc = "Field `CH6C` writer - CH6C"]
pub use CH1C_W as CH6C_W;
#[doc = "CH1S\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1S {
    #[doc = "0: No action"]
    NoAction = 0,
    #[doc = "1: Processor transmit channel n status bit set"]
    Set = 1,
}
impl From<CH1S> for bool {
    #[inline(always)]
    fn from(variant: CH1S) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1S` reader - CH1S"]
pub type CH1S_R = crate::BitReader<CH1S>;
impl CH1S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH1S {
        match self.bits {
            false => CH1S::NoAction,
            true => CH1S::Set,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == CH1S::NoAction
    }
    #[doc = "Processor transmit channel n status bit set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CH1S::Set
    }
}
#[doc = "Field `CH1S` writer - CH1S"]
pub type CH1S_W<'a, REG> = crate::BitWriter<'a, REG, CH1S>;
impl<'a, REG> CH1S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(CH1S::NoAction)
    }
    #[doc = "Processor transmit channel n status bit set"]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(CH1S::Set)
    }
}
#[doc = "Field `CH2S` reader - CH2S"]
pub use CH1S_R as CH2S_R;
#[doc = "Field `CH3S` reader - CH3S"]
pub use CH1S_R as CH3S_R;
#[doc = "Field `CH4S` reader - CH4S"]
pub use CH1S_R as CH4S_R;
#[doc = "Field `CH5S` reader - CH5S"]
pub use CH1S_R as CH5S_R;
#[doc = "Field `CH6S` reader - CH6S"]
pub use CH1S_R as CH6S_R;
#[doc = "Field `CH2S` writer - CH2S"]
pub use CH1S_W as CH2S_W;
#[doc = "Field `CH3S` writer - CH3S"]
pub use CH1S_W as CH3S_W;
#[doc = "Field `CH4S` writer - CH4S"]
pub use CH1S_W as CH4S_W;
#[doc = "Field `CH5S` writer - CH5S"]
pub use CH1S_W as CH5S_W;
#[doc = "Field `CH6S` writer - CH6S"]
pub use CH1S_W as CH6S_W;
impl R {
    #[doc = "Bit 0 - CH1C"]
    #[inline(always)]
    pub fn ch1c(&self) -> CH1C_R {
        CH1C_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH2C"]
    #[inline(always)]
    pub fn ch2c(&self) -> CH2C_R {
        CH2C_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH3C"]
    #[inline(always)]
    pub fn ch3c(&self) -> CH3C_R {
        CH3C_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH4C"]
    #[inline(always)]
    pub fn ch4c(&self) -> CH4C_R {
        CH4C_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CH5C"]
    #[inline(always)]
    pub fn ch5c(&self) -> CH5C_R {
        CH5C_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH6C"]
    #[inline(always)]
    pub fn ch6c(&self) -> CH6C_R {
        CH6C_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - CH1S"]
    #[inline(always)]
    pub fn ch1s(&self) -> CH1S_R {
        CH1S_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CH2S"]
    #[inline(always)]
    pub fn ch2s(&self) -> CH2S_R {
        CH2S_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CH3S"]
    #[inline(always)]
    pub fn ch3s(&self) -> CH3S_R {
        CH3S_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CH4S"]
    #[inline(always)]
    pub fn ch4s(&self) -> CH4S_R {
        CH4S_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CH5S"]
    #[inline(always)]
    pub fn ch5s(&self) -> CH5S_R {
        CH5S_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CH6S"]
    #[inline(always)]
    pub fn ch6s(&self) -> CH6S_R {
        CH6S_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH1C"]
    #[inline(always)]
    #[must_use]
    pub fn ch1c(&mut self) -> CH1C_W<C1SCRrs> {
        CH1C_W::new(self, 0)
    }
    #[doc = "Bit 1 - CH2C"]
    #[inline(always)]
    #[must_use]
    pub fn ch2c(&mut self) -> CH2C_W<C1SCRrs> {
        CH2C_W::new(self, 1)
    }
    #[doc = "Bit 2 - CH3C"]
    #[inline(always)]
    #[must_use]
    pub fn ch3c(&mut self) -> CH3C_W<C1SCRrs> {
        CH3C_W::new(self, 2)
    }
    #[doc = "Bit 3 - CH4C"]
    #[inline(always)]
    #[must_use]
    pub fn ch4c(&mut self) -> CH4C_W<C1SCRrs> {
        CH4C_W::new(self, 3)
    }
    #[doc = "Bit 4 - CH5C"]
    #[inline(always)]
    #[must_use]
    pub fn ch5c(&mut self) -> CH5C_W<C1SCRrs> {
        CH5C_W::new(self, 4)
    }
    #[doc = "Bit 5 - CH6C"]
    #[inline(always)]
    #[must_use]
    pub fn ch6c(&mut self) -> CH6C_W<C1SCRrs> {
        CH6C_W::new(self, 5)
    }
    #[doc = "Bit 16 - CH1S"]
    #[inline(always)]
    #[must_use]
    pub fn ch1s(&mut self) -> CH1S_W<C1SCRrs> {
        CH1S_W::new(self, 16)
    }
    #[doc = "Bit 17 - CH2S"]
    #[inline(always)]
    #[must_use]
    pub fn ch2s(&mut self) -> CH2S_W<C1SCRrs> {
        CH2S_W::new(self, 17)
    }
    #[doc = "Bit 18 - CH3S"]
    #[inline(always)]
    #[must_use]
    pub fn ch3s(&mut self) -> CH3S_W<C1SCRrs> {
        CH3S_W::new(self, 18)
    }
    #[doc = "Bit 19 - CH4S"]
    #[inline(always)]
    #[must_use]
    pub fn ch4s(&mut self) -> CH4S_W<C1SCRrs> {
        CH4S_W::new(self, 19)
    }
    #[doc = "Bit 20 - CH5S"]
    #[inline(always)]
    #[must_use]
    pub fn ch5s(&mut self) -> CH5S_W<C1SCRrs> {
        CH5S_W::new(self, 20)
    }
    #[doc = "Bit 21 - CH6S"]
    #[inline(always)]
    #[must_use]
    pub fn ch6s(&mut self) -> CH6S_W<C1SCRrs> {
        CH6S_W::new(self, 21)
    }
}
#[doc = "Reading this register will always return 0x0000 0000.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1scr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1scr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1SCRrs;
impl crate::RegisterSpec for C1SCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1scr::R`](R) reader structure"]
impl crate::Readable for C1SCRrs {}
#[doc = "`write(|w| ..)` method takes [`c1scr::W`](W) writer structure"]
impl crate::Writable for C1SCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C1SCR to value 0"]
impl crate::Resettable for C1SCRrs {
    const RESET_VALUE: u32 = 0;
}
