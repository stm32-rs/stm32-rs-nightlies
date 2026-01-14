///Register `CCMR2ALTERNATE24` reader
pub type R = crate::R<CCMR2ALTERNATE24rs>;
///Register `CCMR2ALTERNATE24` writer
pub type W = crate::W<CCMR2ALTERNATE24rs>;
///Field `CC3S` reader - CC3S
pub type CC3S_R = crate::FieldReader;
///Field `CC3S` writer - CC3S
pub type CC3S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC3PSC` reader - IC3PSC
pub type IC3PSC_R = crate::FieldReader;
///Field `IC3PSC` writer - IC3PSC
pub type IC3PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC3F` reader - IC3F
pub type IC3F_R = crate::FieldReader;
///Field `IC3F` writer - IC3F
pub type IC3F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CC4S` reader - CC4S
pub type CC4S_R = crate::FieldReader;
///Field `CC4S` writer - CC4S
pub type CC4S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC4PSC` reader - IC4PSC
pub type IC4PSC_R = crate::FieldReader;
///Field `IC4PSC` writer - IC4PSC
pub type IC4PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC4F` reader - IC4F
pub type IC4F_R = crate::FieldReader;
///Field `IC4F` writer - IC4F
pub type IC4F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:1 - CC3S
    #[inline(always)]
    pub fn cc3s(&self) -> CC3S_R {
        CC3S_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - IC3PSC
    #[inline(always)]
    pub fn ic3psc(&self) -> IC3PSC_R {
        IC3PSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:7 - IC3F
    #[inline(always)]
    pub fn ic3f(&self) -> IC3F_R {
        IC3F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:9 - CC4S
    #[inline(always)]
    pub fn cc4s(&self) -> CC4S_R {
        CC4S_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - IC4PSC
    #[inline(always)]
    pub fn ic4psc(&self) -> IC4PSC_R {
        IC4PSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:15 - IC4F
    #[inline(always)]
    pub fn ic4f(&self) -> IC4F_R {
        IC4F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCMR2ALTERNATE24")
            .field("cc3s", &self.cc3s())
            .field("ic3psc", &self.ic3psc())
            .field("ic3f", &self.ic3f())
            .field("cc4s", &self.cc4s())
            .field("ic4psc", &self.ic4psc())
            .field("ic4f", &self.ic4f())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - CC3S
    #[inline(always)]
    pub fn cc3s(&mut self) -> CC3S_W<'_, CCMR2ALTERNATE24rs> {
        CC3S_W::new(self, 0)
    }
    ///Bits 2:3 - IC3PSC
    #[inline(always)]
    pub fn ic3psc(&mut self) -> IC3PSC_W<'_, CCMR2ALTERNATE24rs> {
        IC3PSC_W::new(self, 2)
    }
    ///Bits 4:7 - IC3F
    #[inline(always)]
    pub fn ic3f(&mut self) -> IC3F_W<'_, CCMR2ALTERNATE24rs> {
        IC3F_W::new(self, 4)
    }
    ///Bits 8:9 - CC4S
    #[inline(always)]
    pub fn cc4s(&mut self) -> CC4S_W<'_, CCMR2ALTERNATE24rs> {
        CC4S_W::new(self, 8)
    }
    ///Bits 10:11 - IC4PSC
    #[inline(always)]
    pub fn ic4psc(&mut self) -> IC4PSC_W<'_, CCMR2ALTERNATE24rs> {
        IC4PSC_W::new(self, 10)
    }
    ///Bits 12:15 - IC4F
    #[inline(always)]
    pub fn ic4f(&mut self) -> IC4F_W<'_, CCMR2ALTERNATE24rs> {
        IC4F_W::new(self, 12)
    }
}
/**The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:

You can [`read`](crate::Reg::read) this register and get [`ccmr2alternate24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2alternate24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM8:CCMR2ALTERNATE24)*/
pub struct CCMR2ALTERNATE24rs;
impl crate::RegisterSpec for CCMR2ALTERNATE24rs {
    type Ux = u32;
}
///`read()` method returns [`ccmr2alternate24::R`](R) reader structure
impl crate::Readable for CCMR2ALTERNATE24rs {}
///`write(|w| ..)` method takes [`ccmr2alternate24::W`](W) writer structure
impl crate::Writable for CCMR2ALTERNATE24rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCMR2ALTERNATE24 to value 0
impl crate::Resettable for CCMR2ALTERNATE24rs {}
