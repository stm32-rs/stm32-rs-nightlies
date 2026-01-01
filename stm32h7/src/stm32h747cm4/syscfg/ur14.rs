///Register `UR14` reader
pub type R = crate::R<UR14rs>;
///Register `UR14` writer
pub type W = crate::W<UR14rs>;
///Field `D1STPRST` reader - D1 Stop Reset
pub type D1STPRST_R = crate::BitReader;
///Field `D1STPRST` writer - D1 Stop Reset
pub type D1STPRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `D2SBRST` reader - D2 Standby Reset
pub type D2SBRST_R = crate::BitReader;
///Field `D2SBRST` writer - D2 Standby Reset
pub type D2SBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - D1 Stop Reset
    #[inline(always)]
    pub fn d1stprst(&self) -> D1STPRST_R {
        D1STPRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 16 - D2 Standby Reset
    #[inline(always)]
    pub fn d2sbrst(&self) -> D2SBRST_R {
        D2SBRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UR14")
            .field("d1stprst", &self.d1stprst())
            .field("d2sbrst", &self.d2sbrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - D1 Stop Reset
    #[inline(always)]
    pub fn d1stprst(&mut self) -> D1STPRST_W<'_, UR14rs> {
        D1STPRST_W::new(self, 0)
    }
    ///Bit 16 - D2 Standby Reset
    #[inline(always)]
    pub fn d2sbrst(&mut self) -> D2SBRST_W<'_, UR14rs> {
        D2SBRST_W::new(self, 16)
    }
}
/**SYSCFG user register 14

You can [`read`](crate::Reg::read) this register and get [`ur14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ur14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#SYSCFG:UR14)*/
pub struct UR14rs;
impl crate::RegisterSpec for UR14rs {
    type Ux = u32;
}
///`read()` method returns [`ur14::R`](R) reader structure
impl crate::Readable for UR14rs {}
///`write(|w| ..)` method takes [`ur14::W`](W) writer structure
impl crate::Writable for UR14rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets UR14 to value 0
impl crate::Resettable for UR14rs {}
