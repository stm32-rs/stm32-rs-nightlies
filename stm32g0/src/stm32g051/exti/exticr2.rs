///Register `EXTICR2` reader
pub type R = crate::R<EXTICR2rs>;
///Register `EXTICR2` writer
pub type W = crate::W<EXTICR2rs>;
/**GPIO port selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI0_7 {
    ///0: GPIO port A selected
    Pa = 0,
    ///1: GPIO port B selected
    Pb = 1,
    ///2: GPIO port C selected
    Pc = 2,
    ///3: GPIO port D selected
    Pd = 3,
    ///5: GPIO port F selected
    Pf = 5,
}
impl From<EXTI0_7> for u8 {
    #[inline(always)]
    fn from(variant: EXTI0_7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI0_7 {
    type Ux = u8;
}
impl crate::IsEnum for EXTI0_7 {}
///Field `EXTI0_7` reader - GPIO port selection
pub type EXTI0_7_R = crate::FieldReader<EXTI0_7>;
impl EXTI0_7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI0_7> {
        match self.bits {
            0 => Some(EXTI0_7::Pa),
            1 => Some(EXTI0_7::Pb),
            2 => Some(EXTI0_7::Pc),
            3 => Some(EXTI0_7::Pd),
            5 => Some(EXTI0_7::Pf),
            _ => None,
        }
    }
    ///GPIO port A selected
    #[inline(always)]
    pub fn is_pa(&self) -> bool {
        *self == EXTI0_7::Pa
    }
    ///GPIO port B selected
    #[inline(always)]
    pub fn is_pb(&self) -> bool {
        *self == EXTI0_7::Pb
    }
    ///GPIO port C selected
    #[inline(always)]
    pub fn is_pc(&self) -> bool {
        *self == EXTI0_7::Pc
    }
    ///GPIO port D selected
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        *self == EXTI0_7::Pd
    }
    ///GPIO port F selected
    #[inline(always)]
    pub fn is_pf(&self) -> bool {
        *self == EXTI0_7::Pf
    }
}
///Field `EXTI0_7` writer - GPIO port selection
pub type EXTI0_7_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI0_7>;
impl<'a, REG> EXTI0_7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///GPIO port A selected
    #[inline(always)]
    pub fn pa(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0_7::Pa)
    }
    ///GPIO port B selected
    #[inline(always)]
    pub fn pb(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0_7::Pb)
    }
    ///GPIO port C selected
    #[inline(always)]
    pub fn pc(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0_7::Pc)
    }
    ///GPIO port D selected
    #[inline(always)]
    pub fn pd(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0_7::Pd)
    }
    ///GPIO port F selected
    #[inline(always)]
    pub fn pf(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0_7::Pf)
    }
}
///Field `EXTI8_15` reader - GPIO port selection
pub use EXTI0_7_R as EXTI8_15_R;
///Field `EXTI16_23` reader - GPIO port selection
pub use EXTI0_7_R as EXTI16_23_R;
///Field `EXTI24_31` reader - GPIO port selection
pub use EXTI0_7_R as EXTI24_31_R;
///Field `EXTI8_15` writer - GPIO port selection
pub use EXTI0_7_W as EXTI8_15_W;
///Field `EXTI16_23` writer - GPIO port selection
pub use EXTI0_7_W as EXTI16_23_W;
///Field `EXTI24_31` writer - GPIO port selection
pub use EXTI0_7_W as EXTI24_31_W;
impl R {
    ///Bits 0:7 - GPIO port selection
    #[inline(always)]
    pub fn exti0_7(&self) -> EXTI0_7_R {
        EXTI0_7_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - GPIO port selection
    #[inline(always)]
    pub fn exti8_15(&self) -> EXTI8_15_R {
        EXTI8_15_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - GPIO port selection
    #[inline(always)]
    pub fn exti16_23(&self) -> EXTI16_23_R {
        EXTI16_23_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - GPIO port selection
    #[inline(always)]
    pub fn exti24_31(&self) -> EXTI24_31_R {
        EXTI24_31_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTICR2")
            .field("exti0_7", &self.exti0_7())
            .field("exti8_15", &self.exti8_15())
            .field("exti16_23", &self.exti16_23())
            .field("exti24_31", &self.exti24_31())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - GPIO port selection
    #[inline(always)]
    pub fn exti0_7(&mut self) -> EXTI0_7_W<EXTICR2rs> {
        EXTI0_7_W::new(self, 0)
    }
    ///Bits 8:15 - GPIO port selection
    #[inline(always)]
    pub fn exti8_15(&mut self) -> EXTI8_15_W<EXTICR2rs> {
        EXTI8_15_W::new(self, 8)
    }
    ///Bits 16:23 - GPIO port selection
    #[inline(always)]
    pub fn exti16_23(&mut self) -> EXTI16_23_W<EXTICR2rs> {
        EXTI16_23_W::new(self, 16)
    }
    ///Bits 24:31 - GPIO port selection
    #[inline(always)]
    pub fn exti24_31(&mut self) -> EXTI24_31_W<EXTICR2rs> {
        EXTI24_31_W::new(self, 24)
    }
}
/**EXTI external interrupt selection register

You can [`read`](crate::Reg::read) this register and get [`exticr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G051.html#EXTI:EXTICR2)*/
pub struct EXTICR2rs;
impl crate::RegisterSpec for EXTICR2rs {
    type Ux = u32;
}
///`read()` method returns [`exticr2::R`](R) reader structure
impl crate::Readable for EXTICR2rs {}
///`write(|w| ..)` method takes [`exticr2::W`](W) writer structure
impl crate::Writable for EXTICR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EXTICR2 to value 0
impl crate::Resettable for EXTICR2rs {
    const RESET_VALUE: u32 = 0;
}
