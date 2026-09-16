///Register `TISEL` reader
pub type R = crate::R<TISELrs>;
///Register `TISEL` writer
pub type W = crate::W<TISELrs>;
/**TISEL

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TISEL {
    ///0: TIM1_CH1 input selected
    Selected = 0,
}
impl From<TISEL> for u8 {
    #[inline(always)]
    fn from(variant: TISEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TISEL {
    type Ux = u8;
}
impl crate::IsEnum for TISEL {}
///Field `TISEL` reader - TISEL
pub type TISEL_R = crate::FieldReader<TISEL>;
impl TISEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TISEL> {
        match self.bits {
            0 => Some(TISEL::Selected),
            _ => None,
        }
    }
    ///TIM1_CH1 input selected
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == TISEL::Selected
    }
}
///Field `TISEL` writer - TISEL
pub type TISEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TISEL>;
impl<'a, REG> TISEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///TIM1_CH1 input selected
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(TISEL::Selected)
    }
}
impl R {
    ///Bits 0:3 - TISEL
    #[inline(always)]
    pub fn tisel(&self) -> TISEL_R {
        TISEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TISEL")
            .field("tisel", &self.tisel())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - TISEL
    #[inline(always)]
    pub fn tisel(&mut self) -> TISEL_W<'_, TISELrs> {
        TISEL_W::new(self, 0)
    }
}
/**TIM16 input selection register

You can [`read`](crate::Reg::read) this register and get [`tisel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tisel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#TIM16:TISEL)*/
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
