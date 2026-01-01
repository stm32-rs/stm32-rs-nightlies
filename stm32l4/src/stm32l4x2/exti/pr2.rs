///Register `PR2` reader
pub type R = crate::R<PR2rs>;
///Register `PR2` writer
pub type W = crate::W<PR2rs>;
/**Pending interrupt flag on line 35

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIF35R {
    ///0: No trigger request occurred
    NotPending = 0,
    ///1: Selected trigger request occurred
    Pending = 1,
}
impl From<PIF35R> for bool {
    #[inline(always)]
    fn from(variant: PIF35R) -> Self {
        variant as u8 != 0
    }
}
///Field `PIF35` reader - Pending interrupt flag on line 35
pub type PIF35_R = crate::BitReader<PIF35R>;
impl PIF35_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIF35R {
        match self.bits {
            false => PIF35R::NotPending,
            true => PIF35R::Pending,
        }
    }
    ///No trigger request occurred
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PIF35R::NotPending
    }
    ///Selected trigger request occurred
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PIF35R::Pending
    }
}
/**Pending interrupt flag on line 35

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIF35W {
    ///1: Clears pending bit
    Clear = 1,
}
impl From<PIF35W> for bool {
    #[inline(always)]
    fn from(variant: PIF35W) -> Self {
        variant as u8 != 0
    }
}
///Field `PIF35` writer - Pending interrupt flag on line 35
pub type PIF35_W<'a, REG> = crate::BitWriter1C<'a, REG, PIF35W>;
impl<'a, REG> PIF35_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PIF35W::Clear)
    }
}
///Field `PIF36` reader - Pending interrupt flag on line 36
pub use PIF35_R as PIF36_R;
///Field `PIF37` reader - Pending interrupt flag on line 37
pub use PIF35_R as PIF37_R;
///Field `PIF38` reader - Pending interrupt flag on line 38
pub use PIF35_R as PIF38_R;
///Field `PIF36` writer - Pending interrupt flag on line 36
pub use PIF35_W as PIF36_W;
///Field `PIF37` writer - Pending interrupt flag on line 37
pub use PIF35_W as PIF37_W;
///Field `PIF38` writer - Pending interrupt flag on line 38
pub use PIF35_W as PIF38_W;
impl R {
    ///Bit 3 - Pending interrupt flag on line 35
    #[inline(always)]
    pub fn pif35(&self) -> PIF35_R {
        PIF35_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Pending interrupt flag on line 36
    #[inline(always)]
    pub fn pif36(&self) -> PIF36_R {
        PIF36_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Pending interrupt flag on line 37
    #[inline(always)]
    pub fn pif37(&self) -> PIF37_R {
        PIF37_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Pending interrupt flag on line 38
    #[inline(always)]
    pub fn pif38(&self) -> PIF38_R {
        PIF38_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PR2")
            .field("pif35", &self.pif35())
            .field("pif36", &self.pif36())
            .field("pif37", &self.pif37())
            .field("pif38", &self.pif38())
            .finish()
    }
}
impl W {
    ///Bit 3 - Pending interrupt flag on line 35
    #[inline(always)]
    pub fn pif35(&mut self) -> PIF35_W<'_, PR2rs> {
        PIF35_W::new(self, 3)
    }
    ///Bit 4 - Pending interrupt flag on line 36
    #[inline(always)]
    pub fn pif36(&mut self) -> PIF36_W<'_, PR2rs> {
        PIF36_W::new(self, 4)
    }
    ///Bit 5 - Pending interrupt flag on line 37
    #[inline(always)]
    pub fn pif37(&mut self) -> PIF37_W<'_, PR2rs> {
        PIF37_W::new(self, 5)
    }
    ///Bit 6 - Pending interrupt flag on line 38
    #[inline(always)]
    pub fn pif38(&mut self) -> PIF38_W<'_, PR2rs> {
        PIF38_W::new(self, 6)
    }
}
/**Pending register

You can [`read`](crate::Reg::read) this register and get [`pr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#EXTI:PR2)*/
pub struct PR2rs;
impl crate::RegisterSpec for PR2rs {
    type Ux = u32;
}
///`read()` method returns [`pr2::R`](R) reader structure
impl crate::Readable for PR2rs {}
///`write(|w| ..)` method takes [`pr2::W`](W) writer structure
impl crate::Writable for PR2rs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x78;
}
///`reset()` method sets PR2 to value 0
impl crate::Resettable for PR2rs {}
