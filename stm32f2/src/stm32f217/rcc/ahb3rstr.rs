///Register `AHB3RSTR` reader
pub type R = crate::R<AHB3RSTRrs>;
///Register `AHB3RSTR` writer
pub type W = crate::W<AHB3RSTRrs>;
/**Flexible static memory controller module reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSMCRST {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<FSMCRST> for bool {
    #[inline(always)]
    fn from(variant: FSMCRST) -> Self {
        variant as u8 != 0
    }
}
///Field `FSMCRST` reader - Flexible static memory controller module reset
pub type FSMCRST_R = crate::BitReader<FSMCRST>;
impl FSMCRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<FSMCRST> {
        match self.bits {
            true => Some(FSMCRST::Reset),
            _ => None,
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == FSMCRST::Reset
    }
}
///Field `FSMCRST` writer - Flexible static memory controller module reset
pub type FSMCRST_W<'a, REG> = crate::BitWriter<'a, REG, FSMCRST>;
impl<'a, REG> FSMCRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(FSMCRST::Reset)
    }
}
impl R {
    ///Bit 0 - Flexible static memory controller module reset
    #[inline(always)]
    pub fn fsmcrst(&self) -> FSMCRST_R {
        FSMCRST_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB3RSTR")
            .field("fsmcrst", &self.fsmcrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - Flexible static memory controller module reset
    #[inline(always)]
    pub fn fsmcrst(&mut self) -> FSMCRST_W<'_, AHB3RSTRrs> {
        FSMCRST_W::new(self, 0)
    }
}
/**AHB3 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb3rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F217.html#RCC:AHB3RSTR)*/
pub struct AHB3RSTRrs;
impl crate::RegisterSpec for AHB3RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb3rstr::R`](R) reader structure
impl crate::Readable for AHB3RSTRrs {}
///`write(|w| ..)` method takes [`ahb3rstr::W`](W) writer structure
impl crate::Writable for AHB3RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB3RSTR to value 0
impl crate::Resettable for AHB3RSTRrs {}
