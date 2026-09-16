///Register `CDSR` reader
pub type R = crate::R<CDSRrs>;
///Field `VDES` reader - VDES
pub type VDES_R = crate::BitReader;
///Field `HDES` reader - HDES
pub type HDES_R = crate::BitReader;
///Field `VSYNCS` reader - VSYNCS
pub type VSYNCS_R = crate::BitReader;
///Field `HSYNCS` reader - HSYNCS
pub type HSYNCS_R = crate::BitReader;
impl R {
    ///Bit 0 - VDES
    #[inline(always)]
    pub fn vdes(&self) -> VDES_R {
        VDES_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HDES
    #[inline(always)]
    pub fn hdes(&self) -> HDES_R {
        HDES_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - VSYNCS
    #[inline(always)]
    pub fn vsyncs(&self) -> VSYNCS_R {
        VSYNCS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSYNCS
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
/**This register returns the status of the current display phase which is controlled by the HSYNC, VSYNC, and horizontal/vertical DE signals. Example: if the current display phase is the vertical synchronization, the VSYNCS bit is set (active high). If the current display phase is the horizontal synchronization, the HSYNCS bit is active high.

You can [`read`](crate::Reg::read) this register and get [`cdsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LTDC:CDSR)*/
pub struct CDSRrs;
impl crate::RegisterSpec for CDSRrs {
    type Ux = u32;
}
///`read()` method returns [`cdsr::R`](R) reader structure
impl crate::Readable for CDSRrs {}
///`reset()` method sets CDSR to value 0x0f
impl crate::Resettable for CDSRrs {
    const RESET_VALUE: u32 = 0x0f;
}
