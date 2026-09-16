///Register `APB3RSTR` reader
pub type R = crate::R<APB3RSTRrs>;
///Register `APB3RSTR` writer
pub type W = crate::W<APB3RSTRrs>;
/**LTDC block reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LTDCRST {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<LTDCRST> for bool {
    #[inline(always)]
    fn from(variant: LTDCRST) -> Self {
        variant as u8 != 0
    }
}
///Field `LTDCRST` reader - LTDC block reset
pub type LTDCRST_R = crate::BitReader<LTDCRST>;
impl LTDCRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<LTDCRST> {
        match self.bits {
            true => Some(LTDCRST::Reset),
            _ => None,
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == LTDCRST::Reset
    }
}
///Field `LTDCRST` writer - LTDC block reset
pub type LTDCRST_W<'a, REG> = crate::BitWriter<'a, REG, LTDCRST>;
impl<'a, REG> LTDCRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(LTDCRST::Reset)
    }
}
impl R {
    ///Bit 3 - LTDC block reset
    #[inline(always)]
    pub fn ltdcrst(&self) -> LTDCRST_R {
        LTDCRST_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB3RSTR")
            .field("ltdcrst", &self.ltdcrst())
            .finish()
    }
}
impl W {
    ///Bit 3 - LTDC block reset
    #[inline(always)]
    pub fn ltdcrst(&mut self) -> LTDCRST_W<'_, APB3RSTRrs> {
        LTDCRST_W::new(self, 3)
    }
}
/**RCC APB3 Peripheral Reset Register

You can [`read`](crate::Reg::read) this register and get [`apb3rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743V.html#RCC:APB3RSTR)*/
pub struct APB3RSTRrs;
impl crate::RegisterSpec for APB3RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`apb3rstr::R`](R) reader structure
impl crate::Readable for APB3RSTRrs {}
///`write(|w| ..)` method takes [`apb3rstr::W`](W) writer structure
impl crate::Writable for APB3RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB3RSTR to value 0
impl crate::Resettable for APB3RSTRrs {}
