///Register `HWCFGR` reader
pub type R = crate::R<HWCFGRrs>;
///Field `AXI` reader - AXI interface
pub type AXI_R = crate::FieldReader;
///Field `FIFO` reader - FIFO depth
pub type FIFO_R = crate::FieldReader;
///Field `PRES` reader - Prescaler
pub type PRES_R = crate::FieldReader;
///Field `IDL` reader - ID Length
pub type IDL_R = crate::FieldReader;
///Field `MMW` reader - Memory map write
pub type MMW_R = crate::FieldReader;
///Field `MST` reader - Master
pub type MST_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - AXI interface
    #[inline(always)]
    pub fn axi(&self) -> AXI_R {
        AXI_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:11 - FIFO depth
    #[inline(always)]
    pub fn fifo(&self) -> FIFO_R {
        FIFO_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    ///Bits 12:19 - Prescaler
    #[inline(always)]
    pub fn pres(&self) -> PRES_R {
        PRES_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    ///Bits 20:23 - ID Length
    #[inline(always)]
    pub fn idl(&self) -> IDL_R {
        IDL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Memory map write
    #[inline(always)]
    pub fn mmw(&self) -> MMW_R {
        MMW_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Master
    #[inline(always)]
    pub fn mst(&self) -> MST_R {
        MST_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR")
            .field("axi", &self.axi())
            .field("fifo", &self.fifo())
            .field("pres", &self.pres())
            .field("idl", &self.idl())
            .field("mmw", &self.mmw())
            .field("mst", &self.mst())
            .finish()
    }
}
/**HW configuration register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#OCTOSPI1:HWCFGR)*/
pub struct HWCFGRrs;
impl crate::RegisterSpec for HWCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr::R`](R) reader structure
impl crate::Readable for HWCFGRrs {}
///`reset()` method sets HWCFGR to value 0x1130_0080
impl crate::Resettable for HWCFGRrs {
    const RESET_VALUE: u32 = 0x1130_0080;
}
