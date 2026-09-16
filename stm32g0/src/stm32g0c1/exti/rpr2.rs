///Register `RPR2` reader
pub type R = crate::R<RPR2rs>;
///Register `RPR2` writer
pub type W = crate::W<RPR2rs>;
/**Rising edge event pending for configurable line 34

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF2R {
    ///0: No trigger request occurred
    NotPending = 0,
    ///1: Selected trigger request occurred
    Pending = 1,
}
impl From<RPIF2R> for bool {
    #[inline(always)]
    fn from(variant: RPIF2R) -> Self {
        variant as u8 != 0
    }
}
///Field `RPIF2` reader - Rising edge event pending for configurable line 34
pub type RPIF2_R = crate::BitReader<RPIF2R>;
impl RPIF2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RPIF2R {
        match self.bits {
            false => RPIF2R::NotPending,
            true => RPIF2R::Pending,
        }
    }
    ///No trigger request occurred
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == RPIF2R::NotPending
    }
    ///Selected trigger request occurred
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RPIF2R::Pending
    }
}
/**Rising edge event pending for configurable line 34

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF2W {
    ///1: Clears pending bit
    Clear = 1,
}
impl From<RPIF2W> for bool {
    #[inline(always)]
    fn from(variant: RPIF2W) -> Self {
        variant as u8 != 0
    }
}
///Field `RPIF2` writer - Rising edge event pending for configurable line 34
pub type RPIF2_W<'a, REG> = crate::BitWriter1C<'a, REG, RPIF2W>;
impl<'a, REG> RPIF2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF2W::Clear)
    }
}
impl R {
    ///Bit 2 - Rising edge event pending for configurable line 34
    #[inline(always)]
    pub fn rpif2(&self) -> RPIF2_R {
        RPIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RPR2")
            .field("rpif2", &self.rpif2())
            .finish()
    }
}
impl W {
    ///Bit 2 - Rising edge event pending for configurable line 34
    #[inline(always)]
    pub fn rpif2(&mut self) -> RPIF2_W<'_, RPR2rs> {
        RPIF2_W::new(self, 2)
    }
}
/**EXTI rising edge pending register 2

You can [`read`](crate::Reg::read) this register and get [`rpr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0C1.html#EXTI:RPR2)*/
pub struct RPR2rs;
impl crate::RegisterSpec for RPR2rs {
    type Ux = u32;
}
///`read()` method returns [`rpr2::R`](R) reader structure
impl crate::Readable for RPR2rs {}
///`write(|w| ..)` method takes [`rpr2::W`](W) writer structure
impl crate::Writable for RPR2rs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x04;
}
///`reset()` method sets RPR2 to value 0
impl crate::Resettable for RPR2rs {}
