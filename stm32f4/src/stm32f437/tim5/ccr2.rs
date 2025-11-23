///Register `CCR2` reader
pub type R = crate::R<CCR2rs>;
///Register `CCR2` writer
pub type W = crate::W<CCR2rs>;
///Field `CCR2_L` reader - Low Capture/Compare 2 value
pub type CCR2_L_R = crate::FieldReader<u16>;
///Field `CCR2_L` writer - Low Capture/Compare 2 value
pub type CCR2_L_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `CCR2_H` reader - High Capture/Compare 2 value
pub type CCR2_H_R = crate::FieldReader<u16>;
///Field `CCR2_H` writer - High Capture/Compare 2 value
pub type CCR2_H_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Low Capture/Compare 2 value
    #[inline(always)]
    pub fn ccr2_l(&self) -> CCR2_L_R {
        CCR2_L_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - High Capture/Compare 2 value
    #[inline(always)]
    pub fn ccr2_h(&self) -> CCR2_H_R {
        CCR2_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR2")
            .field("ccr2_h", &self.ccr2_h())
            .field("ccr2_l", &self.ccr2_l())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Low Capture/Compare 2 value
    #[inline(always)]
    pub fn ccr2_l(&mut self) -> CCR2_L_W<'_, CCR2rs> {
        CCR2_L_W::new(self, 0)
    }
    ///Bits 16:31 - High Capture/Compare 2 value
    #[inline(always)]
    pub fn ccr2_h(&mut self) -> CCR2_H_W<'_, CCR2rs> {
        CCR2_H_W::new(self, 16)
    }
}
/**capture/compare register 2

You can [`read`](crate::Reg::read) this register and get [`ccr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#TIM5:CCR2)*/
pub struct CCR2rs;
impl crate::RegisterSpec for CCR2rs {
    type Ux = u32;
}
///`read()` method returns [`ccr2::R`](R) reader structure
impl crate::Readable for CCR2rs {}
///`write(|w| ..)` method takes [`ccr2::W`](W) writer structure
impl crate::Writable for CCR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCR2 to value 0
impl crate::Resettable for CCR2rs {}
