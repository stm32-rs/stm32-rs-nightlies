///Register `C2SCR` reader
pub type R = crate::R<C2SCRrs>;
///Register `C2SCR` writer
pub type W = crate::W<C2SCRrs>;
/**CH1C

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1C {
    ///0: No action
    NoAction = 0,
    ///1: Processor receive channel n status bit clear
    Clear = 1,
}
impl From<CH1C> for bool {
    #[inline(always)]
    fn from(variant: CH1C) -> Self {
        variant as u8 != 0
    }
}
///Field `CH1C` reader - CH1C
pub type CH1C_R = crate::BitReader<CH1C>;
impl CH1C_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CH1C {
        match self.bits {
            false => CH1C::NoAction,
            true => CH1C::Clear,
        }
    }
    ///No action
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == CH1C::NoAction
    }
    ///Processor receive channel n status bit clear
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CH1C::Clear
    }
}
///Field `CH1C` writer - CH1C
pub type CH1C_W<'a, REG> = crate::BitWriter<'a, REG, CH1C>;
impl<'a, REG> CH1C_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(CH1C::NoAction)
    }
    ///Processor receive channel n status bit clear
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CH1C::Clear)
    }
}
///Field `CH2C` reader - CH2C
pub use CH1C_R as CH2C_R;
///Field `CH3C` reader - CH3C
pub use CH1C_R as CH3C_R;
///Field `CH4C` reader - CH4C
pub use CH1C_R as CH4C_R;
///Field `CH5C` reader - CH5C
pub use CH1C_R as CH5C_R;
///Field `CH6C` reader - CH6C
pub use CH1C_R as CH6C_R;
///Field `CH2C` writer - CH2C
pub use CH1C_W as CH2C_W;
///Field `CH3C` writer - CH3C
pub use CH1C_W as CH3C_W;
///Field `CH4C` writer - CH4C
pub use CH1C_W as CH4C_W;
///Field `CH5C` writer - CH5C
pub use CH1C_W as CH5C_W;
///Field `CH6C` writer - CH6C
pub use CH1C_W as CH6C_W;
/**CH1S

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1S {
    ///0: No action
    NoAction = 0,
    ///1: Processor transmit channel n status bit set
    Set = 1,
}
impl From<CH1S> for bool {
    #[inline(always)]
    fn from(variant: CH1S) -> Self {
        variant as u8 != 0
    }
}
///Field `CH1S` reader - CH1S
pub type CH1S_R = crate::BitReader<CH1S>;
impl CH1S_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CH1S {
        match self.bits {
            false => CH1S::NoAction,
            true => CH1S::Set,
        }
    }
    ///No action
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == CH1S::NoAction
    }
    ///Processor transmit channel n status bit set
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CH1S::Set
    }
}
///Field `CH1S` writer - CH1S
pub type CH1S_W<'a, REG> = crate::BitWriter<'a, REG, CH1S>;
impl<'a, REG> CH1S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(CH1S::NoAction)
    }
    ///Processor transmit channel n status bit set
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(CH1S::Set)
    }
}
///Field `CH2S` reader - CH2S
pub use CH1S_R as CH2S_R;
///Field `CH3S` reader - CH3S
pub use CH1S_R as CH3S_R;
///Field `CH4S` reader - CH4S
pub use CH1S_R as CH4S_R;
///Field `CH5S` reader - CH5S
pub use CH1S_R as CH5S_R;
///Field `CH6S` reader - CH6S
pub use CH1S_R as CH6S_R;
///Field `CH2S` writer - CH2S
pub use CH1S_W as CH2S_W;
///Field `CH3S` writer - CH3S
pub use CH1S_W as CH3S_W;
///Field `CH4S` writer - CH4S
pub use CH1S_W as CH4S_W;
///Field `CH5S` writer - CH5S
pub use CH1S_W as CH5S_W;
///Field `CH6S` writer - CH6S
pub use CH1S_W as CH6S_W;
impl R {
    ///Bit 0 - CH1C
    #[inline(always)]
    pub fn ch1c(&self) -> CH1C_R {
        CH1C_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CH2C
    #[inline(always)]
    pub fn ch2c(&self) -> CH2C_R {
        CH2C_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CH3C
    #[inline(always)]
    pub fn ch3c(&self) -> CH3C_R {
        CH3C_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CH4C
    #[inline(always)]
    pub fn ch4c(&self) -> CH4C_R {
        CH4C_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CH5C
    #[inline(always)]
    pub fn ch5c(&self) -> CH5C_R {
        CH5C_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CH6C
    #[inline(always)]
    pub fn ch6c(&self) -> CH6C_R {
        CH6C_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 16 - CH1S
    #[inline(always)]
    pub fn ch1s(&self) -> CH1S_R {
        CH1S_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CH2S
    #[inline(always)]
    pub fn ch2s(&self) -> CH2S_R {
        CH2S_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - CH3S
    #[inline(always)]
    pub fn ch3s(&self) -> CH3S_R {
        CH3S_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - CH4S
    #[inline(always)]
    pub fn ch4s(&self) -> CH4S_R {
        CH4S_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - CH5S
    #[inline(always)]
    pub fn ch5s(&self) -> CH5S_R {
        CH5S_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - CH6S
    #[inline(always)]
    pub fn ch6s(&self) -> CH6S_R {
        CH6S_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2SCR")
            .field("ch1c", &self.ch1c())
            .field("ch2c", &self.ch2c())
            .field("ch3c", &self.ch3c())
            .field("ch4c", &self.ch4c())
            .field("ch5c", &self.ch5c())
            .field("ch6c", &self.ch6c())
            .field("ch1s", &self.ch1s())
            .field("ch2s", &self.ch2s())
            .field("ch3s", &self.ch3s())
            .field("ch4s", &self.ch4s())
            .field("ch5s", &self.ch5s())
            .field("ch6s", &self.ch6s())
            .finish()
    }
}
impl W {
    ///Bit 0 - CH1C
    #[inline(always)]
    pub fn ch1c(&mut self) -> CH1C_W<'_, C2SCRrs> {
        CH1C_W::new(self, 0)
    }
    ///Bit 1 - CH2C
    #[inline(always)]
    pub fn ch2c(&mut self) -> CH2C_W<'_, C2SCRrs> {
        CH2C_W::new(self, 1)
    }
    ///Bit 2 - CH3C
    #[inline(always)]
    pub fn ch3c(&mut self) -> CH3C_W<'_, C2SCRrs> {
        CH3C_W::new(self, 2)
    }
    ///Bit 3 - CH4C
    #[inline(always)]
    pub fn ch4c(&mut self) -> CH4C_W<'_, C2SCRrs> {
        CH4C_W::new(self, 3)
    }
    ///Bit 4 - CH5C
    #[inline(always)]
    pub fn ch5c(&mut self) -> CH5C_W<'_, C2SCRrs> {
        CH5C_W::new(self, 4)
    }
    ///Bit 5 - CH6C
    #[inline(always)]
    pub fn ch6c(&mut self) -> CH6C_W<'_, C2SCRrs> {
        CH6C_W::new(self, 5)
    }
    ///Bit 16 - CH1S
    #[inline(always)]
    pub fn ch1s(&mut self) -> CH1S_W<'_, C2SCRrs> {
        CH1S_W::new(self, 16)
    }
    ///Bit 17 - CH2S
    #[inline(always)]
    pub fn ch2s(&mut self) -> CH2S_W<'_, C2SCRrs> {
        CH2S_W::new(self, 17)
    }
    ///Bit 18 - CH3S
    #[inline(always)]
    pub fn ch3s(&mut self) -> CH3S_W<'_, C2SCRrs> {
        CH3S_W::new(self, 18)
    }
    ///Bit 19 - CH4S
    #[inline(always)]
    pub fn ch4s(&mut self) -> CH4S_W<'_, C2SCRrs> {
        CH4S_W::new(self, 19)
    }
    ///Bit 20 - CH5S
    #[inline(always)]
    pub fn ch5s(&mut self) -> CH5S_W<'_, C2SCRrs> {
        CH5S_W::new(self, 20)
    }
    ///Bit 21 - CH6S
    #[inline(always)]
    pub fn ch6s(&mut self) -> CH6S_W<'_, C2SCRrs> {
        CH6S_W::new(self, 21)
    }
}
/**Reading this register will always return 0x0000 0000.

You can [`read`](crate::Reg::read) this register and get [`c2scr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2scr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#IPCC:C2SCR)*/
pub struct C2SCRrs;
impl crate::RegisterSpec for C2SCRrs {
    type Ux = u32;
}
///`read()` method returns [`c2scr::R`](R) reader structure
impl crate::Readable for C2SCRrs {}
///`write(|w| ..)` method takes [`c2scr::W`](W) writer structure
impl crate::Writable for C2SCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2SCR to value 0
impl crate::Resettable for C2SCRrs {}
