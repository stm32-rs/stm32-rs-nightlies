///Register `BDCR` reader
pub type R = crate::R<BDCRrs>;
///Register `BDCR` writer
pub type W = crate::W<BDCRrs>;
///Field `VSWRST` reader - VSW domain software reset.
pub type VSWRST_R = crate::BitReader;
///Field `VSWRST` writer - VSW domain software reset.
pub type VSWRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 31 - VSW domain software reset.
    #[inline(always)]
    pub fn vswrst(&self) -> VSWRST_R {
        VSWRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BDCR")
            .field("vswrst", &self.vswrst())
            .finish()
    }
}
impl W {
    ///Bit 31 - VSW domain software reset.
    #[inline(always)]
    pub fn vswrst(&mut self) -> VSWRST_W<'_, BDCRrs> {
        VSWRST_W::new(self, 31)
    }
}
/**RCC backup domain protection register

You can [`read`](crate::Reg::read) this register and get [`bdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:BDCR)*/
pub struct BDCRrs;
impl crate::RegisterSpec for BDCRrs {
    type Ux = u32;
}
///`read()` method returns [`bdcr::R`](R) reader structure
impl crate::Readable for BDCRrs {}
///`write(|w| ..)` method takes [`bdcr::W`](W) writer structure
impl crate::Writable for BDCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BDCR to value 0
impl crate::Resettable for BDCRrs {}
