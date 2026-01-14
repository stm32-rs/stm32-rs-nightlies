///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
/**TAMP1IE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1IE {
    ///0: Tamper x interrupt disabled
    Disabled = 0,
    ///1: Tampoer x interrupt enabled
    Enabled = 1,
}
impl From<TAMP1IE> for bool {
    #[inline(always)]
    fn from(variant: TAMP1IE) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMP1IE` reader - TAMP1IE
pub type TAMP1IE_R = crate::BitReader<TAMP1IE>;
impl TAMP1IE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAMP1IE {
        match self.bits {
            false => TAMP1IE::Disabled,
            true => TAMP1IE::Enabled,
        }
    }
    ///Tamper x interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TAMP1IE::Disabled
    }
    ///Tampoer x interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TAMP1IE::Enabled
    }
}
///Field `TAMP1IE` writer - TAMP1IE
pub type TAMP1IE_W<'a, REG> = crate::BitWriter<'a, REG, TAMP1IE>;
impl<'a, REG> TAMP1IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Tamper x interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1IE::Disabled)
    }
    ///Tampoer x interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1IE::Enabled)
    }
}
///Field `TAMP2IE` reader - TAMP2IE
pub use TAMP1IE_R as TAMP2IE_R;
///Field `TAMP3IE` reader - TAMP3IE
pub use TAMP1IE_R as TAMP3IE_R;
///Field `TAMP2IE` writer - TAMP2IE
pub use TAMP1IE_W as TAMP2IE_W;
///Field `TAMP3IE` writer - TAMP3IE
pub use TAMP1IE_W as TAMP3IE_W;
/**ITAMP3IE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP3IE {
    ///0: Internal tamper x interrupt disabled
    Disabled = 0,
    ///1: Internal tamper x interrupt enabled
    Enabled = 1,
}
impl From<ITAMP3IE> for bool {
    #[inline(always)]
    fn from(variant: ITAMP3IE) -> Self {
        variant as u8 != 0
    }
}
///Field `ITAMP3IE` reader - ITAMP3IE
pub type ITAMP3IE_R = crate::BitReader<ITAMP3IE>;
impl ITAMP3IE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP3IE {
        match self.bits {
            false => ITAMP3IE::Disabled,
            true => ITAMP3IE::Enabled,
        }
    }
    ///Internal tamper x interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ITAMP3IE::Disabled
    }
    ///Internal tamper x interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ITAMP3IE::Enabled
    }
}
///Field `ITAMP3IE` writer - ITAMP3IE
pub type ITAMP3IE_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP3IE>;
impl<'a, REG> ITAMP3IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Internal tamper x interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP3IE::Disabled)
    }
    ///Internal tamper x interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP3IE::Enabled)
    }
}
///Field `ITAMP5IE` reader - ITAMP5IE
pub use ITAMP3IE_R as ITAMP5IE_R;
///Field `ITAMP6IE` reader - ITAMP6IE
pub use ITAMP3IE_R as ITAMP6IE_R;
///Field `ITAMP8IE` reader - ITAMP8IE
pub use ITAMP3IE_R as ITAMP8IE_R;
///Field `ITAMP5IE` writer - ITAMP5IE
pub use ITAMP3IE_W as ITAMP5IE_W;
///Field `ITAMP6IE` writer - ITAMP6IE
pub use ITAMP3IE_W as ITAMP6IE_W;
///Field `ITAMP8IE` writer - ITAMP8IE
pub use ITAMP3IE_W as ITAMP8IE_W;
impl R {
    ///Bit 0 - TAMP1IE
    #[inline(always)]
    pub fn tamp1ie(&self) -> TAMP1IE_R {
        TAMP1IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TAMP2IE
    #[inline(always)]
    pub fn tamp2ie(&self) -> TAMP2IE_R {
        TAMP2IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TAMP3IE
    #[inline(always)]
    pub fn tamp3ie(&self) -> TAMP3IE_R {
        TAMP3IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 18 - ITAMP3IE
    #[inline(always)]
    pub fn itamp3ie(&self) -> ITAMP3IE_R {
        ITAMP3IE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - ITAMP5IE
    #[inline(always)]
    pub fn itamp5ie(&self) -> ITAMP5IE_R {
        ITAMP5IE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - ITAMP6IE
    #[inline(always)]
    pub fn itamp6ie(&self) -> ITAMP6IE_R {
        ITAMP6IE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - ITAMP8IE
    #[inline(always)]
    pub fn itamp8ie(&self) -> ITAMP8IE_R {
        ITAMP8IE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("tamp1ie", &self.tamp1ie())
            .field("tamp2ie", &self.tamp2ie())
            .field("tamp3ie", &self.tamp3ie())
            .field("itamp3ie", &self.itamp3ie())
            .field("itamp5ie", &self.itamp5ie())
            .field("itamp6ie", &self.itamp6ie())
            .field("itamp8ie", &self.itamp8ie())
            .finish()
    }
}
impl W {
    ///Bit 0 - TAMP1IE
    #[inline(always)]
    pub fn tamp1ie(&mut self) -> TAMP1IE_W<'_, IERrs> {
        TAMP1IE_W::new(self, 0)
    }
    ///Bit 1 - TAMP2IE
    #[inline(always)]
    pub fn tamp2ie(&mut self) -> TAMP2IE_W<'_, IERrs> {
        TAMP2IE_W::new(self, 1)
    }
    ///Bit 2 - TAMP3IE
    #[inline(always)]
    pub fn tamp3ie(&mut self) -> TAMP3IE_W<'_, IERrs> {
        TAMP3IE_W::new(self, 2)
    }
    ///Bit 18 - ITAMP3IE
    #[inline(always)]
    pub fn itamp3ie(&mut self) -> ITAMP3IE_W<'_, IERrs> {
        ITAMP3IE_W::new(self, 18)
    }
    ///Bit 20 - ITAMP5IE
    #[inline(always)]
    pub fn itamp5ie(&mut self) -> ITAMP5IE_W<'_, IERrs> {
        ITAMP5IE_W::new(self, 20)
    }
    ///Bit 21 - ITAMP6IE
    #[inline(always)]
    pub fn itamp6ie(&mut self) -> ITAMP6IE_W<'_, IERrs> {
        ITAMP6IE_W::new(self, 21)
    }
    ///Bit 23 - ITAMP8IE
    #[inline(always)]
    pub fn itamp8ie(&mut self) -> ITAMP8IE_W<'_, IERrs> {
        ITAMP8IE_W::new(self, 23)
    }
}
/**TAMP interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#TAMP:IER)*/
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
///`read()` method returns [`ier::R`](R) reader structure
impl crate::Readable for IERrs {}
///`write(|w| ..)` method takes [`ier::W`](W) writer structure
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IERrs {}
