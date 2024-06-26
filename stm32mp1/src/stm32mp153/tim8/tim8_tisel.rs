///Register `TIM8_TISEL` reader
pub type R = crate::R<TIM8_TISELrs>;
///Register `TIM8_TISEL` writer
pub type W = crate::W<TIM8_TISELrs>;
///Field `TI1SEL` reader - TI1SEL
pub type TI1SEL_R = crate::FieldReader;
///Field `TI1SEL` writer - TI1SEL
pub type TI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TI2SEL` reader - TI2SEL
pub type TI2SEL_R = crate::FieldReader;
///Field `TI2SEL` writer - TI2SEL
pub type TI2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TI3SEL` reader - TI3SEL
pub type TI3SEL_R = crate::FieldReader;
///Field `TI3SEL` writer - TI3SEL
pub type TI3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TI4SEL` reader - TI4SEL
pub type TI4SEL_R = crate::FieldReader;
///Field `TI4SEL` writer - TI4SEL
pub type TI4SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - TI1SEL
    #[inline(always)]
    pub fn ti1sel(&self) -> TI1SEL_R {
        TI1SEL_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:11 - TI2SEL
    #[inline(always)]
    pub fn ti2sel(&self) -> TI2SEL_R {
        TI2SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:19 - TI3SEL
    #[inline(always)]
    pub fn ti3sel(&self) -> TI3SEL_R {
        TI3SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 24:27 - TI4SEL
    #[inline(always)]
    pub fn ti4sel(&self) -> TI4SEL_R {
        TI4SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM8_TISEL")
            .field("ti1sel", &self.ti1sel())
            .field("ti2sel", &self.ti2sel())
            .field("ti3sel", &self.ti3sel())
            .field("ti4sel", &self.ti4sel())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - TI1SEL
    #[inline(always)]
    #[must_use]
    pub fn ti1sel(&mut self) -> TI1SEL_W<TIM8_TISELrs> {
        TI1SEL_W::new(self, 0)
    }
    ///Bits 8:11 - TI2SEL
    #[inline(always)]
    #[must_use]
    pub fn ti2sel(&mut self) -> TI2SEL_W<TIM8_TISELrs> {
        TI2SEL_W::new(self, 8)
    }
    ///Bits 16:19 - TI3SEL
    #[inline(always)]
    #[must_use]
    pub fn ti3sel(&mut self) -> TI3SEL_W<TIM8_TISELrs> {
        TI3SEL_W::new(self, 16)
    }
    ///Bits 24:27 - TI4SEL
    #[inline(always)]
    #[must_use]
    pub fn ti4sel(&mut self) -> TI4SEL_W<TIM8_TISELrs> {
        TI4SEL_W::new(self, 24)
    }
}
/**TIM8 timer input selection register

You can [`read`](crate::Reg::read) this register and get [`tim8_tisel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim8_tisel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM8:TIM8_TISEL)*/
pub struct TIM8_TISELrs;
impl crate::RegisterSpec for TIM8_TISELrs {
    type Ux = u32;
}
///`read()` method returns [`tim8_tisel::R`](R) reader structure
impl crate::Readable for TIM8_TISELrs {}
///`write(|w| ..)` method takes [`tim8_tisel::W`](W) writer structure
impl crate::Writable for TIM8_TISELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TIM8_TISEL to value 0
impl crate::Resettable for TIM8_TISELrs {
    const RESET_VALUE: u32 = 0;
}
