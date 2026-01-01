///Register `P0SCSZR` reader
pub type R = crate::R<P0SCSZRrs>;
///Register `P0SCSZR` writer
pub type W = crate::W<P0SCSZRrs>;
///Field `HSIZE` reader - Horizontal size, from 0 to 4094 word wide (data 32-bit)
pub type HSIZE_R = crate::FieldReader<u16>;
///Field `HSIZE` writer - Horizontal size, from 0 to 4094 word wide (data 32-bit)
pub type HSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `VSIZE` reader - Vertical size, from 0 to 4094 pixels high
pub type VSIZE_R = crate::FieldReader<u16>;
///Field `VSIZE` writer - Vertical size, from 0 to 4094 pixels high
pub type VSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `POSNEG` reader - This bit is set and cleared by software. It has a meaning only if ENABLE bit is set.
pub type POSNEG_R = crate::BitReader;
///Field `POSNEG` writer - This bit is set and cleared by software. It has a meaning only if ENABLE bit is set.
pub type POSNEG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENABLE` reader - This bit is set and cleared by software.
pub type ENABLE_R = crate::BitReader;
///Field `ENABLE` writer - This bit is set and cleared by software.
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:11 - Horizontal size, from 0 to 4094 word wide (data 32-bit)
    #[inline(always)]
    pub fn hsize(&self) -> HSIZE_R {
        HSIZE_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - Vertical size, from 0 to 4094 pixels high
    #[inline(always)]
    pub fn vsize(&self) -> VSIZE_R {
        VSIZE_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    ///Bit 30 - This bit is set and cleared by software. It has a meaning only if ENABLE bit is set.
    #[inline(always)]
    pub fn posneg(&self) -> POSNEG_R {
        POSNEG_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - This bit is set and cleared by software.
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0SCSZR")
            .field("hsize", &self.hsize())
            .field("vsize", &self.vsize())
            .field("posneg", &self.posneg())
            .field("enable", &self.enable())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Horizontal size, from 0 to 4094 word wide (data 32-bit)
    #[inline(always)]
    pub fn hsize(&mut self) -> HSIZE_W<'_, P0SCSZRrs> {
        HSIZE_W::new(self, 0)
    }
    ///Bits 16:27 - Vertical size, from 0 to 4094 pixels high
    #[inline(always)]
    pub fn vsize(&mut self) -> VSIZE_W<'_, P0SCSZRrs> {
        VSIZE_W::new(self, 16)
    }
    ///Bit 30 - This bit is set and cleared by software. It has a meaning only if ENABLE bit is set.
    #[inline(always)]
    pub fn posneg(&mut self) -> POSNEG_W<'_, P0SCSZRrs> {
        POSNEG_W::new(self, 30)
    }
    ///Bit 31 - This bit is set and cleared by software.
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<'_, P0SCSZRrs> {
        ENABLE_W::new(self, 31)
    }
}
/**DCMIPP Pipe0 stat/crop size register

You can [`read`](crate::Reg::read) this register and get [`p0scszr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0scszr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DCMIPP:P0SCSZR)*/
pub struct P0SCSZRrs;
impl crate::RegisterSpec for P0SCSZRrs {
    type Ux = u32;
}
///`read()` method returns [`p0scszr::R`](R) reader structure
impl crate::Readable for P0SCSZRrs {}
///`write(|w| ..)` method takes [`p0scszr::W`](W) writer structure
impl crate::Writable for P0SCSZRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P0SCSZR to value 0
impl crate::Resettable for P0SCSZRrs {}
