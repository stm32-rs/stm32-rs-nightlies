///Register `CCR` reader
pub type R = crate::R<CCRrs>;
///Register `CCR` writer
pub type W = crate::W<CCRrs>;
///Field `OTRIM1` reader - DAC Channel 1 offset trimming value
pub type OTRIM1_R = crate::FieldReader;
///Field `OTRIM1` writer - DAC Channel 1 offset trimming value
pub type OTRIM1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `OTRIM2` reader - DAC Channel 2 offset trimming value
pub type OTRIM2_R = crate::FieldReader;
///Field `OTRIM2` writer - DAC Channel 2 offset trimming value
pub type OTRIM2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - DAC Channel 1 offset trimming value
    #[inline(always)]
    pub fn otrim1(&self) -> OTRIM1_R {
        OTRIM1_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 16:20 - DAC Channel 2 offset trimming value
    #[inline(always)]
    pub fn otrim2(&self) -> OTRIM2_R {
        OTRIM2_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("otrim1", &self.otrim1())
            .field("otrim2", &self.otrim2())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - DAC Channel 1 offset trimming value
    #[inline(always)]
    pub fn otrim1(&mut self) -> OTRIM1_W<CCRrs> {
        OTRIM1_W::new(self, 0)
    }
    ///Bits 16:20 - DAC Channel 2 offset trimming value
    #[inline(always)]
    pub fn otrim2(&mut self) -> OTRIM2_W<CCRrs> {
        OTRIM2_W::new(self, 16)
    }
}
/**DAC calibration control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#DAC:CCR)*/
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
///`read()` method returns [`ccr::R`](R) reader structure
impl crate::Readable for CCRrs {}
///`write(|w| ..)` method takes [`ccr::W`](W) writer structure
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCR to value 0
impl crate::Resettable for CCRrs {}
