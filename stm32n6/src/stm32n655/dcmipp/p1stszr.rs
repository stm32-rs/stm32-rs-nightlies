///Register `P1STSZR` reader
pub type R = crate::R<P1STSZRrs>;
///Register `P1STSZR` writer
pub type W = crate::W<P1STSZRrs>;
///Field `HSIZE` reader - Horizontal size, from 0 to 4094 pixels wide
pub type HSIZE_R = crate::FieldReader<u16>;
///Field `HSIZE` writer - Horizontal size, from 0 to 4094 pixels wide
pub type HSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `VSIZE` reader - Vertical size, from 0 to 4094 pixels high
pub type VSIZE_R = crate::FieldReader<u16>;
///Field `VSIZE` writer - Vertical size, from 0 to 4094 pixels high
pub type VSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `CROPEN` reader - None
pub type CROPEN_R = crate::BitReader;
///Field `CROPEN` writer - None
pub type CROPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:11 - Horizontal size, from 0 to 4094 pixels wide
    #[inline(always)]
    pub fn hsize(&self) -> HSIZE_R {
        HSIZE_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - Vertical size, from 0 to 4094 pixels high
    #[inline(always)]
    pub fn vsize(&self) -> VSIZE_R {
        VSIZE_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    ///Bit 31 - None
    #[inline(always)]
    pub fn cropen(&self) -> CROPEN_R {
        CROPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1STSZR")
            .field("hsize", &self.hsize())
            .field("vsize", &self.vsize())
            .field("cropen", &self.cropen())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Horizontal size, from 0 to 4094 pixels wide
    #[inline(always)]
    pub fn hsize(&mut self) -> HSIZE_W<'_, P1STSZRrs> {
        HSIZE_W::new(self, 0)
    }
    ///Bits 16:27 - Vertical size, from 0 to 4094 pixels high
    #[inline(always)]
    pub fn vsize(&mut self) -> VSIZE_W<'_, P1STSZRrs> {
        VSIZE_W::new(self, 16)
    }
    ///Bit 31 - None
    #[inline(always)]
    pub fn cropen(&mut self) -> CROPEN_W<'_, P1STSZRrs> {
        CROPEN_W::new(self, 31)
    }
}
/**DCMIPP Pipe1 statistics window size register

You can [`read`](crate::Reg::read) this register and get [`p1stszr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1stszr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DCMIPP:P1STSZR)*/
pub struct P1STSZRrs;
impl crate::RegisterSpec for P1STSZRrs {
    type Ux = u32;
}
///`read()` method returns [`p1stszr::R`](R) reader structure
impl crate::Readable for P1STSZRrs {}
///`write(|w| ..)` method takes [`p1stszr::W`](W) writer structure
impl crate::Writable for P1STSZRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P1STSZR to value 0
impl crate::Resettable for P1STSZRrs {}
