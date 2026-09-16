///Register `CCR4` reader
pub type R = crate::R<CCR4rs>;
///Register `CCR4` writer
pub type W = crate::W<CCR4rs>;
///Field `CCR4_L` reader - Low Capture/Compare value
pub type CCR4_L_R = crate::FieldReader<u16>;
///Field `CCR4_L` writer - Low Capture/Compare value
pub type CCR4_L_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `CCR4_H` reader - High Capture/Compare value
pub type CCR4_H_R = crate::FieldReader<u16>;
///Field `CCR4_H` writer - High Capture/Compare value
pub type CCR4_H_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Low Capture/Compare value
    #[inline(always)]
    pub fn ccr4_l(&self) -> CCR4_L_R {
        CCR4_L_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - High Capture/Compare value
    #[inline(always)]
    pub fn ccr4_h(&self) -> CCR4_H_R {
        CCR4_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR4")
            .field("ccr4_h", &self.ccr4_h())
            .field("ccr4_l", &self.ccr4_l())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Low Capture/Compare value
    #[inline(always)]
    pub fn ccr4_l(&mut self) -> CCR4_L_W<'_, CCR4rs> {
        CCR4_L_W::new(self, 0)
    }
    ///Bits 16:31 - High Capture/Compare value
    #[inline(always)]
    pub fn ccr4_h(&mut self) -> CCR4_H_W<'_, CCR4rs> {
        CCR4_H_W::new(self, 16)
    }
}
/**capture/compare register 4

You can [`read`](crate::Reg::read) this register and get [`ccr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#TIM3:CCR4)*/
pub struct CCR4rs;
impl crate::RegisterSpec for CCR4rs {
    type Ux = u32;
}
///`read()` method returns [`ccr4::R`](R) reader structure
impl crate::Readable for CCR4rs {}
///`write(|w| ..)` method takes [`ccr4::W`](W) writer structure
impl crate::Writable for CCR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCR4 to value 0
impl crate::Resettable for CCR4rs {}
