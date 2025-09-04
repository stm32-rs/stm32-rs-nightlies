///Register `P2CRSZR` reader
pub type R = crate::R<P2CRSZRrs>;
///Register `P2CRSZR` writer
pub type W = crate::W<P2CRSZRrs>;
///Field `HSIZE` reader - Horizontal size, from 0 to 4094 pixels wide. If the value is maintained at 0 when enabling the crop by means of the ENABLE bit, the value is forced internally at 0xFFE, which is the maximum value.
pub type HSIZE_R = crate::FieldReader<u16>;
///Field `HSIZE` writer - Horizontal size, from 0 to 4094 pixels wide. If the value is maintained at 0 when enabling the crop by means of the ENABLE bit, the value is forced internally at 0xFFE, which is the maximum value.
pub type HSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `VSIZE` reader - Vertical size, from 0 to 4094 pixels high. If the value is maintained at 0 when enabling the crop thanks to the ENABLE bit, the value is forced internally at 0xFFE, which is the maximum value.
pub type VSIZE_R = crate::FieldReader<u16>;
///Field `VSIZE` writer - Vertical size, from 0 to 4094 pixels high. If the value is maintained at 0 when enabling the crop thanks to the ENABLE bit, the value is forced internally at 0xFFE, which is the maximum value.
pub type VSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `ENABLE` reader - None
pub type ENABLE_R = crate::BitReader;
///Field `ENABLE` writer - None
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:11 - Horizontal size, from 0 to 4094 pixels wide. If the value is maintained at 0 when enabling the crop by means of the ENABLE bit, the value is forced internally at 0xFFE, which is the maximum value.
    #[inline(always)]
    pub fn hsize(&self) -> HSIZE_R {
        HSIZE_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - Vertical size, from 0 to 4094 pixels high. If the value is maintained at 0 when enabling the crop thanks to the ENABLE bit, the value is forced internally at 0xFFE, which is the maximum value.
    #[inline(always)]
    pub fn vsize(&self) -> VSIZE_R {
        VSIZE_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    ///Bit 31 - None
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P2CRSZR")
            .field("hsize", &self.hsize())
            .field("vsize", &self.vsize())
            .field("enable", &self.enable())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Horizontal size, from 0 to 4094 pixels wide. If the value is maintained at 0 when enabling the crop by means of the ENABLE bit, the value is forced internally at 0xFFE, which is the maximum value.
    #[inline(always)]
    pub fn hsize(&mut self) -> HSIZE_W<P2CRSZRrs> {
        HSIZE_W::new(self, 0)
    }
    ///Bits 16:27 - Vertical size, from 0 to 4094 pixels high. If the value is maintained at 0 when enabling the crop thanks to the ENABLE bit, the value is forced internally at 0xFFE, which is the maximum value.
    #[inline(always)]
    pub fn vsize(&mut self) -> VSIZE_W<P2CRSZRrs> {
        VSIZE_W::new(self, 16)
    }
    ///Bit 31 - None
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<P2CRSZRrs> {
        ENABLE_W::new(self, 31)
    }
}
/**DCMIPP Pipex crop window size register

You can [`read`](crate::Reg::read) this register and get [`p2crszr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2crszr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DCMIPP:P2CRSZR)*/
pub struct P2CRSZRrs;
impl crate::RegisterSpec for P2CRSZRrs {
    type Ux = u32;
}
///`read()` method returns [`p2crszr::R`](R) reader structure
impl crate::Readable for P2CRSZRrs {}
///`write(|w| ..)` method takes [`p2crszr::W`](W) writer structure
impl crate::Writable for P2CRSZRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P2CRSZR to value 0
impl crate::Resettable for P2CRSZRrs {}
