///Register `ARR` reader
pub type R = crate::R<ARRrs>;
///Register `ARR` writer
pub type W = crate::W<ARRrs>;
///Field `ARR_L` reader - Low Auto-reload value
pub type ARR_L_R = crate::FieldReader<u16>;
///Field `ARR_L` writer - Low Auto-reload value
pub type ARR_L_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `ARR_H` reader - High Auto-reload value
pub type ARR_H_R = crate::FieldReader<u16>;
///Field `ARR_H` writer - High Auto-reload value
pub type ARR_H_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Low Auto-reload value
    #[inline(always)]
    pub fn arr_l(&self) -> ARR_L_R {
        ARR_L_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - High Auto-reload value
    #[inline(always)]
    pub fn arr_h(&self) -> ARR_H_R {
        ARR_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ARR")
            .field("arr_h", &self.arr_h())
            .field("arr_l", &self.arr_l())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Low Auto-reload value
    #[inline(always)]
    pub fn arr_l(&mut self) -> ARR_L_W<'_, ARRrs> {
        ARR_L_W::new(self, 0)
    }
    ///Bits 16:31 - High Auto-reload value
    #[inline(always)]
    pub fn arr_h(&mut self) -> ARR_H_W<'_, ARRrs> {
        ARR_H_W::new(self, 16)
    }
}
/**auto-reload register

You can [`read`](crate::Reg::read) this register and get [`arr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#TIM3:ARR)*/
pub struct ARRrs;
impl crate::RegisterSpec for ARRrs {
    type Ux = u32;
}
///`read()` method returns [`arr::R`](R) reader structure
impl crate::Readable for ARRrs {}
///`write(|w| ..)` method takes [`arr::W`](W) writer structure
impl crate::Writable for ARRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ARR to value 0
impl crate::Resettable for ARRrs {}
