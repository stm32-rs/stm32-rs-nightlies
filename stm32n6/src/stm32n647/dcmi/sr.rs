///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `HSYNC` reader - Horizontal synchronization
pub type HSYNC_R = crate::BitReader;
///Field `VSYNC` reader - Vertical synchronization
pub type VSYNC_R = crate::BitReader;
///Field `FNE` reader - FIFO not empty
pub type FNE_R = crate::BitReader;
impl R {
    ///Bit 0 - Horizontal synchronization
    #[inline(always)]
    pub fn hsync(&self) -> HSYNC_R {
        HSYNC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Vertical synchronization
    #[inline(always)]
    pub fn vsync(&self) -> VSYNC_R {
        VSYNC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - FIFO not empty
    #[inline(always)]
    pub fn fne(&self) -> FNE_R {
        FNE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("hsync", &self.hsync())
            .field("vsync", &self.vsync())
            .field("fne", &self.fne())
            .finish()
    }
}
/**DCMI status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DCMI:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
