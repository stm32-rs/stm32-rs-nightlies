///Register `P1CDCCR` reader
pub type R = crate::R<P1CDCCRrs>;
///Register `P1CDCCR` writer
pub type W = crate::W<P1CDCCRrs>;
///Field `ENABLE` reader - None
pub type ENABLE_R = crate::BitReader;
///Field `ENABLE` writer - None
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HDEC` reader - Horizontal decimation ratio
pub type HDEC_R = crate::FieldReader;
///Field `HDEC` writer - Horizontal decimation ratio
pub type HDEC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `VDEC` reader - Vertical decimation ratio
pub type VDEC_R = crate::FieldReader;
///Field `VDEC` writer - Vertical decimation ratio
pub type VDEC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - None
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Horizontal decimation ratio
    #[inline(always)]
    pub fn hdec(&self) -> HDEC_R {
        HDEC_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 3:4 - Vertical decimation ratio
    #[inline(always)]
    pub fn vdec(&self) -> VDEC_R {
        VDEC_R::new(((self.bits >> 3) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1CDCCR")
            .field("enable", &self.enable())
            .field("hdec", &self.hdec())
            .field("vdec", &self.vdec())
            .finish()
    }
}
impl W {
    ///Bit 0 - None
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<'_, P1CDCCRrs> {
        ENABLE_W::new(self, 0)
    }
    ///Bits 1:2 - Horizontal decimation ratio
    #[inline(always)]
    pub fn hdec(&mut self) -> HDEC_W<'_, P1CDCCRrs> {
        HDEC_W::new(self, 1)
    }
    ///Bits 3:4 - Vertical decimation ratio
    #[inline(always)]
    pub fn vdec(&mut self) -> VDEC_W<'_, P1CDCCRrs> {
        VDEC_W::new(self, 3)
    }
}
/**DCMIPP Pipex current decimation register

You can [`read`](crate::Reg::read) this register and get [`p1cdccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1cdccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DCMIPP:P1CDCCR)*/
pub struct P1CDCCRrs;
impl crate::RegisterSpec for P1CDCCRrs {
    type Ux = u32;
}
///`read()` method returns [`p1cdccr::R`](R) reader structure
impl crate::Readable for P1CDCCRrs {}
///`write(|w| ..)` method takes [`p1cdccr::W`](W) writer structure
impl crate::Writable for P1CDCCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P1CDCCR to value 0
impl crate::Resettable for P1CDCCRrs {}
