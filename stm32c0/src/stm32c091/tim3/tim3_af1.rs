///Register `TIM3_AF1` reader
pub type R = crate::R<TIM3_AF1rs>;
///Register `TIM3_AF1` writer
pub type W = crate::W<TIM3_AF1rs>;
/**ETR source selection These bits select the ETR input source. Others: Reserved

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETRSEL {
    ///0: ETR legacy mode
    B0x0 = 0,
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
///Field `ETRSEL` reader - ETR source selection These bits select the ETR input source. Others: Reserved
pub type ETRSEL_R = crate::FieldReader<ETRSEL>;
impl ETRSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ETRSEL> {
        match self.bits {
            0 => Some(ETRSEL::B0x0),
            _ => None,
        }
    }
    ///ETR legacy mode
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ETRSEL::B0x0
    }
}
///Field `ETRSEL` writer - ETR source selection These bits select the ETR input source. Others: Reserved
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
}
impl R {
    ///Bits 14:17 - ETR source selection These bits select the ETR input source. Others: Reserved
    #[inline(always)]
    pub fn etrsel(&self) -> ETRSEL_R {
        ETRSEL_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM3_AF1")
            .field("etrsel", &self.etrsel())
            .finish()
    }
}
impl W {
    ///Bits 14:17 - ETR source selection These bits select the ETR input source. Others: Reserved
    #[inline(always)]
    pub fn etrsel(&mut self) -> ETRSEL_W<'_, TIM3_AF1rs> {
        ETRSEL_W::new(self, 14)
    }
}
/**TIM3 alternate function option register 1

You can [`read`](crate::Reg::read) this register and get [`tim3_af1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_af1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM3:TIM3_AF1)*/
pub struct TIM3_AF1rs;
impl crate::RegisterSpec for TIM3_AF1rs {
    type Ux = u32;
}
///`read()` method returns [`tim3_af1::R`](R) reader structure
impl crate::Readable for TIM3_AF1rs {}
///`write(|w| ..)` method takes [`tim3_af1::W`](W) writer structure
impl crate::Writable for TIM3_AF1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM3_AF1 to value 0
impl crate::Resettable for TIM3_AF1rs {}
