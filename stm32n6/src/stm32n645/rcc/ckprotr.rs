///Register `CKPROTR` reader
pub type R = crate::R<CKPROTRrs>;
///Field `XSPI3SELS` reader - XSPI3 clock selection current status
pub type XSPI3SELS_R = crate::FieldReader;
///Field `XSPI2SELS` reader - XSPI2 clock selection current status
pub type XSPI2SELS_R = crate::FieldReader;
///Field `XSPI1SELS` reader - XSPI1 clock selection current status
pub type XSPI1SELS_R = crate::FieldReader;
///Field `FMCSELS` reader - FMC clock selection current status
pub type FMCSELS_R = crate::FieldReader;
impl R {
    ///Bits 16:17 - XSPI3 clock selection current status
    #[inline(always)]
    pub fn xspi3sels(&self) -> XSPI3SELS_R {
        XSPI3SELS_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 20:21 - XSPI2 clock selection current status
    #[inline(always)]
    pub fn xspi2sels(&self) -> XSPI2SELS_R {
        XSPI2SELS_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 24:25 - XSPI1 clock selection current status
    #[inline(always)]
    pub fn xspi1sels(&self) -> XSPI1SELS_R {
        XSPI1SELS_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 28:29 - FMC clock selection current status
    #[inline(always)]
    pub fn fmcsels(&self) -> FMCSELS_R {
        FMCSELS_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CKPROTR")
            .field("xspi3sels", &self.xspi3sels())
            .field("xspi2sels", &self.xspi2sels())
            .field("xspi1sels", &self.xspi1sels())
            .field("fmcsels", &self.fmcsels())
            .finish()
    }
}
/**RCC clock protection register

You can [`read`](crate::Reg::read) this register and get [`ckprotr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:CKPROTR)*/
pub struct CKPROTRrs;
impl crate::RegisterSpec for CKPROTRrs {
    type Ux = u32;
}
///`read()` method returns [`ckprotr::R`](R) reader structure
impl crate::Readable for CKPROTRrs {}
///`reset()` method sets CKPROTR to value 0
impl crate::Resettable for CKPROTRrs {}
