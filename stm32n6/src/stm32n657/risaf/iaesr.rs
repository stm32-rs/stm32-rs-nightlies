///Register `IAESR` reader
pub type R = crate::R<IAESRrs>;
///Field `IACID` reader - illegal access compartment ID
pub type IACID_R = crate::FieldReader;
///Field `IAPRIV` reader - illegal access privileged
pub type IAPRIV_R = crate::BitReader;
///Field `IASEC` reader - illegal access security
pub type IASEC_R = crate::BitReader;
///Field `IANRW` reader - illegal access read/write
pub type IANRW_R = crate::BitReader;
impl R {
    ///Bits 0:2 - illegal access compartment ID
    #[inline(always)]
    pub fn iacid(&self) -> IACID_R {
        IACID_R::new((self.bits & 7) as u8)
    }
    ///Bit 4 - illegal access privileged
    #[inline(always)]
    pub fn iapriv(&self) -> IAPRIV_R {
        IAPRIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - illegal access security
    #[inline(always)]
    pub fn iasec(&self) -> IASEC_R {
        IASEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - illegal access read/write
    #[inline(always)]
    pub fn ianrw(&self) -> IANRW_R {
        IANRW_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IAESR")
            .field("iacid", &self.iacid())
            .field("iapriv", &self.iapriv())
            .field("iasec", &self.iasec())
            .field("ianrw", &self.ianrw())
            .finish()
    }
}
/**RISAF illegal access error status register

You can [`read`](crate::Reg::read) this register and get [`iaesr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RISAF:IAESR)*/
pub struct IAESRrs;
impl crate::RegisterSpec for IAESRrs {
    type Ux = u32;
}
///`read()` method returns [`iaesr::R`](R) reader structure
impl crate::Readable for IAESRrs {}
///`reset()` method sets IAESR to value 0
impl crate::Resettable for IAESRrs {}
