///Register `CCMR1_INPUT` reader
pub type R = crate::R<CCMR1_INPUTrs>;
///Register `CCMR1_INPUT` writer
pub type W = crate::W<CCMR1_INPUTrs>;
///Field `CC1S` reader - Capture/Compare 1 selection
pub type CC1S_R = crate::FieldReader;
///Field `CC1S` writer - Capture/Compare 1 selection
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
    ///Bits 0:1 - Capture/Compare 1 selection
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
        f.debug_struct("CCMR1_INPUT")
            .field("cc1s", &self.cc1s())
            .field("ic1psc", &self.ic1psc())
            .field("ic1f", &self.ic1f())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Capture/Compare 1 selection
    #[inline(always)]
    pub fn cc1s(&mut self) -> CC1S_W<'_, CCMR1_INPUTrs> {
        CC1S_W::new(self, 0)
    }
    ///Bits 2:3 - Input capture 1 prescaler
    #[inline(always)]
    pub fn ic1psc(&mut self) -> IC1PSC_W<'_, CCMR1_INPUTrs> {
        IC1PSC_W::new(self, 2)
    }
    ///Bits 4:7 - Input capture 1 filter
    #[inline(always)]
    pub fn ic1f(&mut self) -> IC1F_W<'_, CCMR1_INPUTrs> {
        IC1F_W::new(self, 4)
    }
}
/**TIM17 capture/compare mode register 1

You can [`read`](crate::Reg::read) this register and get [`ccmr1_input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#TIM17:CCMR1_INPUT)*/
pub struct CCMR1_INPUTrs;
impl crate::RegisterSpec for CCMR1_INPUTrs {
    type Ux = u32;
}
///`read()` method returns [`ccmr1_input::R`](R) reader structure
impl crate::Readable for CCMR1_INPUTrs {}
///`write(|w| ..)` method takes [`ccmr1_input::W`](W) writer structure
impl crate::Writable for CCMR1_INPUTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCMR1_INPUT to value 0
impl crate::Resettable for CCMR1_INPUTrs {}
