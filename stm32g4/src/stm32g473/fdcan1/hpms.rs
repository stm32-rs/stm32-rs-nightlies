///Register `HPMS` reader
pub type R = crate::R<HPMSrs>;
///Field `BIDX` reader - BIDX
pub type BIDX_R = crate::FieldReader;
///Field `MSI` reader - MSI
pub type MSI_R = crate::FieldReader;
///Field `FIDX` reader - FIDX
pub type FIDX_R = crate::FieldReader;
///Field `FLST` reader - FLST
pub type FLST_R = crate::BitReader;
impl R {
    ///Bits 0:2 - BIDX
    #[inline(always)]
    pub fn bidx(&self) -> BIDX_R {
        BIDX_R::new((self.bits & 7) as u8)
    }
    ///Bits 6:7 - MSI
    #[inline(always)]
    pub fn msi(&self) -> MSI_R {
        MSI_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:12 - FIDX
    #[inline(always)]
    pub fn fidx(&self) -> FIDX_R {
        FIDX_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bit 15 - FLST
    #[inline(always)]
    pub fn flst(&self) -> FLST_R {
        FLST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPMS")
            .field("bidx", &self.bidx())
            .field("msi", &self.msi())
            .field("fidx", &self.fidx())
            .field("flst", &self.flst())
            .finish()
    }
}
/**This register is updated every time a Message ID filter element configured to generate a priority event match. This can be used to monitor the status of incoming high priority messages and to enable fast access to these messages.

You can [`read`](crate::Reg::read) this register and get [`hpms::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G473.html#FDCAN1:HPMS)*/
pub struct HPMSrs;
impl crate::RegisterSpec for HPMSrs {
    type Ux = u32;
}
///`read()` method returns [`hpms::R`](R) reader structure
impl crate::Readable for HPMSrs {}
///`reset()` method sets HPMS to value 0
impl crate::Resettable for HPMSrs {}
