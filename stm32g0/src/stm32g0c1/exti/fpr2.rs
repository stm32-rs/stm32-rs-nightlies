///Register `FPR2` reader
pub type R = crate::R<FPR2rs>;
///Register `FPR2` writer
pub type W = crate::W<FPR2rs>;
/**Falling edge event pending for configurable line 34

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF2R {
    ///0: No trigger request occurred
    NotPending = 0,
    ///1: Selected trigger request occurred
    Pending = 1,
}
impl From<FPIF2R> for bool {
    #[inline(always)]
    fn from(variant: FPIF2R) -> Self {
        variant as u8 != 0
    }
}
///Field `FPIF2` reader - Falling edge event pending for configurable line 34
pub type FPIF2_R = crate::BitReader<FPIF2R>;
impl FPIF2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FPIF2R {
        match self.bits {
            false => FPIF2R::NotPending,
            true => FPIF2R::Pending,
        }
    }
    ///No trigger request occurred
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == FPIF2R::NotPending
    }
    ///Selected trigger request occurred
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == FPIF2R::Pending
    }
}
/**Falling edge event pending for configurable line 34

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF2W {
    ///1: Clears pending bit
    Clear = 1,
}
impl From<FPIF2W> for bool {
    #[inline(always)]
    fn from(variant: FPIF2W) -> Self {
        variant as u8 != 0
    }
}
///Field `FPIF2` writer - Falling edge event pending for configurable line 34
pub type FPIF2_W<'a, REG> = crate::BitWriter1C<'a, REG, FPIF2W>;
impl<'a, REG> FPIF2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF2W::Clear)
    }
}
impl R {
    ///Bit 2 - Falling edge event pending for configurable line 34
    #[inline(always)]
    pub fn fpif2(&self) -> FPIF2_R {
        FPIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPR2")
            .field("fpif2", &self.fpif2())
            .finish()
    }
}
impl W {
    ///Bit 2 - Falling edge event pending for configurable line 34
    #[inline(always)]
    pub fn fpif2(&mut self) -> FPIF2_W<'_, FPR2rs> {
        FPIF2_W::new(self, 2)
    }
}
/**EXTI falling edge pending register 2

You can [`read`](crate::Reg::read) this register and get [`fpr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0C1.html#EXTI:FPR2)*/
pub struct FPR2rs;
impl crate::RegisterSpec for FPR2rs {
    type Ux = u32;
}
///`read()` method returns [`fpr2::R`](R) reader structure
impl crate::Readable for FPR2rs {}
///`write(|w| ..)` method takes [`fpr2::W`](W) writer structure
impl crate::Writable for FPR2rs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x04;
}
///`reset()` method sets FPR2 to value 0
impl crate::Resettable for FPR2rs {}
