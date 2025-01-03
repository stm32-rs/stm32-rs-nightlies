///Register `TIM16_CCMR1` reader
pub type R = crate::R<TIM16_CCMR1rs>;
///Register `TIM16_CCMR1` writer
pub type W = crate::W<TIM16_CCMR1rs>;
///Field `CC1S` reader - Capture/Compare 1 Selection
pub type CC1S_R = crate::FieldReader;
///Field `CC1S` writer - Capture/Compare 1 Selection
pub type CC1S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC1PSC` reader - Input capture 1 prescaler
pub type IC1PSC_R = crate::FieldReader;
///Field `IC1PSC` writer - Input capture 1 prescaler
pub type IC1PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC1F` reader - Input capture 1 filter
pub type IC1F_R = crate::FieldReader;
///Field `IC1F` writer - Input capture 1 filter
pub type IC1F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:1 - Capture/Compare 1 Selection
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Input capture 1 prescaler
    #[inline(always)]
    pub fn ic1psc(&self) -> IC1PSC_R {
        IC1PSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:7 - Input capture 1 filter
    #[inline(always)]
    pub fn ic1f(&self) -> IC1F_R {
        IC1F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM16_CCMR1")
            .field("cc1s", &self.cc1s())
            .field("ic1psc", &self.ic1psc())
            .field("ic1f", &self.ic1f())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Capture/Compare 1 Selection
    #[inline(always)]
    pub fn cc1s(&mut self) -> CC1S_W<TIM16_CCMR1rs> {
        CC1S_W::new(self, 0)
    }
    ///Bits 2:3 - Input capture 1 prescaler
    #[inline(always)]
    pub fn ic1psc(&mut self) -> IC1PSC_W<TIM16_CCMR1rs> {
        IC1PSC_W::new(self, 2)
    }
    ///Bits 4:7 - Input capture 1 filter
    #[inline(always)]
    pub fn ic1f(&mut self) -> IC1F_W<TIM16_CCMR1rs> {
        IC1F_W::new(self, 4)
    }
}
/**TIM16 capture/compare mode register 1

You can [`read`](crate::Reg::read) this register and get [`tim16_ccmr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim16_ccmr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#TIM16:TIM16_CCMR1)*/
pub struct TIM16_CCMR1rs;
impl crate::RegisterSpec for TIM16_CCMR1rs {
    type Ux = u32;
}
///`read()` method returns [`tim16_ccmr1::R`](R) reader structure
impl crate::Readable for TIM16_CCMR1rs {}
///`write(|w| ..)` method takes [`tim16_ccmr1::W`](W) writer structure
impl crate::Writable for TIM16_CCMR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TIM16_CCMR1 to value 0
impl crate::Resettable for TIM16_CCMR1rs {
    const RESET_VALUE: u32 = 0;
}
