///Register `EXTICR2` reader
pub type R = crate::R<EXTICR2rs>;
///Register `EXTICR2` writer
pub type W = crate::W<EXTICR2rs>;
///Field `EXTI4` reader - EXTI 4 configuration bits
pub use super::exticr1::EXTI0_R as EXTI4_R;
///Field `EXTI5` reader - EXTI 5 configuration bits
pub use super::exticr1::EXTI0_R as EXTI5_R;
///Field `EXTI6` reader - EXTI 6 configuration bits
pub use super::exticr1::EXTI0_R as EXTI6_R;
///Field `EXTI4` writer - EXTI 4 configuration bits
pub use super::exticr1::EXTI0_W as EXTI4_W;
///Field `EXTI5` writer - EXTI 5 configuration bits
pub use super::exticr1::EXTI0_W as EXTI5_W;
///Field `EXTI6` writer - EXTI 6 configuration bits
pub use super::exticr1::EXTI0_W as EXTI6_W;
///EXTI 4 configuration bits
pub use super::exticr1::EXTI_ABC;
/**EXTI 7 configuration bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI_AB {
    ///0: Select PAx as the source input for the EXTIx external interrupt
    Pa = 0,
    ///1: Select PBx as the source input for the EXTIx external interrupt
    Pb = 1,
}
impl From<EXTI_AB> for u8 {
    #[inline(always)]
    fn from(variant: EXTI_AB) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI_AB {
    type Ux = u8;
}
impl crate::IsEnum for EXTI_AB {}
///Field `EXTI7` reader - EXTI 7 configuration bits
pub type EXTI7_R = crate::FieldReader<EXTI_AB>;
impl EXTI7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI_AB> {
        match self.bits {
            0 => Some(EXTI_AB::Pa),
            1 => Some(EXTI_AB::Pb),
            _ => None,
        }
    }
    ///Select PAx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn is_pa(&self) -> bool {
        *self == EXTI_AB::Pa
    }
    ///Select PBx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn is_pb(&self) -> bool {
        *self == EXTI_AB::Pb
    }
}
///Field `EXTI7` writer - EXTI 7 configuration bits
pub type EXTI7_W<'a, REG> = crate::FieldWriter<'a, REG, 3, EXTI_AB>;
impl<'a, REG> EXTI7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Select PAx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn pa(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI_AB::Pa)
    }
    ///Select PBx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn pb(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI_AB::Pb)
    }
}
impl R {
    ///Bits 0:2 - EXTI 4 configuration bits
    #[inline(always)]
    pub fn exti4(&self) -> EXTI4_R {
        EXTI4_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - EXTI 5 configuration bits
    #[inline(always)]
    pub fn exti5(&self) -> EXTI5_R {
        EXTI5_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - EXTI 6 configuration bits
    #[inline(always)]
    pub fn exti6(&self) -> EXTI6_R {
        EXTI6_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:14 - EXTI 7 configuration bits
    #[inline(always)]
    pub fn exti7(&self) -> EXTI7_R {
        EXTI7_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTICR2")
            .field("exti7", &self.exti7())
            .field("exti6", &self.exti6())
            .field("exti5", &self.exti5())
            .field("exti4", &self.exti4())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - EXTI 4 configuration bits
    #[inline(always)]
    pub fn exti4(&mut self) -> EXTI4_W<'_, EXTICR2rs> {
        EXTI4_W::new(self, 0)
    }
    ///Bits 4:6 - EXTI 5 configuration bits
    #[inline(always)]
    pub fn exti5(&mut self) -> EXTI5_W<'_, EXTICR2rs> {
        EXTI5_W::new(self, 4)
    }
    ///Bits 8:10 - EXTI 6 configuration bits
    #[inline(always)]
    pub fn exti6(&mut self) -> EXTI6_W<'_, EXTICR2rs> {
        EXTI6_W::new(self, 8)
    }
    ///Bits 12:14 - EXTI 7 configuration bits
    #[inline(always)]
    pub fn exti7(&mut self) -> EXTI7_W<'_, EXTICR2rs> {
        EXTI7_W::new(self, 12)
    }
}
/**external interrupt configuration register 2

You can [`read`](crate::Reg::read) this register and get [`exticr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#SYSCFG:EXTICR2)*/
pub struct EXTICR2rs;
impl crate::RegisterSpec for EXTICR2rs {
    type Ux = u32;
}
///`read()` method returns [`exticr2::R`](R) reader structure
impl crate::Readable for EXTICR2rs {}
///`write(|w| ..)` method takes [`exticr2::W`](W) writer structure
impl crate::Writable for EXTICR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EXTICR2 to value 0
impl crate::Resettable for EXTICR2rs {}
