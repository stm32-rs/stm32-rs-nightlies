///Register `AF1` reader
pub type R = crate::R<AF1rs>;
///Register `AF1` writer
pub type W = crate::W<AF1rs>;
/**etr_in source selection These bits select the etr_in input source. ... Refer to for product specific implementation.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETRSEL {
    ///0: ETR legacy mode
    Legacy = 0,
    ///1: COMP1 output
    Comp1 = 1,
    ///2: COMP2 output
    Comp2 = 2,
}
impl From<ETRSEL> for u8 {
    #[inline(always)]
    fn from(variant: ETRSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ETRSEL {
    type Ux = u8;
}
impl crate::IsEnum for ETRSEL {}
///Field `ETRSEL` reader - etr_in source selection These bits select the etr_in input source. ... Refer to for product specific implementation.
pub type ETRSEL_R = crate::FieldReader<ETRSEL>;
impl ETRSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ETRSEL> {
        match self.bits {
            0 => Some(ETRSEL::Legacy),
            1 => Some(ETRSEL::Comp1),
            2 => Some(ETRSEL::Comp2),
            _ => None,
        }
    }
    ///ETR legacy mode
    #[inline(always)]
    pub fn is_legacy(&self) -> bool {
        *self == ETRSEL::Legacy
    }
    ///COMP1 output
    #[inline(always)]
    pub fn is_comp1(&self) -> bool {
        *self == ETRSEL::Comp1
    }
    ///COMP2 output
    #[inline(always)]
    pub fn is_comp2(&self) -> bool {
        *self == ETRSEL::Comp2
    }
}
///Field `ETRSEL` writer - etr_in source selection These bits select the etr_in input source. ... Refer to for product specific implementation.
pub type ETRSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, ETRSEL>;
impl<'a, REG> ETRSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///ETR legacy mode
    #[inline(always)]
    pub fn legacy(self) -> &'a mut crate::W<REG> {
        self.variant(ETRSEL::Legacy)
    }
    ///COMP1 output
    #[inline(always)]
    pub fn comp1(self) -> &'a mut crate::W<REG> {
        self.variant(ETRSEL::Comp1)
    }
    ///COMP2 output
    #[inline(always)]
    pub fn comp2(self) -> &'a mut crate::W<REG> {
        self.variant(ETRSEL::Comp2)
    }
}
impl R {
    ///Bits 14:17 - etr_in source selection These bits select the etr_in input source. ... Refer to for product specific implementation.
    #[inline(always)]
    pub fn etrsel(&self) -> ETRSEL_R {
        ETRSEL_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AF1")
            .field("etrsel", &self.etrsel())
            .finish()
    }
}
impl W {
    ///Bits 14:17 - etr_in source selection These bits select the etr_in input source. ... Refer to for product specific implementation.
    #[inline(always)]
    pub fn etrsel(&mut self) -> ETRSEL_W<'_, AF1rs> {
        ETRSEL_W::new(self, 14)
    }
}
/**TIM4 alternate function register 1

You can [`read`](crate::Reg::read) this register and get [`af1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#TIM4:AF1)*/
pub struct AF1rs;
impl crate::RegisterSpec for AF1rs {
    type Ux = u32;
}
///`read()` method returns [`af1::R`](R) reader structure
impl crate::Readable for AF1rs {}
///`write(|w| ..)` method takes [`af1::W`](W) writer structure
impl crate::Writable for AF1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AF1 to value 0
impl crate::Resettable for AF1rs {}
