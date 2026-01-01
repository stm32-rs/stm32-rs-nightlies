///Register `CCR1` reader
pub type R = crate::R<CCR1rs>;
///Register `CCR1` writer
pub type W = crate::W<CCR1rs>;
///Field `CCR1_L` reader - Low Capture/Compare 1 value
pub type CCR1_L_R = crate::FieldReader<u16>;
///Field `CCR1_L` writer - Low Capture/Compare 1 value
pub type CCR1_L_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `CCR1_H` reader - High Capture/Compare 1 value
pub type CCR1_H_R = crate::FieldReader<u16>;
///Field `CCR1_H` writer - High Capture/Compare 1 value
pub type CCR1_H_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Low Capture/Compare 1 value
    #[inline(always)]
    pub fn ccr1_l(&self) -> CCR1_L_R {
        CCR1_L_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - High Capture/Compare 1 value
    #[inline(always)]
    pub fn ccr1_h(&self) -> CCR1_H_R {
        CCR1_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR1")
            .field("ccr1_h", &self.ccr1_h())
            .field("ccr1_l", &self.ccr1_l())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Low Capture/Compare 1 value
    #[inline(always)]
    pub fn ccr1_l(&mut self) -> CCR1_L_W<'_, CCR1rs> {
        CCR1_L_W::new(self, 0)
    }
    ///Bits 16:31 - High Capture/Compare 1 value
    #[inline(always)]
    pub fn ccr1_h(&mut self) -> CCR1_H_W<'_, CCR1rs> {
        CCR1_H_W::new(self, 16)
    }
}
/**capture/compare register 1

You can [`read`](crate::Reg::read) this register and get [`ccr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#TIM3:CCR1)*/
pub struct CCR1rs;
impl crate::RegisterSpec for CCR1rs {
    type Ux = u32;
}
///`read()` method returns [`ccr1::R`](R) reader structure
impl crate::Readable for CCR1rs {}
///`write(|w| ..)` method takes [`ccr1::W`](W) writer structure
impl crate::Writable for CCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCR1 to value 0
impl crate::Resettable for CCR1rs {}
