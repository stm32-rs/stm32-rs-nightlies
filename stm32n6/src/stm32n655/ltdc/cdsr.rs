///Register `CDSR` reader
pub type R = crate::R<CDSRrs>;
///Field `VDES` reader - vertical data enable display status
pub type VDES_R = crate::BitReader;
///Field `HDES` reader - horizontal data enable display status
pub type HDES_R = crate::BitReader;
///Field `VSYNCS` reader - vertical synchronization display status
pub type VSYNCS_R = crate::BitReader;
///Field `HSYNCS` reader - horizontal synchronization display status
pub type HSYNCS_R = crate::BitReader;
impl R {
    ///Bit 0 - vertical data enable display status
    #[inline(always)]
    pub fn vdes(&self) -> VDES_R {
        VDES_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - horizontal data enable display status
    #[inline(always)]
    pub fn hdes(&self) -> HDES_R {
        HDES_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - vertical synchronization display status
    #[inline(always)]
    pub fn vsyncs(&self) -> VSYNCS_R {
        VSYNCS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - horizontal synchronization display status
    #[inline(always)]
    pub fn hsyncs(&self) -> HSYNCS_R {
        HSYNCS_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CDSR")
            .field("vdes", &self.vdes())
            .field("hdes", &self.hdes())
            .field("vsyncs", &self.vsyncs())
            .field("hsyncs", &self.hsyncs())
            .finish()
    }
}
/**LTDC current display status register

You can [`read`](crate::Reg::read) this register and get [`cdsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#LTDC:CDSR)*/
pub struct CDSRrs;
impl crate::RegisterSpec for CDSRrs {
    type Ux = u32;
}
///`read()` method returns [`cdsr::R`](R) reader structure
impl crate::Readable for CDSRrs {}
///`reset()` method sets CDSR to value 0x03
impl crate::Resettable for CDSRrs {
    const RESET_VALUE: u32 = 0x03;
}
