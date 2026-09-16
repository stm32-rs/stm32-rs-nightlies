///Register `DMATBSCTRL0R` reader
pub type R = crate::R<DMATBSCTRL0Rrs>;
///Register `DMATBSCTRL0R` writer
pub type W = crate::W<DMATBSCTRL0Rrs>;
///Field `FTOV` reader - Fetch time offset valid
pub type FTOV_R = crate::BitReader;
///Field `FTOV` writer - Fetch time offset valid
pub type FTOV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FGOS` reader - Fetch GSN offset
pub type FGOS_R = crate::FieldReader;
///Field `FGOS` writer - Fetch GSN offset
pub type FGOS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `FTOS` reader - Fetch time offset
pub type FTOS_R = crate::FieldReader<u32>;
///Field `FTOS` writer - Fetch time offset
pub type FTOS_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bit 0 - Fetch time offset valid
    #[inline(always)]
    pub fn ftov(&self) -> FTOV_R {
        FTOV_R::new((self.bits & 1) != 0)
    }
    ///Bits 4:6 - Fetch GSN offset
    #[inline(always)]
    pub fn fgos(&self) -> FGOS_R {
        FGOS_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:31 - Fetch time offset
    #[inline(always)]
    pub fn ftos(&self) -> FTOS_R {
        FTOS_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMATBSCTRL0R")
            .field("ftov", &self.ftov())
            .field("fgos", &self.fgos())
            .field("ftos", &self.ftos())
            .finish()
    }
}
impl W {
    ///Bit 0 - Fetch time offset valid
    #[inline(always)]
    pub fn ftov(&mut self) -> FTOV_W<'_, DMATBSCTRL0Rrs> {
        FTOV_W::new(self, 0)
    }
    ///Bits 4:6 - Fetch GSN offset
    #[inline(always)]
    pub fn fgos(&mut self) -> FGOS_W<'_, DMATBSCTRL0Rrs> {
        FGOS_W::new(self, 4)
    }
    ///Bits 8:31 - Fetch time offset
    #[inline(always)]
    pub fn ftos(&mut self) -> FTOS_W<'_, DMATBSCTRL0Rrs> {
        FTOS_W::new(self, 8)
    }
}
/**DMA TBS control register 0

You can [`read`](crate::Reg::read) this register and get [`dmatbsctrl0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatbsctrl0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMATBSCTRL0R)*/
pub struct DMATBSCTRL0Rrs;
impl crate::RegisterSpec for DMATBSCTRL0Rrs {
    type Ux = u32;
}
///`read()` method returns [`dmatbsctrl0r::R`](R) reader structure
impl crate::Readable for DMATBSCTRL0Rrs {}
///`write(|w| ..)` method takes [`dmatbsctrl0r::W`](W) writer structure
impl crate::Writable for DMATBSCTRL0Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMATBSCTRL0R to value 0
impl crate::Resettable for DMATBSCTRL0Rrs {}
