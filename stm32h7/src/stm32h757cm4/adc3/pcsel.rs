///Register `PCSEL` reader
pub type R = crate::R<PCSELrs>;
///Register `PCSEL` writer
pub type W = crate::W<PCSELrs>;
/**Channel x (VINP\[i\]) pre selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum PCSEL {
    ///0: Input channel x is not pre-selected
    NotPreselected = 0,
    ///1: Pre-select input channel x
    Preselected = 1,
}
impl From<PCSEL> for u32 {
    #[inline(always)]
    fn from(variant: PCSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCSEL {
    type Ux = u32;
}
impl crate::IsEnum for PCSEL {}
///Field `PCSEL` reader - Channel x (VINP\[i\]) pre selection
pub type PCSEL_R = crate::FieldReader<PCSEL>;
impl PCSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PCSEL> {
        match self.bits {
            0 => Some(PCSEL::NotPreselected),
            1 => Some(PCSEL::Preselected),
            _ => None,
        }
    }
    ///Input channel x is not pre-selected
    #[inline(always)]
    pub fn is_not_preselected(&self) -> bool {
        *self == PCSEL::NotPreselected
    }
    ///Pre-select input channel x
    #[inline(always)]
    pub fn is_preselected(&self) -> bool {
        *self == PCSEL::Preselected
    }
}
///Field `PCSEL` writer - Channel x (VINP\[i\]) pre selection
pub type PCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 20, PCSEL>;
impl<'a, REG> PCSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    ///Input channel x is not pre-selected
    #[inline(always)]
    pub fn not_preselected(self) -> &'a mut crate::W<REG> {
        self.variant(PCSEL::NotPreselected)
    }
    ///Pre-select input channel x
    #[inline(always)]
    pub fn preselected(self) -> &'a mut crate::W<REG> {
        self.variant(PCSEL::Preselected)
    }
}
impl R {
    ///Bits 0:19 - Channel x (VINP\[i\]) pre selection
    #[inline(always)]
    pub fn pcsel(&self) -> PCSEL_R {
        PCSEL_R::new(self.bits & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCSEL")
            .field("pcsel", &self.pcsel())
            .finish()
    }
}
impl W {
    ///Bits 0:19 - Channel x (VINP\[i\]) pre selection
    #[inline(always)]
    pub fn pcsel(&mut self) -> PCSEL_W<'_, PCSELrs> {
        PCSEL_W::new(self, 0)
    }
}
/**ADC pre channel selection register

You can [`read`](crate::Reg::read) this register and get [`pcsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#ADC3:PCSEL)*/
pub struct PCSELrs;
impl crate::RegisterSpec for PCSELrs {
    type Ux = u32;
}
///`read()` method returns [`pcsel::R`](R) reader structure
impl crate::Readable for PCSELrs {}
///`write(|w| ..)` method takes [`pcsel::W`](W) writer structure
impl crate::Writable for PCSELrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCSEL to value 0
impl crate::Resettable for PCSELrs {}
