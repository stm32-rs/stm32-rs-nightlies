///Register `_CCMR1_input` reader
pub type R = crate::R<_CCMR1_INPUTrs>;
///Register `_CCMR1_input` writer
pub type W = crate::W<_CCMR1_INPUTrs>;
///Field `CC1S` reader - CC1S
pub type CC1S_R = crate::FieldReader;
///Field `CC1S` writer - CC1S
pub type CC1S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC1PSC` reader - IC1PSC
pub type IC1PSC_R = crate::FieldReader;
///Field `IC1PSC` writer - IC1PSC
pub type IC1PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC1F` reader - IC1F
pub type IC1F_R = crate::FieldReader;
///Field `IC1F` writer - IC1F
pub type IC1F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CC2S` reader - CC2S
pub type CC2S_R = crate::FieldReader;
///Field `CC2S` writer - CC2S
pub type CC2S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC2PSC` reader - IC2PSC
pub type IC2PSC_R = crate::FieldReader;
///Field `IC2PSC` writer - IC2PSC
pub type IC2PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC2F` reader - IC2F
pub type IC2F_R = crate::FieldReader;
///Field `IC2F` writer - IC2F
pub type IC2F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:1 - CC1S
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - IC1PSC
    #[inline(always)]
    pub fn ic1psc(&self) -> IC1PSC_R {
        IC1PSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:7 - IC1F
    #[inline(always)]
    pub fn ic1f(&self) -> IC1F_R {
        IC1F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:9 - CC2S
    #[inline(always)]
    pub fn cc2s(&self) -> CC2S_R {
        CC2S_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - IC2PSC
    #[inline(always)]
    pub fn ic2psc(&self) -> IC2PSC_R {
        IC2PSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:15 - IC2F
    #[inline(always)]
    pub fn ic2f(&self) -> IC2F_R {
        IC2F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_CCMR1_input")
            .field("cc1s", &self.cc1s())
            .field("ic1psc", &self.ic1psc())
            .field("ic1f", &self.ic1f())
            .field("cc2s", &self.cc2s())
            .field("ic2psc", &self.ic2psc())
            .field("ic2f", &self.ic2f())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - CC1S
    #[inline(always)]
    pub fn cc1s(&mut self) -> CC1S_W<'_, _CCMR1_INPUTrs> {
        CC1S_W::new(self, 0)
    }
    ///Bits 2:3 - IC1PSC
    #[inline(always)]
    pub fn ic1psc(&mut self) -> IC1PSC_W<'_, _CCMR1_INPUTrs> {
        IC1PSC_W::new(self, 2)
    }
    ///Bits 4:7 - IC1F
    #[inline(always)]
    pub fn ic1f(&mut self) -> IC1F_W<'_, _CCMR1_INPUTrs> {
        IC1F_W::new(self, 4)
    }
    ///Bits 8:9 - CC2S
    #[inline(always)]
    pub fn cc2s(&mut self) -> CC2S_W<'_, _CCMR1_INPUTrs> {
        CC2S_W::new(self, 8)
    }
    ///Bits 10:11 - IC2PSC
    #[inline(always)]
    pub fn ic2psc(&mut self) -> IC2PSC_W<'_, _CCMR1_INPUTrs> {
        IC2PSC_W::new(self, 10)
    }
    ///Bits 12:15 - IC2F
    #[inline(always)]
    pub fn ic2f(&mut self) -> IC2F_W<'_, _CCMR1_INPUTrs> {
        IC2F_W::new(self, 12)
    }
}
/**TIM12 capture/compare mode register 1

You can [`read`](crate::Reg::read) this register and get [`_ccmr1_input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_ccmr1_input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM12:_CCMR1_input)*/
pub struct _CCMR1_INPUTrs;
impl crate::RegisterSpec for _CCMR1_INPUTrs {
    type Ux = u32;
}
///`read()` method returns [`_ccmr1_input::R`](R) reader structure
impl crate::Readable for _CCMR1_INPUTrs {}
///`write(|w| ..)` method takes [`_ccmr1_input::W`](W) writer structure
impl crate::Writable for _CCMR1_INPUTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets _CCMR1_input to value 0
impl crate::Resettable for _CCMR1_INPUTrs {}
