///Register `TIM3_TISEL` reader
pub type R = crate::R<TIM3_TISELrs>;
///Register `TIM3_TISEL` writer
pub type W = crate::W<TIM3_TISELrs>;
/**Field `TI1SEL` reader - TI1\[0\]
to TI1\[15\]
input selection*/
pub type TI1SEL_R = crate::FieldReader;
/**Field `TI1SEL` writer - TI1\[0\]
to TI1\[15\]
input selection*/
pub type TI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
/**Field `TI2SEL` reader - TI2\[0\]
to TI2\[15\]
input selection*/
pub type TI2SEL_R = crate::FieldReader;
/**Field `TI2SEL` writer - TI2\[0\]
to TI2\[15\]
input selection*/
pub type TI2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
/**Field `TI3SEL` reader - TI3\[0\]
to TI3\[15\]
input selection*/
pub type TI3SEL_R = crate::FieldReader;
/**Field `TI3SEL` writer - TI3\[0\]
to TI3\[15\]
input selection*/
pub type TI3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    /**Bits 0:3 - TI1\[0\]
    to TI1\[15\]
    input selection*/
    #[inline(always)]
    pub fn ti1sel(&self) -> TI1SEL_R {
        TI1SEL_R::new((self.bits & 0x0f) as u8)
    }
    /**Bits 8:11 - TI2\[0\]
    to TI2\[15\]
    input selection*/
    #[inline(always)]
    pub fn ti2sel(&self) -> TI2SEL_R {
        TI2SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    /**Bits 16:19 - TI3\[0\]
    to TI3\[15\]
    input selection*/
    #[inline(always)]
    pub fn ti3sel(&self) -> TI3SEL_R {
        TI3SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM3_TISEL")
            .field("ti1sel", &self.ti1sel())
            .field("ti2sel", &self.ti2sel())
            .field("ti3sel", &self.ti3sel())
            .finish()
    }
}
impl W {
    /**Bits 0:3 - TI1\[0\]
    to TI1\[15\]
    input selection*/
    #[inline(always)]
    pub fn ti1sel(&mut self) -> TI1SEL_W<TIM3_TISELrs> {
        TI1SEL_W::new(self, 0)
    }
    /**Bits 8:11 - TI2\[0\]
    to TI2\[15\]
    input selection*/
    #[inline(always)]
    pub fn ti2sel(&mut self) -> TI2SEL_W<TIM3_TISELrs> {
        TI2SEL_W::new(self, 8)
    }
    /**Bits 16:19 - TI3\[0\]
    to TI3\[15\]
    input selection*/
    #[inline(always)]
    pub fn ti3sel(&mut self) -> TI3SEL_W<TIM3_TISELrs> {
        TI3SEL_W::new(self, 16)
    }
}
/**TIM3 timer input selection register

You can [`read`](crate::Reg::read) this register and get [`tim3_tisel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_tisel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#TIM3:TIM3_TISEL)*/
pub struct TIM3_TISELrs;
impl crate::RegisterSpec for TIM3_TISELrs {
    type Ux = u32;
}
///`read()` method returns [`tim3_tisel::R`](R) reader structure
impl crate::Readable for TIM3_TISELrs {}
///`write(|w| ..)` method takes [`tim3_tisel::W`](W) writer structure
impl crate::Writable for TIM3_TISELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TIM3_TISEL to value 0
impl crate::Resettable for TIM3_TISELrs {
    const RESET_VALUE: u32 = 0;
}
