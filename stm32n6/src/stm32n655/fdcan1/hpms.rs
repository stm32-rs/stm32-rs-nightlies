///Register `HPMS` reader
pub type R = crate::R<HPMSrs>;
///Field `BIDX` reader - Buffer index
pub type BIDX_R = crate::FieldReader;
///Field `MSI` reader - Message storage indicator
pub type MSI_R = crate::FieldReader;
///Field `FIDX` reader - Filter index
pub type FIDX_R = crate::FieldReader;
///Field `FLST` reader - Filter list
pub type FLST_R = crate::BitReader;
impl R {
    ///Bits 0:5 - Buffer index
    #[inline(always)]
    pub fn bidx(&self) -> BIDX_R {
        BIDX_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:7 - Message storage indicator
    #[inline(always)]
    pub fn msi(&self) -> MSI_R {
        MSI_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:14 - Filter index
    #[inline(always)]
    pub fn fidx(&self) -> FIDX_R {
        FIDX_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 15 - Filter list
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
/**FDCAN high priority message status register

You can [`read`](crate::Reg::read) this register and get [`hpms::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#FDCAN1:HPMS)*/
pub struct HPMSrs;
impl crate::RegisterSpec for HPMSrs {
    type Ux = u32;
}
///`read()` method returns [`hpms::R`](R) reader structure
impl crate::Readable for HPMSrs {}
///`reset()` method sets HPMS to value 0
impl crate::Resettable for HPMSrs {}
