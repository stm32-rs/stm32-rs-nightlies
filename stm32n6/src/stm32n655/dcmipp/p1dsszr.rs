///Register `P1DSSZR` reader
pub type R = crate::R<P1DSSZRrs>;
///Register `P1DSSZR` writer
pub type W = crate::W<P1DSSZRrs>;
///Field `HSIZE` reader - Horizontal size, from 0 to 4094 pixels wide
pub type HSIZE_R = crate::FieldReader<u16>;
///Field `HSIZE` writer - Horizontal size, from 0 to 4094 pixels wide
pub type HSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `VSIZE` reader - Vertical size, from 0 to 4094 pixels high
pub type VSIZE_R = crate::FieldReader<u16>;
///Field `VSIZE` writer - Vertical size, from 0 to 4094 pixels high
pub type VSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1DSSZR")
            .field("hsize", &self.hsize())
            .field("vsize", &self.vsize())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Horizontal size, from 0 to 4094 pixels wide
    #[inline(always)]
    pub fn hsize(&mut self) -> HSIZE_W<'_, P1DSSZRrs> {
        HSIZE_W::new(self, 0)
    }
    ///Bits 16:27 - Vertical size, from 0 to 4094 pixels high
    #[inline(always)]
    pub fn vsize(&mut self) -> VSIZE_W<'_, P1DSSZRrs> {
        VSIZE_W::new(self, 16)
    }
}
/**DCMIPP Pipex downsize destination size register

You can [`read`](crate::Reg::read) this register and get [`p1dsszr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1dsszr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DCMIPP:P1DSSZR)*/
pub struct P1DSSZRrs;
impl crate::RegisterSpec for P1DSSZRrs {
    type Ux = u32;
}
///`read()` method returns [`p1dsszr::R`](R) reader structure
impl crate::Readable for P1DSSZRrs {}
///`write(|w| ..)` method takes [`p1dsszr::W`](W) writer structure
impl crate::Writable for P1DSSZRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P1DSSZR to value 0
impl crate::Resettable for P1DSSZRrs {}
