///Register `P2CRI3CR2` reader
pub type R = crate::R<P2CRI3CR2rs>;
///Field `HSIZE` reader - Current horizontal size, from 0 to 4094 pixels wide
pub type HSIZE_R = crate::FieldReader<u16>;
///Field `VSIZE` reader - Current vertical size, from 0 to 4094 pixels high
pub type VSIZE_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:11 - Current horizontal size, from 0 to 4094 pixels wide
    #[inline(always)]
    pub fn hsize(&self) -> HSIZE_R {
        HSIZE_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - Current vertical size, from 0 to 4094 pixels high
    #[inline(always)]
    pub fn vsize(&self) -> VSIZE_R {
        VSIZE_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P2CRI3CR2")
            .field("hsize", &self.hsize())
            .field("vsize", &self.vsize())
            .finish()
    }
}
/**DCMIPP Pipe2 current ROI3 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`p2cri3cr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DCMIPP:P2CRI3CR2)*/
pub struct P2CRI3CR2rs;
impl crate::RegisterSpec for P2CRI3CR2rs {
    type Ux = u32;
}
///`read()` method returns [`p2cri3cr2::R`](R) reader structure
impl crate::Readable for P2CRI3CR2rs {}
///`reset()` method sets P2CRI3CR2 to value 0
impl crate::Resettable for P2CRI3CR2rs {}
