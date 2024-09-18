///Register `C1ICR` reader
pub type R = crate::R<C1ICRrs>;
///Register `C1ICR` writer
pub type W = crate::W<C1ICRrs>;
/**Interrupt(N) semaphore n clear bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISC0R {
    ///0: Always reads 0
    NoEffect = 0,
}
impl From<ISC0R> for bool {
    #[inline(always)]
    fn from(variant: ISC0R) -> Self {
        variant as u8 != 0
    }
}
///Field `ISC0` reader - Interrupt(N) semaphore n clear bit
pub type ISC0_R = crate::BitReader<ISC0R>;
impl ISC0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ISC0R> {
        match self.bits {
            false => Some(ISC0R::NoEffect),
            _ => None,
        }
    }
    ///Always reads 0
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == ISC0R::NoEffect
    }
}
/**Interrupt(N) semaphore n clear bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISC0W {
    ///0: Interrupt semaphore x status ISFx and masked status MISFx not affected
    NoEffect = 0,
    ///1: Interrupt semaphore x status ISFx and masked status MISFx cleared
    Clear = 1,
}
impl From<ISC0W> for bool {
    #[inline(always)]
    fn from(variant: ISC0W) -> Self {
        variant as u8 != 0
    }
}
///Field `ISC0` writer - Interrupt(N) semaphore n clear bit
pub type ISC0_W<'a, REG> = crate::BitWriter<'a, REG, ISC0W>;
impl<'a, REG> ISC0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt semaphore x status ISFx and masked status MISFx not affected
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(ISC0W::NoEffect)
    }
    ///Interrupt semaphore x status ISFx and masked status MISFx cleared
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ISC0W::Clear)
    }
}
///Field `ISC1` reader - Interrupt(N) semaphore n clear bit
pub use ISC0_R as ISC1_R;
///Field `ISC2` reader - Interrupt(N) semaphore n clear bit
pub use ISC0_R as ISC2_R;
///Field `ISC3` reader - Interrupt(N) semaphore n clear bit
pub use ISC0_R as ISC3_R;
///Field `ISC4` reader - Interrupt(N) semaphore n clear bit
pub use ISC0_R as ISC4_R;
///Field `ISC5` reader - Interrupt(N) semaphore n clear bit
pub use ISC0_R as ISC5_R;
///Field `ISC6` reader - Interrupt(N) semaphore n clear bit
pub use ISC0_R as ISC6_R;
///Field `ISC7` reader - Interrupt(N) semaphore n clear bit
pub use ISC0_R as ISC7_R;
///Field `ISC8` reader - Interrupt(N) semaphore n clear bit
pub use ISC0_R as ISC8_R;
///Field `ISC9` reader - Interrupt(N) semaphore n clear bit
pub use ISC0_R as ISC9_R;
///Field `ISC10` reader - Interrupt(N) semaphore n clear bit
pub use ISC0_R as ISC10_R;
///Field `ISC11` reader - Interrupt(N) semaphore n clear bit
pub use ISC0_R as ISC11_R;
///Field `ISC12` reader - Interrupt(N) semaphore n clear bit
pub use ISC0_R as ISC12_R;
///Field `ISC13` reader - Interrupt(N) semaphore n clear bit
pub use ISC0_R as ISC13_R;
///Field `ISC14` reader - Interrupt(N) semaphore n clear bit
pub use ISC0_R as ISC14_R;
///Field `ISC15` reader - Interrupt(N) semaphore n clear bit
pub use ISC0_R as ISC15_R;
///Field `ISC1` writer - Interrupt(N) semaphore n clear bit
pub use ISC0_W as ISC1_W;
///Field `ISC2` writer - Interrupt(N) semaphore n clear bit
pub use ISC0_W as ISC2_W;
///Field `ISC3` writer - Interrupt(N) semaphore n clear bit
pub use ISC0_W as ISC3_W;
///Field `ISC4` writer - Interrupt(N) semaphore n clear bit
pub use ISC0_W as ISC4_W;
///Field `ISC5` writer - Interrupt(N) semaphore n clear bit
pub use ISC0_W as ISC5_W;
///Field `ISC6` writer - Interrupt(N) semaphore n clear bit
pub use ISC0_W as ISC6_W;
///Field `ISC7` writer - Interrupt(N) semaphore n clear bit
pub use ISC0_W as ISC7_W;
///Field `ISC8` writer - Interrupt(N) semaphore n clear bit
pub use ISC0_W as ISC8_W;
///Field `ISC9` writer - Interrupt(N) semaphore n clear bit
pub use ISC0_W as ISC9_W;
///Field `ISC10` writer - Interrupt(N) semaphore n clear bit
pub use ISC0_W as ISC10_W;
///Field `ISC11` writer - Interrupt(N) semaphore n clear bit
pub use ISC0_W as ISC11_W;
///Field `ISC12` writer - Interrupt(N) semaphore n clear bit
pub use ISC0_W as ISC12_W;
///Field `ISC13` writer - Interrupt(N) semaphore n clear bit
pub use ISC0_W as ISC13_W;
///Field `ISC14` writer - Interrupt(N) semaphore n clear bit
pub use ISC0_W as ISC14_W;
///Field `ISC15` writer - Interrupt(N) semaphore n clear bit
pub use ISC0_W as ISC15_W;
impl R {
    ///Bit 0 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    pub fn isc0(&self) -> ISC0_R {
        ISC0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    pub fn isc1(&self) -> ISC1_R {
        ISC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    pub fn isc2(&self) -> ISC2_R {
        ISC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    pub fn isc3(&self) -> ISC3_R {
        ISC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    pub fn isc4(&self) -> ISC4_R {
        ISC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    pub fn isc5(&self) -> ISC5_R {
        ISC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    pub fn isc6(&self) -> ISC6_R {
        ISC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    pub fn isc7(&self) -> ISC7_R {
        ISC7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    pub fn isc8(&self) -> ISC8_R {
        ISC8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    pub fn isc9(&self) -> ISC9_R {
        ISC9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    pub fn isc10(&self) -> ISC10_R {
        ISC10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    pub fn isc11(&self) -> ISC11_R {
        ISC11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    pub fn isc12(&self) -> ISC12_R {
        ISC12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    pub fn isc13(&self) -> ISC13_R {
        ISC13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    pub fn isc14(&self) -> ISC14_R {
        ISC14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    pub fn isc15(&self) -> ISC15_R {
        ISC15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C1ICR")
            .field("isc0", &self.isc0())
            .field("isc1", &self.isc1())
            .field("isc2", &self.isc2())
            .field("isc3", &self.isc3())
            .field("isc4", &self.isc4())
            .field("isc5", &self.isc5())
            .field("isc6", &self.isc6())
            .field("isc7", &self.isc7())
            .field("isc8", &self.isc8())
            .field("isc9", &self.isc9())
            .field("isc10", &self.isc10())
            .field("isc11", &self.isc11())
            .field("isc12", &self.isc12())
            .field("isc13", &self.isc13())
            .field("isc14", &self.isc14())
            .field("isc15", &self.isc15())
            .finish()
    }
}
impl W {
    ///Bit 0 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    #[must_use]
    pub fn isc0(&mut self) -> ISC0_W<C1ICRrs> {
        ISC0_W::new(self, 0)
    }
    ///Bit 1 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    #[must_use]
    pub fn isc1(&mut self) -> ISC1_W<C1ICRrs> {
        ISC1_W::new(self, 1)
    }
    ///Bit 2 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    #[must_use]
    pub fn isc2(&mut self) -> ISC2_W<C1ICRrs> {
        ISC2_W::new(self, 2)
    }
    ///Bit 3 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    #[must_use]
    pub fn isc3(&mut self) -> ISC3_W<C1ICRrs> {
        ISC3_W::new(self, 3)
    }
    ///Bit 4 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    #[must_use]
    pub fn isc4(&mut self) -> ISC4_W<C1ICRrs> {
        ISC4_W::new(self, 4)
    }
    ///Bit 5 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    #[must_use]
    pub fn isc5(&mut self) -> ISC5_W<C1ICRrs> {
        ISC5_W::new(self, 5)
    }
    ///Bit 6 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    #[must_use]
    pub fn isc6(&mut self) -> ISC6_W<C1ICRrs> {
        ISC6_W::new(self, 6)
    }
    ///Bit 7 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    #[must_use]
    pub fn isc7(&mut self) -> ISC7_W<C1ICRrs> {
        ISC7_W::new(self, 7)
    }
    ///Bit 8 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    #[must_use]
    pub fn isc8(&mut self) -> ISC8_W<C1ICRrs> {
        ISC8_W::new(self, 8)
    }
    ///Bit 9 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    #[must_use]
    pub fn isc9(&mut self) -> ISC9_W<C1ICRrs> {
        ISC9_W::new(self, 9)
    }
    ///Bit 10 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    #[must_use]
    pub fn isc10(&mut self) -> ISC10_W<C1ICRrs> {
        ISC10_W::new(self, 10)
    }
    ///Bit 11 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    #[must_use]
    pub fn isc11(&mut self) -> ISC11_W<C1ICRrs> {
        ISC11_W::new(self, 11)
    }
    ///Bit 12 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    #[must_use]
    pub fn isc12(&mut self) -> ISC12_W<C1ICRrs> {
        ISC12_W::new(self, 12)
    }
    ///Bit 13 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    #[must_use]
    pub fn isc13(&mut self) -> ISC13_W<C1ICRrs> {
        ISC13_W::new(self, 13)
    }
    ///Bit 14 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    #[must_use]
    pub fn isc14(&mut self) -> ISC14_W<C1ICRrs> {
        ISC14_W::new(self, 14)
    }
    ///Bit 15 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    #[must_use]
    pub fn isc15(&mut self) -> ISC15_W<C1ICRrs> {
        ISC15_W::new(self, 15)
    }
}
/**HSEM Interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`c1icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#HSEM:C1ICR)*/
pub struct C1ICRrs;
impl crate::RegisterSpec for C1ICRrs {
    type Ux = u32;
}
///`read()` method returns [`c1icr::R`](R) reader structure
impl crate::Readable for C1ICRrs {}
///`write(|w| ..)` method takes [`c1icr::W`](W) writer structure
impl crate::Writable for C1ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets C1ICR to value 0
impl crate::Resettable for C1ICRrs {
    const RESET_VALUE: u32 = 0;
}
