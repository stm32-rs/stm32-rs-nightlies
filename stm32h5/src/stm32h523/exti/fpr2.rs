///Register `FPR2` reader
pub type R = crate::R<FPR2rs>;
///Register `FPR2` writer
pub type W = crate::W<FPR2rs>;
/**configurable event inputs x falling edge pending bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF46R {
    ///0: No trigger request occurred
    NotPending = 0,
    ///1: Selected trigger request occurred
    Pending = 1,
}
impl From<FPIF46R> for bool {
    #[inline(always)]
    fn from(variant: FPIF46R) -> Self {
        variant as u8 != 0
    }
}
///Field `FPIF46` reader - configurable event inputs x falling edge pending bit
pub type FPIF46_R = crate::BitReader<FPIF46R>;
impl FPIF46_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FPIF46R {
        match self.bits {
            false => FPIF46R::NotPending,
            true => FPIF46R::Pending,
        }
    }
    ///No trigger request occurred
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == FPIF46R::NotPending
    }
    ///Selected trigger request occurred
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == FPIF46R::Pending
    }
}
/**configurable event inputs x falling edge pending bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF46W {
    ///1: Clears pending bit
    Clear = 1,
}
impl From<FPIF46W> for bool {
    #[inline(always)]
    fn from(variant: FPIF46W) -> Self {
        variant as u8 != 0
    }
}
///Field `FPIF46` writer - configurable event inputs x falling edge pending bit
pub type FPIF46_W<'a, REG> = crate::BitWriter1C<'a, REG, FPIF46W>;
impl<'a, REG> FPIF46_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF46W::Clear)
    }
}
///Field `FPIF50` reader - configurable event inputs x falling edge pending bit
pub use FPIF46_R as FPIF50_R;
///Field `FPIF53` reader - configurable event inputs x falling edge pending bit
pub use FPIF46_R as FPIF53_R;
///Field `FPIF50` writer - configurable event inputs x falling edge pending bit
pub use FPIF46_W as FPIF50_W;
///Field `FPIF53` writer - configurable event inputs x falling edge pending bit
pub use FPIF46_W as FPIF53_W;
impl R {
    ///Bit 14 - configurable event inputs x falling edge pending bit
    #[inline(always)]
    pub fn fpif46(&self) -> FPIF46_R {
        FPIF46_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 18 - configurable event inputs x falling edge pending bit
    #[inline(always)]
    pub fn fpif50(&self) -> FPIF50_R {
        FPIF50_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 21 - configurable event inputs x falling edge pending bit
    #[inline(always)]
    pub fn fpif53(&self) -> FPIF53_R {
        FPIF53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPR2")
            .field("fpif46", &self.fpif46())
            .field("fpif50", &self.fpif50())
            .field("fpif53", &self.fpif53())
            .finish()
    }
}
impl W {
    ///Bit 14 - configurable event inputs x falling edge pending bit
    #[inline(always)]
    pub fn fpif46(&mut self) -> FPIF46_W<'_, FPR2rs> {
        FPIF46_W::new(self, 14)
    }
    ///Bit 18 - configurable event inputs x falling edge pending bit
    #[inline(always)]
    pub fn fpif50(&mut self) -> FPIF50_W<'_, FPR2rs> {
        FPIF50_W::new(self, 18)
    }
    ///Bit 21 - configurable event inputs x falling edge pending bit
    #[inline(always)]
    pub fn fpif53(&mut self) -> FPIF53_W<'_, FPR2rs> {
        FPIF53_W::new(self, 21)
    }
}
/**EXTI falling edge pending register 2

You can [`read`](crate::Reg::read) this register and get [`fpr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#EXTI:FPR2)*/
pub struct FPR2rs;
impl crate::RegisterSpec for FPR2rs {
    type Ux = u32;
}
///`read()` method returns [`fpr2::R`](R) reader structure
impl crate::Readable for FPR2rs {}
///`write(|w| ..)` method takes [`fpr2::W`](W) writer structure
impl crate::Writable for FPR2rs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0024_4000;
}
///`reset()` method sets FPR2 to value 0
impl crate::Resettable for FPR2rs {}
