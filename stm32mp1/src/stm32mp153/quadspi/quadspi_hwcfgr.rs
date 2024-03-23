#[doc = "Register `QUADSPI_HWCFGR` reader"]
pub type R = crate::R<QUADSPI_HWCFGRrs>;
#[doc = "Field `FIFOSIZE` reader - FIFOSIZE"]
pub type FIFOSIZE_R = crate::FieldReader;
#[doc = "Field `FIFOPTR` reader - FIFOPTR"]
pub type FIFOPTR_R = crate::FieldReader;
#[doc = "Field `PRESCVAL` reader - PRESCVAL"]
pub type PRESCVAL_R = crate::FieldReader;
#[doc = "Field `IDLENGTH` reader - IDLENGTH"]
pub type IDLENGTH_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - FIFOSIZE"]
    #[inline(always)]
    pub fn fifosize(&self) -> FIFOSIZE_R {
        FIFOSIZE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - FIFOPTR"]
    #[inline(always)]
    pub fn fifoptr(&self) -> FIFOPTR_R {
        FIFOPTR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PRESCVAL"]
    #[inline(always)]
    pub fn prescval(&self) -> PRESCVAL_R {
        PRESCVAL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - IDLENGTH"]
    #[inline(always)]
    pub fn idlength(&self) -> IDLENGTH_R {
        IDLENGTH_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[doc = "QUADSPI HW configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`quadspi_hwcfgr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QUADSPI_HWCFGRrs;
impl crate::RegisterSpec for QUADSPI_HWCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`quadspi_hwcfgr::R`](R) reader structure"]
impl crate::Readable for QUADSPI_HWCFGRrs {}
#[doc = "`reset()` method sets QUADSPI_HWCFGR to value 0xb058"]
impl crate::Resettable for QUADSPI_HWCFGRrs {
    const RESET_VALUE: u32 = 0xb058;
}
