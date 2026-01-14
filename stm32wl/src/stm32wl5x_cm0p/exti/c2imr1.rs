///Register `C2IMR1` reader
pub type R = crate::R<C2IMR1rs>;
///Register `C2IMR1` writer
pub type W = crate::W<C2IMR1rs>;
/**Wake-up with interrupt mask on event input x For each bit of this field:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum INTERRUPT_MASK {
    ///0: Interrupt request line is masked
    Masked = 0,
    ///1: Interrupt request line is unmasked
    Unmasked = 1,
}
impl From<INTERRUPT_MASK> for u32 {
    #[inline(always)]
    fn from(variant: INTERRUPT_MASK) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INTERRUPT_MASK {
    type Ux = u32;
}
impl crate::IsEnum for INTERRUPT_MASK {}
///Field `IM` reader - Wake-up with interrupt mask on event input x For each bit of this field:
pub type IM_R = crate::FieldReader<INTERRUPT_MASK>;
impl IM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<INTERRUPT_MASK> {
        match self.bits {
            0 => Some(INTERRUPT_MASK::Masked),
            1 => Some(INTERRUPT_MASK::Unmasked),
            _ => None,
        }
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == INTERRUPT_MASK::Masked
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == INTERRUPT_MASK::Unmasked
    }
}
///Field `IM` writer - Wake-up with interrupt mask on event input x For each bit of this field:
pub type IM_W<'a, REG> = crate::FieldWriter<'a, REG, 32, INTERRUPT_MASK>;
impl<'a, REG> IM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(INTERRUPT_MASK::Masked)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(INTERRUPT_MASK::Unmasked)
    }
}
impl R {
    ///Bits 0:31 - Wake-up with interrupt mask on event input x For each bit of this field:
    #[inline(always)]
    pub fn im(&self) -> IM_R {
        IM_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2IMR1").field("im", &self.im()).finish()
    }
}
impl W {
    ///Bits 0:31 - Wake-up with interrupt mask on event input x For each bit of this field:
    #[inline(always)]
    pub fn im(&mut self) -> IM_W<'_, C2IMR1rs> {
        IM_W::new(self, 0)
    }
}
/**EXTI interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`c2imr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2imr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#EXTI:C2IMR1)*/
pub struct C2IMR1rs;
impl crate::RegisterSpec for C2IMR1rs {
    type Ux = u32;
}
///`read()` method returns [`c2imr1::R`](R) reader structure
impl crate::Readable for C2IMR1rs {}
///`write(|w| ..)` method takes [`c2imr1::W`](W) writer structure
impl crate::Writable for C2IMR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2IMR1 to value 0
impl crate::Resettable for C2IMR1rs {}
