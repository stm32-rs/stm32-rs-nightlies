///Register `P2DSCR` reader
pub type R = crate::R<P2DSCRrs>;
///Register `P2DSCR` writer
pub type W = crate::W<P2DSCRrs>;
///Field `HDIV` reader - Horizontal division factor, from 128 (8x) to 1023 (1x)
pub type HDIV_R = crate::FieldReader<u16>;
///Field `HDIV` writer - Horizontal division factor, from 128 (8x) to 1023 (1x)
pub type HDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `VDIV` reader - Vertical division factor, from 128 (8x) to 1023 (1x)
pub type VDIV_R = crate::FieldReader<u16>;
///Field `VDIV` writer - Vertical division factor, from 128 (8x) to 1023 (1x)
pub type VDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `ENABLE` reader - None
pub type ENABLE_R = crate::BitReader;
///Field `ENABLE` writer - None
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:9 - Horizontal division factor, from 128 (8x) to 1023 (1x)
    #[inline(always)]
    pub fn hdiv(&self) -> HDIV_R {
        HDIV_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 16:25 - Vertical division factor, from 128 (8x) to 1023 (1x)
    #[inline(always)]
    pub fn vdiv(&self) -> VDIV_R {
        VDIV_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    ///Bit 31 - None
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P2DSCR")
            .field("hdiv", &self.hdiv())
            .field("vdiv", &self.vdiv())
            .field("enable", &self.enable())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Horizontal division factor, from 128 (8x) to 1023 (1x)
    #[inline(always)]
    pub fn hdiv(&mut self) -> HDIV_W<'_, P2DSCRrs> {
        HDIV_W::new(self, 0)
    }
    ///Bits 16:25 - Vertical division factor, from 128 (8x) to 1023 (1x)
    #[inline(always)]
    pub fn vdiv(&mut self) -> VDIV_W<'_, P2DSCRrs> {
        VDIV_W::new(self, 16)
    }
    ///Bit 31 - None
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<'_, P2DSCRrs> {
        ENABLE_W::new(self, 31)
    }
}
/**DCMIPP Pipex downsize configuration register

You can [`read`](crate::Reg::read) this register and get [`p2dscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2dscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DCMIPP:P2DSCR)*/
pub struct P2DSCRrs;
impl crate::RegisterSpec for P2DSCRrs {
    type Ux = u32;
}
///`read()` method returns [`p2dscr::R`](R) reader structure
impl crate::Readable for P2DSCRrs {}
///`write(|w| ..)` method takes [`p2dscr::W`](W) writer structure
impl crate::Writable for P2DSCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P2DSCR to value 0
impl crate::Resettable for P2DSCRrs {}
