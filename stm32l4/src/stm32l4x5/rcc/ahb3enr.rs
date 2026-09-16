///Register `AHB3ENR` reader
pub type R = crate::R<AHB3ENRrs>;
///Register `AHB3ENR` writer
pub type W = crate::W<AHB3ENRrs>;
///Field `FMCEN` reader - Flexible memory controller clock enable
pub type FMCEN_R = crate::BitReader;
///Field `FMCEN` writer - Flexible memory controller clock enable
pub type FMCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `QSPIEN` reader - QSPIEN
pub type QSPIEN_R = crate::BitReader;
///Field `QSPIEN` writer - QSPIEN
pub type QSPIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Flexible memory controller clock enable
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - QSPIEN
    #[inline(always)]
    pub fn qspien(&self) -> QSPIEN_R {
        QSPIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB3ENR")
            .field("qspien", &self.qspien())
            .field("fmcen", &self.fmcen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Flexible memory controller clock enable
    #[inline(always)]
    pub fn fmcen(&mut self) -> FMCEN_W<'_, AHB3ENRrs> {
        FMCEN_W::new(self, 0)
    }
    ///Bit 8 - QSPIEN
    #[inline(always)]
    pub fn qspien(&mut self) -> QSPIEN_W<'_, AHB3ENRrs> {
        QSPIEN_W::new(self, 8)
    }
}
/**AHB3 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb3enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x5.html#RCC:AHB3ENR)*/
pub struct AHB3ENRrs;
impl crate::RegisterSpec for AHB3ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb3enr::R`](R) reader structure
impl crate::Readable for AHB3ENRrs {}
///`write(|w| ..)` method takes [`ahb3enr::W`](W) writer structure
impl crate::Writable for AHB3ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB3ENR to value 0
impl crate::Resettable for AHB3ENRrs {}
