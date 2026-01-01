///Register `RF1R` reader
pub type R = crate::R<RF1Rrs>;
///Register `RF1R` writer
pub type W = crate::W<RF1Rrs>;
///Field `FMP1` reader - FMP1
pub type FMP1_R = crate::FieldReader;
///Field `FULL1` reader - FULL1
pub type FULL1_R = crate::BitReader;
///Field `FULL1` writer - FULL1
pub type FULL1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FOVR1` reader - FOVR1
pub type FOVR1_R = crate::BitReader;
///Field `FOVR1` writer - FOVR1
pub type FOVR1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFOM1` reader - RFOM1
pub type RFOM1_R = crate::BitReader;
///Field `RFOM1` writer - RFOM1
pub type RFOM1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - FMP1
    #[inline(always)]
    pub fn fmp1(&self) -> FMP1_R {
        FMP1_R::new((self.bits & 3) as u8)
    }
    ///Bit 3 - FULL1
    #[inline(always)]
    pub fn full1(&self) -> FULL1_R {
        FULL1_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - FOVR1
    #[inline(always)]
    pub fn fovr1(&self) -> FOVR1_R {
        FOVR1_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RFOM1
    #[inline(always)]
    pub fn rfom1(&self) -> RFOM1_R {
        RFOM1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RF1R")
            .field("rfom1", &self.rfom1())
            .field("fovr1", &self.fovr1())
            .field("full1", &self.full1())
            .field("fmp1", &self.fmp1())
            .finish()
    }
}
impl W {
    ///Bit 3 - FULL1
    #[inline(always)]
    pub fn full1(&mut self) -> FULL1_W<'_, RF1Rrs> {
        FULL1_W::new(self, 3)
    }
    ///Bit 4 - FOVR1
    #[inline(always)]
    pub fn fovr1(&mut self) -> FOVR1_W<'_, RF1Rrs> {
        FOVR1_W::new(self, 4)
    }
    ///Bit 5 - RFOM1
    #[inline(always)]
    pub fn rfom1(&mut self) -> RFOM1_W<'_, RF1Rrs> {
        RFOM1_W::new(self, 5)
    }
}
/**receive FIFO 1 register

You can [`read`](crate::Reg::read) this register and get [`rf1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#CAN1:RF1R)*/
pub struct RF1Rrs;
impl crate::RegisterSpec for RF1Rrs {
    type Ux = u32;
}
///`read()` method returns [`rf1r::R`](R) reader structure
impl crate::Readable for RF1Rrs {}
///`write(|w| ..)` method takes [`rf1r::W`](W) writer structure
impl crate::Writable for RF1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RF1R to value 0
impl crate::Resettable for RF1Rrs {}
