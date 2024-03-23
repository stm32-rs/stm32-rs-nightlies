#[doc = "Register `HWCFGR` reader"]
pub type R = crate::R<HWCFGRrs>;
#[doc = "Field `AXI` reader - AXI interface"]
pub type AXI_R = crate::FieldReader;
#[doc = "Field `FIFO` reader - FIFO depth"]
pub type FIFO_R = crate::FieldReader;
#[doc = "Field `PRES` reader - Prescaler"]
pub type PRES_R = crate::FieldReader;
#[doc = "Field `IDL` reader - ID Length"]
pub type IDL_R = crate::FieldReader;
#[doc = "Field `MMW` reader - Memory map write"]
pub type MMW_R = crate::FieldReader;
#[doc = "Field `MST` reader - Master"]
pub type MST_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - AXI interface"]
    #[inline(always)]
    pub fn axi(&self) -> AXI_R {
        AXI_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:11 - FIFO depth"]
    #[inline(always)]
    pub fn fifo(&self) -> FIFO_R {
        FIFO_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:19 - Prescaler"]
    #[inline(always)]
    pub fn pres(&self) -> PRES_R {
        PRES_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:23 - ID Length"]
    #[inline(always)]
    pub fn idl(&self) -> IDL_R {
        IDL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Memory map write"]
    #[inline(always)]
    pub fn mmw(&self) -> MMW_R {
        MMW_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Master"]
    #[inline(always)]
    pub fn mst(&self) -> MST_R {
        MST_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "HW configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HWCFGRrs;
impl crate::RegisterSpec for HWCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwcfgr::R`](R) reader structure"]
impl crate::Readable for HWCFGRrs {}
#[doc = "`reset()` method sets HWCFGR to value 0x1130_0080"]
impl crate::Resettable for HWCFGRrs {
    const RESET_VALUE: u32 = 0x1130_0080;
}
