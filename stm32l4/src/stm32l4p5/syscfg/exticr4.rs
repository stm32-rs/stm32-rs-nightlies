///Register `EXTICR4` reader
pub type R = crate::R<EXTICR4rs>;
///Register `EXTICR4` writer
pub type W = crate::W<EXTICR4rs>;
/**EXTI12 configuration bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI_ABCDEFGH {
    ///0: Select PAx as the source input for the EXTIx external interrupt
    Pa = 0,
    ///1: Select PBx as the source input for the EXTIx external interrupt
    Pb = 1,
    ///2: Select PCx as the source input for the EXTIx external interrupt
    Pc = 2,
    ///3: Select PDx as the source input for the EXTIx external interrupt
    Pd = 3,
    ///4: Select PEx as the source input for the EXTIx external interrupt
    Pe = 4,
    ///5: Select PFx as the source input for the EXTIx external interrupt
    Pf = 5,
    ///6: Select PGx as the source input for the EXTIx external interrupt
    Pg = 6,
    ///7: Select PHx as the source input for the EXTIx external interrupt
    Ph = 7,
}
impl From<EXTI_ABCDEFGH> for u8 {
    #[inline(always)]
    fn from(variant: EXTI_ABCDEFGH) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI_ABCDEFGH {
    type Ux = u8;
}
impl crate::IsEnum for EXTI_ABCDEFGH {}
///Field `EXTI12` reader - EXTI12 configuration bits
pub type EXTI12_R = crate::FieldReader<EXTI_ABCDEFGH>;
impl EXTI12_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI_ABCDEFGH> {
        match self.bits {
            0 => Some(EXTI_ABCDEFGH::Pa),
            1 => Some(EXTI_ABCDEFGH::Pb),
            2 => Some(EXTI_ABCDEFGH::Pc),
            3 => Some(EXTI_ABCDEFGH::Pd),
            4 => Some(EXTI_ABCDEFGH::Pe),
            5 => Some(EXTI_ABCDEFGH::Pf),
            6 => Some(EXTI_ABCDEFGH::Pg),
            7 => Some(EXTI_ABCDEFGH::Ph),
            _ => None,
        }
    }
    ///Select PAx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn is_pa(&self) -> bool {
        *self == EXTI_ABCDEFGH::Pa
    }
    ///Select PBx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn is_pb(&self) -> bool {
        *self == EXTI_ABCDEFGH::Pb
    }
    ///Select PCx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn is_pc(&self) -> bool {
        *self == EXTI_ABCDEFGH::Pc
    }
    ///Select PDx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        *self == EXTI_ABCDEFGH::Pd
    }
    ///Select PEx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn is_pe(&self) -> bool {
        *self == EXTI_ABCDEFGH::Pe
    }
    ///Select PFx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn is_pf(&self) -> bool {
        *self == EXTI_ABCDEFGH::Pf
    }
    ///Select PGx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn is_pg(&self) -> bool {
        *self == EXTI_ABCDEFGH::Pg
    }
    ///Select PHx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn is_ph(&self) -> bool {
        *self == EXTI_ABCDEFGH::Ph
    }
}
///Field `EXTI12` writer - EXTI12 configuration bits
pub type EXTI12_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTI_ABCDEFGH>;
impl<'a, REG> EXTI12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Select PAx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn pa(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI_ABCDEFGH::Pa)
    }
    ///Select PBx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn pb(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI_ABCDEFGH::Pb)
    }
    ///Select PCx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn pc(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI_ABCDEFGH::Pc)
    }
    ///Select PDx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn pd(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI_ABCDEFGH::Pd)
    }
    ///Select PEx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn pe(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI_ABCDEFGH::Pe)
    }
    ///Select PFx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn pf(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI_ABCDEFGH::Pf)
    }
    ///Select PGx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn pg(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI_ABCDEFGH::Pg)
    }
    ///Select PHx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn ph(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI_ABCDEFGH::Ph)
    }
}
///Field `EXTI13` reader - EXTI13 configuration bits
pub use EXTI12_R as EXTI13_R;
///Field `EXTI14` reader - EXTI14 configuration bits
pub use EXTI12_R as EXTI14_R;
///Field `EXTI15` reader - EXTI15 configuration bits
pub use EXTI12_R as EXTI15_R;
///Field `EXTI13` writer - EXTI13 configuration bits
pub use EXTI12_W as EXTI13_W;
///Field `EXTI14` writer - EXTI14 configuration bits
pub use EXTI12_W as EXTI14_W;
///Field `EXTI15` writer - EXTI15 configuration bits
pub use EXTI12_W as EXTI15_W;
impl R {
    ///Bits 0:3 - EXTI12 configuration bits
    #[inline(always)]
    pub fn exti12(&self) -> EXTI12_R {
        EXTI12_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - EXTI13 configuration bits
    #[inline(always)]
    pub fn exti13(&self) -> EXTI13_R {
        EXTI13_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - EXTI14 configuration bits
    #[inline(always)]
    pub fn exti14(&self) -> EXTI14_R {
        EXTI14_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - EXTI15 configuration bits
    #[inline(always)]
    pub fn exti15(&self) -> EXTI15_R {
        EXTI15_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTICR4")
            .field("exti15", &self.exti15())
            .field("exti14", &self.exti14())
            .field("exti13", &self.exti13())
            .field("exti12", &self.exti12())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - EXTI12 configuration bits
    #[inline(always)]
    pub fn exti12(&mut self) -> EXTI12_W<'_, EXTICR4rs> {
        EXTI12_W::new(self, 0)
    }
    ///Bits 4:7 - EXTI13 configuration bits
    #[inline(always)]
    pub fn exti13(&mut self) -> EXTI13_W<'_, EXTICR4rs> {
        EXTI13_W::new(self, 4)
    }
    ///Bits 8:11 - EXTI14 configuration bits
    #[inline(always)]
    pub fn exti14(&mut self) -> EXTI14_W<'_, EXTICR4rs> {
        EXTI14_W::new(self, 8)
    }
    ///Bits 12:15 - EXTI15 configuration bits
    #[inline(always)]
    pub fn exti15(&mut self) -> EXTI15_W<'_, EXTICR4rs> {
        EXTI15_W::new(self, 12)
    }
}
/**external interrupt configuration register 4

You can [`read`](crate::Reg::read) this register and get [`exticr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#SYSCFG:EXTICR4)*/
pub struct EXTICR4rs;
impl crate::RegisterSpec for EXTICR4rs {
    type Ux = u32;
}
///`read()` method returns [`exticr4::R`](R) reader structure
impl crate::Readable for EXTICR4rs {}
///`write(|w| ..)` method takes [`exticr4::W`](W) writer structure
impl crate::Writable for EXTICR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EXTICR4 to value 0
impl crate::Resettable for EXTICR4rs {}
