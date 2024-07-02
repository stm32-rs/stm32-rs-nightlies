///Register `CNT` reader
pub type R = crate::R<CNTrs>;
///Register `CNT` writer
pub type W = crate::W<CNTrs>;
///Field `CNT_L` reader - Least significant part of counter value
pub type CNT_L_R = crate::FieldReader<u16>;
///Field `CNT_L` writer - Least significant part of counter value
pub type CNT_L_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `CNT_H` reader - Most significant part counter value (on TIM2 and TIM5)
pub type CNT_H_R = crate::FieldReader<u16>;
///Field `CNT_H` writer - Most significant part counter value (on TIM2 and TIM5)
pub type CNT_H_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
///Field `CNT_bit31` reader - Most significant bit of counter value (on TIM2 and TIM5)
pub type CNT_BIT31_R = crate::BitReader;
///Field `CNT_bit31` writer - Most significant bit of counter value (on TIM2 and TIM5)
pub type CNT_BIT31_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - Least significant part of counter value
    #[inline(always)]
    pub fn cnt_l(&self) -> CNT_L_R {
        CNT_L_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:30 - Most significant part counter value (on TIM2 and TIM5)
    #[inline(always)]
    pub fn cnt_h(&self) -> CNT_H_R {
        CNT_H_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    ///Bit 31 - Most significant bit of counter value (on TIM2 and TIM5)
    #[inline(always)]
    pub fn cnt_bit31(&self) -> CNT_BIT31_R {
        CNT_BIT31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNT")
            .field("cnt_h", &self.cnt_h())
            .field("cnt_l", &self.cnt_l())
            .field("cnt_bit31", &self.cnt_bit31())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Least significant part of counter value
    #[inline(always)]
    #[must_use]
    pub fn cnt_l(&mut self) -> CNT_L_W<CNTrs> {
        CNT_L_W::new(self, 0)
    }
    ///Bits 16:30 - Most significant part counter value (on TIM2 and TIM5)
    #[inline(always)]
    #[must_use]
    pub fn cnt_h(&mut self) -> CNT_H_W<CNTrs> {
        CNT_H_W::new(self, 16)
    }
    ///Bit 31 - Most significant bit of counter value (on TIM2 and TIM5)
    #[inline(always)]
    #[must_use]
    pub fn cnt_bit31(&mut self) -> CNT_BIT31_W<CNTrs> {
        CNT_BIT31_W::new(self, 31)
    }
}
/**counter

You can [`read`](crate::Reg::read) this register and get [`cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#TIM4:CNT)*/
pub struct CNTrs;
impl crate::RegisterSpec for CNTrs {
    type Ux = u32;
}
///`read()` method returns [`cnt::R`](R) reader structure
impl crate::Readable for CNTrs {}
///`write(|w| ..)` method takes [`cnt::W`](W) writer structure
impl crate::Writable for CNTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CNT to value 0
impl crate::Resettable for CNTrs {
    const RESET_VALUE: u32 = 0;
}
