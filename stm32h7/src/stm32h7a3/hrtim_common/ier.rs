///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
/**Fault 1 Interrupt Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT1IE {
    ///0: Fault interrupt disabled
    Disabled = 0,
    ///1: Fault interrupt enabled
    Enabled = 1,
}
impl From<FLT1IE> for bool {
    #[inline(always)]
    fn from(variant: FLT1IE) -> Self {
        variant as u8 != 0
    }
}
///Field `FLT1IE` reader - Fault 1 Interrupt Enable
pub type FLT1IE_R = crate::BitReader<FLT1IE>;
impl FLT1IE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLT1IE {
        match self.bits {
            false => FLT1IE::Disabled,
            true => FLT1IE::Enabled,
        }
    }
    ///Fault interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLT1IE::Disabled
    }
    ///Fault interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FLT1IE::Enabled
    }
}
///Field `FLT1IE` writer - Fault 1 Interrupt Enable
pub type FLT1IE_W<'a, REG> = crate::BitWriter<'a, REG, FLT1IE>;
impl<'a, REG> FLT1IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Fault interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1IE::Disabled)
    }
    ///Fault interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1IE::Enabled)
    }
}
///Field `FLT2IE` reader - Fault 2 Interrupt Enable
pub use FLT1IE_R as FLT2IE_R;
///Field `FLT3IE` reader - Fault 3 Interrupt Enable
pub use FLT1IE_R as FLT3IE_R;
///Field `FLT4IE` reader - Fault 4 Interrupt Enable
pub use FLT1IE_R as FLT4IE_R;
///Field `FLT5IE` reader - Fault 5 Interrupt Enable
pub use FLT1IE_R as FLT5IE_R;
///Field `SYSFLTIE` reader - System Fault Interrupt Enable
pub use FLT1IE_R as SYSFLTIE_R;
///Field `FLT2IE` writer - Fault 2 Interrupt Enable
pub use FLT1IE_W as FLT2IE_W;
///Field `FLT3IE` writer - Fault 3 Interrupt Enable
pub use FLT1IE_W as FLT3IE_W;
///Field `FLT4IE` writer - Fault 4 Interrupt Enable
pub use FLT1IE_W as FLT4IE_W;
///Field `FLT5IE` writer - Fault 5 Interrupt Enable
pub use FLT1IE_W as FLT5IE_W;
///Field `SYSFLTIE` writer - System Fault Interrupt Enable
pub use FLT1IE_W as SYSFLTIE_W;
/**Burst mode period Interrupt Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BMPERIE {
    ///0: Burst mode period interrupt disabled
    Disabled = 0,
    ///1: Burst mode period interrupt enabled
    Enabled = 1,
}
impl From<BMPERIE> for bool {
    #[inline(always)]
    fn from(variant: BMPERIE) -> Self {
        variant as u8 != 0
    }
}
///Field `BMPERIE` reader - Burst mode period Interrupt Enable
pub type BMPERIE_R = crate::BitReader<BMPERIE>;
impl BMPERIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BMPERIE {
        match self.bits {
            false => BMPERIE::Disabled,
            true => BMPERIE::Enabled,
        }
    }
    ///Burst mode period interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BMPERIE::Disabled
    }
    ///Burst mode period interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BMPERIE::Enabled
    }
}
///Field `BMPERIE` writer - Burst mode period Interrupt Enable
pub type BMPERIE_W<'a, REG> = crate::BitWriter<'a, REG, BMPERIE>;
impl<'a, REG> BMPERIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Burst mode period interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BMPERIE::Disabled)
    }
    ///Burst mode period interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BMPERIE::Enabled)
    }
}
impl R {
    ///Bit 0 - Fault 1 Interrupt Enable
    #[inline(always)]
    pub fn flt1ie(&self) -> FLT1IE_R {
        FLT1IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Fault 2 Interrupt Enable
    #[inline(always)]
    pub fn flt2ie(&self) -> FLT2IE_R {
        FLT2IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Fault 3 Interrupt Enable
    #[inline(always)]
    pub fn flt3ie(&self) -> FLT3IE_R {
        FLT3IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Fault 4 Interrupt Enable
    #[inline(always)]
    pub fn flt4ie(&self) -> FLT4IE_R {
        FLT4IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Fault 5 Interrupt Enable
    #[inline(always)]
    pub fn flt5ie(&self) -> FLT5IE_R {
        FLT5IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - System Fault Interrupt Enable
    #[inline(always)]
    pub fn sysfltie(&self) -> SYSFLTIE_R {
        SYSFLTIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 17 - Burst mode period Interrupt Enable
    #[inline(always)]
    pub fn bmperie(&self) -> BMPERIE_R {
        BMPERIE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("bmperie", &self.bmperie())
            .field("flt1ie", &self.flt1ie())
            .field("sysfltie", &self.sysfltie())
            .field("flt5ie", &self.flt5ie())
            .field("flt4ie", &self.flt4ie())
            .field("flt3ie", &self.flt3ie())
            .field("flt2ie", &self.flt2ie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Fault 1 Interrupt Enable
    #[inline(always)]
    pub fn flt1ie(&mut self) -> FLT1IE_W<'_, IERrs> {
        FLT1IE_W::new(self, 0)
    }
    ///Bit 1 - Fault 2 Interrupt Enable
    #[inline(always)]
    pub fn flt2ie(&mut self) -> FLT2IE_W<'_, IERrs> {
        FLT2IE_W::new(self, 1)
    }
    ///Bit 2 - Fault 3 Interrupt Enable
    #[inline(always)]
    pub fn flt3ie(&mut self) -> FLT3IE_W<'_, IERrs> {
        FLT3IE_W::new(self, 2)
    }
    ///Bit 3 - Fault 4 Interrupt Enable
    #[inline(always)]
    pub fn flt4ie(&mut self) -> FLT4IE_W<'_, IERrs> {
        FLT4IE_W::new(self, 3)
    }
    ///Bit 4 - Fault 5 Interrupt Enable
    #[inline(always)]
    pub fn flt5ie(&mut self) -> FLT5IE_W<'_, IERrs> {
        FLT5IE_W::new(self, 4)
    }
    ///Bit 5 - System Fault Interrupt Enable
    #[inline(always)]
    pub fn sysfltie(&mut self) -> SYSFLTIE_W<'_, IERrs> {
        SYSFLTIE_W::new(self, 5)
    }
    ///Bit 17 - Burst mode period Interrupt Enable
    #[inline(always)]
    pub fn bmperie(&mut self) -> BMPERIE_W<'_, IERrs> {
        BMPERIE_W::new(self, 17)
    }
}
/**Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7A3.html#HRTIM_Common:IER)*/
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
