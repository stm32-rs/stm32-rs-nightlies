///Register `APB3ENR` reader
pub type R = crate::R<APB3ENRrs>;
///Register `APB3ENR` writer
pub type W = crate::W<APB3ENRrs>;
///Field `DFTEN` reader - DFT enable
pub type DFTEN_R = crate::BitReader;
///Field `DFTEN` writer - DFT enable
pub type DFTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - DFT enable
    #[inline(always)]
    pub fn dften(&self) -> DFTEN_R {
        DFTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB3ENR")
            .field("dften", &self.dften())
            .finish()
    }
}
impl W {
    ///Bit 2 - DFT enable
    #[inline(always)]
    pub fn dften(&mut self) -> DFTEN_W<'_, APB3ENRrs> {
        DFTEN_W::new(self, 2)
    }
}
/**RCC APB3 enable register

You can [`read`](crate::Reg::read) this register and get [`apb3enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:APB3ENR)*/
pub struct APB3ENRrs;
impl crate::RegisterSpec for APB3ENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb3enr::R`](R) reader structure
impl crate::Readable for APB3ENRrs {}
///`write(|w| ..)` method takes [`apb3enr::W`](W) writer structure
impl crate::Writable for APB3ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB3ENR to value 0
impl crate::Resettable for APB3ENRrs {}
