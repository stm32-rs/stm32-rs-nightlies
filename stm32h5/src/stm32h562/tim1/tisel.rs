///Register `TISEL` reader
pub type R = crate::R<TISELrs>;
///Register `TISEL` writer
pub type W = crate::W<TISELrs>;
/**Selects tim_ti1\[0..15\] input ... Refer to Section 65.3.2: TIM1 pins and internal signals for interconnects list.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TI1SEL {
    ///0: TIM1_CHx input selected
    Selected = 0,
}
impl From<TI1SEL> for u8 {
    #[inline(always)]
    fn from(variant: TI1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TI1SEL {
    type Ux = u8;
}
impl crate::IsEnum for TI1SEL {}
///Field `TI1SEL` reader - Selects tim_ti1\[0..15\] input ... Refer to Section 65.3.2: TIM1 pins and internal signals for interconnects list.
pub type TI1SEL_R = crate::FieldReader<TI1SEL>;
impl TI1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TI1SEL> {
        match self.bits {
            0 => Some(TI1SEL::Selected),
            _ => None,
        }
    }
    ///TIM1_CHx input selected
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == TI1SEL::Selected
    }
}
///Field `TI1SEL` writer - Selects tim_ti1\[0..15\] input ... Refer to Section 65.3.2: TIM1 pins and internal signals for interconnects list.
pub type TI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TI1SEL>;
impl<'a, REG> TI1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///TIM1_CHx input selected
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(TI1SEL::Selected)
    }
}
///Field `TI2SEL` reader - Selects tim_ti2\[0..15\] input ... Refer to Section 65.3.2: TIM1 pins and internal signals for interconnects list.
pub use TI1SEL_R as TI2SEL_R;
///Field `TI3SEL` reader - Selects tim_ti3\[0..15\] input ... Refer to Section 65.3.2: TIM1 pins and internal signals for interconnects list.
pub use TI1SEL_R as TI3SEL_R;
///Field `TI4SEL` reader - Selects tim_ti4\[0..15\] input ... Refer to Section 65.3.2: TIM1 pins and internal signals for interconnects list.
pub use TI1SEL_R as TI4SEL_R;
///Field `TI2SEL` writer - Selects tim_ti2\[0..15\] input ... Refer to Section 65.3.2: TIM1 pins and internal signals for interconnects list.
pub use TI1SEL_W as TI2SEL_W;
///Field `TI3SEL` writer - Selects tim_ti3\[0..15\] input ... Refer to Section 65.3.2: TIM1 pins and internal signals for interconnects list.
pub use TI1SEL_W as TI3SEL_W;
///Field `TI4SEL` writer - Selects tim_ti4\[0..15\] input ... Refer to Section 65.3.2: TIM1 pins and internal signals for interconnects list.
pub use TI1SEL_W as TI4SEL_W;
impl R {
    ///Bits 0:3 - Selects tim_ti1\[0..15\] input ... Refer to Section 65.3.2: TIM1 pins and internal signals for interconnects list.
    #[inline(always)]
    pub fn ti1sel(&self) -> TI1SEL_R {
        TI1SEL_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:11 - Selects tim_ti2\[0..15\] input ... Refer to Section 65.3.2: TIM1 pins and internal signals for interconnects list.
    #[inline(always)]
    pub fn ti2sel(&self) -> TI2SEL_R {
        TI2SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:19 - Selects tim_ti3\[0..15\] input ... Refer to Section 65.3.2: TIM1 pins and internal signals for interconnects list.
    #[inline(always)]
    pub fn ti3sel(&self) -> TI3SEL_R {
        TI3SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 24:27 - Selects tim_ti4\[0..15\] input ... Refer to Section 65.3.2: TIM1 pins and internal signals for interconnects list.
    #[inline(always)]
    pub fn ti4sel(&self) -> TI4SEL_R {
        TI4SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TISEL")
            .field("ti1sel", &self.ti1sel())
            .field("ti2sel", &self.ti2sel())
            .field("ti3sel", &self.ti3sel())
            .field("ti4sel", &self.ti4sel())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Selects tim_ti1\[0..15\] input ... Refer to Section 65.3.2: TIM1 pins and internal signals for interconnects list.
    #[inline(always)]
    pub fn ti1sel(&mut self) -> TI1SEL_W<'_, TISELrs> {
        TI1SEL_W::new(self, 0)
    }
    ///Bits 8:11 - Selects tim_ti2\[0..15\] input ... Refer to Section 65.3.2: TIM1 pins and internal signals for interconnects list.
    #[inline(always)]
    pub fn ti2sel(&mut self) -> TI2SEL_W<'_, TISELrs> {
        TI2SEL_W::new(self, 8)
    }
    ///Bits 16:19 - Selects tim_ti3\[0..15\] input ... Refer to Section 65.3.2: TIM1 pins and internal signals for interconnects list.
    #[inline(always)]
    pub fn ti3sel(&mut self) -> TI3SEL_W<'_, TISELrs> {
        TI3SEL_W::new(self, 16)
    }
    ///Bits 24:27 - Selects tim_ti4\[0..15\] input ... Refer to Section 65.3.2: TIM1 pins and internal signals for interconnects list.
    #[inline(always)]
    pub fn ti4sel(&mut self) -> TI4SEL_W<'_, TISELrs> {
        TI4SEL_W::new(self, 24)
    }
}
/**TIM1 timer input selection register

You can [`read`](crate::Reg::read) this register and get [`tisel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tisel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#TIM1:TISEL)*/
pub struct TISELrs;
impl crate::RegisterSpec for TISELrs {
    type Ux = u32;
}
///`read()` method returns [`tisel::R`](R) reader structure
impl crate::Readable for TISELrs {}
///`write(|w| ..)` method takes [`tisel::W`](W) writer structure
impl crate::Writable for TISELrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TISEL to value 0
impl crate::Resettable for TISELrs {}
