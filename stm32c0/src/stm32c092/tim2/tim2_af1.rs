///Register `TIM2_AF1` reader
pub type R = crate::R<TIM2_AF1rs>;
///Register `TIM2_AF1` writer
pub type W = crate::W<TIM2_AF1rs>;
/**ETR source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETRSEL {
    ///0: ETR legacy mode
    B0x0 = 0,
    ///3: LSE
    B0x3 = 3,
    ///4: MCO
    B0x4 = 4,
    ///5: MCO2
    B0x5 = 5,
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
///Field `ETRSEL` reader - ETR source selection
pub type ETRSEL_R = crate::FieldReader<ETRSEL>;
impl ETRSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ETRSEL> {
        match self.bits {
            0 => Some(ETRSEL::B0x0),
            3 => Some(ETRSEL::B0x3),
            4 => Some(ETRSEL::B0x4),
            5 => Some(ETRSEL::B0x5),
            _ => None,
        }
    }
    ///ETR legacy mode
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ETRSEL::B0x0
    }
    ///LSE
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == ETRSEL::B0x3
    }
    ///MCO
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == ETRSEL::B0x4
    }
    ///MCO2
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == ETRSEL::B0x5
    }
}
///Field `ETRSEL` writer - ETR source selection
pub type ETRSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, ETRSEL>;
impl<'a, REG> ETRSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///ETR legacy mode
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ETRSEL::B0x0)
    }
    ///LSE
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(ETRSEL::B0x3)
    }
    ///MCO
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(ETRSEL::B0x4)
    }
    ///MCO2
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(ETRSEL::B0x5)
    }
}
impl R {
    ///Bits 14:17 - ETR source selection
    #[inline(always)]
    pub fn etrsel(&self) -> ETRSEL_R {
        ETRSEL_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM2_AF1")
            .field("etrsel", &self.etrsel())
            .finish()
    }
}
impl W {
    ///Bits 14:17 - ETR source selection
    #[inline(always)]
    pub fn etrsel(&mut self) -> ETRSEL_W<'_, TIM2_AF1rs> {
        ETRSEL_W::new(self, 14)
    }
}
/**TIM2 alternate function option register 1

You can [`read`](crate::Reg::read) this register and get [`tim2_af1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_af1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM2:TIM2_AF1)*/
pub struct TIM2_AF1rs;
impl crate::RegisterSpec for TIM2_AF1rs {
    type Ux = u32;
}
///`read()` method returns [`tim2_af1::R`](R) reader structure
impl crate::Readable for TIM2_AF1rs {}
///`write(|w| ..)` method takes [`tim2_af1::W`](W) writer structure
impl crate::Writable for TIM2_AF1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM2_AF1 to value 0
impl crate::Resettable for TIM2_AF1rs {}
