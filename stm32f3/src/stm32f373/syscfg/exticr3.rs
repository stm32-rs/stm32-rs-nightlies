///Register `EXTICR3` reader
pub type R = crate::R<EXTICR3rs>;
///Register `EXTICR3` writer
pub type W = crate::W<EXTICR3rs>;
///Field `EXTI9` reader - EXTI 9 configuration bits
pub use super::exticr1::EXTI0_R as EXTI9_R;
///Field `EXTI10` reader - EXTI 10 configuration bits
pub use super::exticr1::EXTI0_R as EXTI10_R;
///Field `EXTI9` writer - EXTI 9 configuration bits
pub use super::exticr1::EXTI0_W as EXTI9_W;
///Field `EXTI10` writer - EXTI 10 configuration bits
pub use super::exticr1::EXTI0_W as EXTI10_W;
///Field `EXTI8` reader - EXTI 8 configuration bits
pub use super::exticr1::EXTI3_R as EXTI8_R;
///Field `EXTI8` writer - EXTI 8 configuration bits
pub use super::exticr1::EXTI3_W as EXTI8_W;
///EXTI 8 configuration bits
pub use super::exticr1::EXTI_ABCDE;
///EXTI 9 configuration bits
pub use super::exticr1::EXTI_ABCDEF;
/**EXTI 11 configuration bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI_ACDE {
    ///0: Select PAx as the source input for the EXTIx external interrupt
    Pa = 0,
    ///2: Select PCx as the source input for the EXTIx external interrupt
    Pc = 2,
    ///3: Select PDx as the source input for the EXTIx external interrupt
    Pd = 3,
    ///4: Select PEx as the source input for the EXTIx external interrupt
    Pe = 4,
}
impl From<EXTI_ACDE> for u8 {
    #[inline(always)]
    fn from(variant: EXTI_ACDE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI_ACDE {
    type Ux = u8;
}
impl crate::IsEnum for EXTI_ACDE {}
///Field `EXTI11` reader - EXTI 11 configuration bits
pub type EXTI11_R = crate::FieldReader<EXTI_ACDE>;
impl EXTI11_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI_ACDE> {
        match self.bits {
            0 => Some(EXTI_ACDE::Pa),
            2 => Some(EXTI_ACDE::Pc),
            3 => Some(EXTI_ACDE::Pd),
            4 => Some(EXTI_ACDE::Pe),
            _ => None,
        }
    }
    ///Select PAx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn is_pa(&self) -> bool {
        *self == EXTI_ACDE::Pa
    }
    ///Select PCx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn is_pc(&self) -> bool {
        *self == EXTI_ACDE::Pc
    }
    ///Select PDx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        *self == EXTI_ACDE::Pd
    }
    ///Select PEx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn is_pe(&self) -> bool {
        *self == EXTI_ACDE::Pe
    }
}
///Field `EXTI11` writer - EXTI 11 configuration bits
pub type EXTI11_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTI_ACDE>;
impl<'a, REG> EXTI11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Select PAx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn pa(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI_ACDE::Pa)
    }
    ///Select PCx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn pc(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI_ACDE::Pc)
    }
    ///Select PDx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn pd(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI_ACDE::Pd)
    }
    ///Select PEx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn pe(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI_ACDE::Pe)
    }
}
impl R {
    ///Bits 0:3 - EXTI 8 configuration bits
    #[inline(always)]
    pub fn exti8(&self) -> EXTI8_R {
        EXTI8_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - EXTI 9 configuration bits
    #[inline(always)]
    pub fn exti9(&self) -> EXTI9_R {
        EXTI9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - EXTI 10 configuration bits
    #[inline(always)]
    pub fn exti10(&self) -> EXTI10_R {
        EXTI10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - EXTI 11 configuration bits
    #[inline(always)]
    pub fn exti11(&self) -> EXTI11_R {
        EXTI11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTICR3")
            .field("exti11", &self.exti11())
            .field("exti10", &self.exti10())
            .field("exti9", &self.exti9())
            .field("exti8", &self.exti8())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - EXTI 8 configuration bits
    #[inline(always)]
    pub fn exti8(&mut self) -> EXTI8_W<EXTICR3rs> {
        EXTI8_W::new(self, 0)
    }
    ///Bits 4:7 - EXTI 9 configuration bits
    #[inline(always)]
    pub fn exti9(&mut self) -> EXTI9_W<EXTICR3rs> {
        EXTI9_W::new(self, 4)
    }
    ///Bits 8:11 - EXTI 10 configuration bits
    #[inline(always)]
    pub fn exti10(&mut self) -> EXTI10_W<EXTICR3rs> {
        EXTI10_W::new(self, 8)
    }
    ///Bits 12:15 - EXTI 11 configuration bits
    #[inline(always)]
    pub fn exti11(&mut self) -> EXTI11_W<EXTICR3rs> {
        EXTI11_W::new(self, 12)
    }
}
/**external interrupt configuration register 3

You can [`read`](crate::Reg::read) this register and get [`exticr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#SYSCFG:EXTICR3)*/
pub struct EXTICR3rs;
impl crate::RegisterSpec for EXTICR3rs {
    type Ux = u32;
}
///`read()` method returns [`exticr3::R`](R) reader structure
impl crate::Readable for EXTICR3rs {}
///`write(|w| ..)` method takes [`exticr3::W`](W) writer structure
impl crate::Writable for EXTICR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EXTICR3 to value 0
impl crate::Resettable for EXTICR3rs {}
