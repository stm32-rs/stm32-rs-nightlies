///Register `RF0R` reader
pub type R = crate::R<RF0Rrs>;
///Register `RF0R` writer
pub type W = crate::W<RF0Rrs>;
///Field `FMP0` reader - FMP0
pub type FMP0_R = crate::FieldReader;
///Field `FULL0` reader - FULL0
pub type FULL0_R = crate::BitReader;
///Field `FULL0` writer - FULL0
pub type FULL0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FOVR0` reader - FOVR0
pub type FOVR0_R = crate::BitReader;
///Field `FOVR0` writer - FOVR0
pub type FOVR0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFOM0` reader - RFOM0
pub type RFOM0_R = crate::BitReader;
///Field `RFOM0` writer - RFOM0
pub type RFOM0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - FMP0
    #[inline(always)]
    pub fn fmp0(&self) -> FMP0_R {
        FMP0_R::new((self.bits & 3) as u8)
    }
    ///Bit 3 - FULL0
    #[inline(always)]
    pub fn full0(&self) -> FULL0_R {
        FULL0_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - FOVR0
    #[inline(always)]
    pub fn fovr0(&self) -> FOVR0_R {
        FOVR0_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RFOM0
    #[inline(always)]
    pub fn rfom0(&self) -> RFOM0_R {
        RFOM0_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RF0R")
            .field("rfom0", &self.rfom0())
            .field("fovr0", &self.fovr0())
            .field("full0", &self.full0())
            .field("fmp0", &self.fmp0())
            .finish()
    }
}
impl W {
    ///Bit 3 - FULL0
    #[inline(always)]
    pub fn full0(&mut self) -> FULL0_W<'_, RF0Rrs> {
        FULL0_W::new(self, 3)
    }
    ///Bit 4 - FOVR0
    #[inline(always)]
    pub fn fovr0(&mut self) -> FOVR0_W<'_, RF0Rrs> {
        FOVR0_W::new(self, 4)
    }
    ///Bit 5 - RFOM0
    #[inline(always)]
    pub fn rfom0(&mut self) -> RFOM0_W<'_, RF0Rrs> {
        RFOM0_W::new(self, 5)
    }
}
/**receive FIFO 0 register

You can [`read`](crate::Reg::read) this register and get [`rf0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#CAN1:RF0R)*/
pub struct RF0Rrs;
impl crate::RegisterSpec for RF0Rrs {
    type Ux = u32;
}
///`read()` method returns [`rf0r::R`](R) reader structure
impl crate::Readable for RF0Rrs {}
///`write(|w| ..)` method takes [`rf0r::W`](W) writer structure
impl crate::Writable for RF0Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RF0R to value 0
impl crate::Resettable for RF0Rrs {}
