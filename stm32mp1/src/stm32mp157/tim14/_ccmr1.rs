///Register `_CCMR1` reader
pub type R = crate::R<_CCMR1rs>;
///Register `_CCMR1` writer
pub type W = crate::W<_CCMR1rs>;
///Field `CC1S` reader - CC1S
pub type CC1S_R = crate::FieldReader;
///Field `CC1S` writer - CC1S
pub type CC1S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OC1FE` reader - OC1FE
pub type OC1FE_R = crate::BitReader;
///Field `OC1FE` writer - OC1FE
pub type OC1FE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC1PE` reader - OC1PE
pub type OC1PE_R = crate::BitReader;
///Field `OC1PE` writer - OC1PE
pub type OC1PE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC1M` reader - OC1M
pub type OC1M_R = crate::FieldReader;
///Field `OC1M` writer - OC1M
pub type OC1M_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `OC1M3` reader - OC1M3
pub type OC1M3_R = crate::BitReader;
///Field `OC1M3` writer - OC1M3
pub type OC1M3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - CC1S
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - OC1FE
    #[inline(always)]
    pub fn oc1fe(&self) -> OC1FE_R {
        OC1FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - OC1PE
    #[inline(always)]
    pub fn oc1pe(&self) -> OC1PE_R {
        OC1PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - OC1M
    #[inline(always)]
    pub fn oc1m(&self) -> OC1M_R {
        OC1M_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 16 - OC1M3
    #[inline(always)]
    pub fn oc1m3(&self) -> OC1M3_R {
        OC1M3_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_CCMR1")
            .field("cc1s", &self.cc1s())
            .field("oc1fe", &self.oc1fe())
            .field("oc1pe", &self.oc1pe())
            .field("oc1m", &self.oc1m())
            .field("oc1m3", &self.oc1m3())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - CC1S
    #[inline(always)]
    pub fn cc1s(&mut self) -> CC1S_W<'_, _CCMR1rs> {
        CC1S_W::new(self, 0)
    }
    ///Bit 2 - OC1FE
    #[inline(always)]
    pub fn oc1fe(&mut self) -> OC1FE_W<'_, _CCMR1rs> {
        OC1FE_W::new(self, 2)
    }
    ///Bit 3 - OC1PE
    #[inline(always)]
    pub fn oc1pe(&mut self) -> OC1PE_W<'_, _CCMR1rs> {
        OC1PE_W::new(self, 3)
    }
    ///Bits 4:6 - OC1M
    #[inline(always)]
    pub fn oc1m(&mut self) -> OC1M_W<'_, _CCMR1rs> {
        OC1M_W::new(self, 4)
    }
    ///Bit 16 - OC1M3
    #[inline(always)]
    pub fn oc1m3(&mut self) -> OC1M3_W<'_, _CCMR1rs> {
        OC1M3_W::new(self, 16)
    }
}
/**The channels can be used in input (capture mode) or in output (compare mode). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function in input and in output mode. For a given bit, OCxx describes its function when the channel is configured in output, ICxx describes its function when the channel is configured in input. So one must take care that the same bit can have a different meaning for the input stage and for the output stage. Output compare mode

You can [`read`](crate::Reg::read) this register and get [`_ccmr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_ccmr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM14:_CCMR1)*/
pub struct _CCMR1rs;
impl crate::RegisterSpec for _CCMR1rs {
    type Ux = u32;
}
///`read()` method returns [`_ccmr1::R`](R) reader structure
impl crate::Readable for _CCMR1rs {}
///`write(|w| ..)` method takes [`_ccmr1::W`](W) writer structure
impl crate::Writable for _CCMR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets _CCMR1 to value 0
impl crate::Resettable for _CCMR1rs {}
