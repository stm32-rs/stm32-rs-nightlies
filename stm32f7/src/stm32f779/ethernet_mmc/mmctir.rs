///Register `MMCTIR` reader
pub type R = crate::R<MMCTIRrs>;
///Field `TGFSCS` reader - Transmitted good frames single collision status
pub type TGFSCS_R = crate::BitReader;
///Field `TGFMSCS` reader - Transmitted good frames more than single collision status
pub type TGFMSCS_R = crate::BitReader;
///Field `TGFS` reader - Transmitted good frames status
pub type TGFS_R = crate::BitReader;
impl R {
    ///Bit 14 - Transmitted good frames single collision status
    #[inline(always)]
    pub fn tgfscs(&self) -> TGFSCS_R {
        TGFSCS_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Transmitted good frames more than single collision status
    #[inline(always)]
    pub fn tgfmscs(&self) -> TGFMSCS_R {
        TGFMSCS_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 21 - Transmitted good frames status
    #[inline(always)]
    pub fn tgfs(&self) -> TGFS_R {
        TGFS_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCTIR")
            .field("tgfscs", &self.tgfscs())
            .field("tgfmscs", &self.tgfmscs())
            .field("tgfs", &self.tgfs())
            .finish()
    }
}
/**Ethernet MMC transmit interrupt register

You can [`read`](crate::Reg::read) this register and get [`mmctir::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F779.html#Ethernet_MMC:MMCTIR)*/
pub struct MMCTIRrs;
impl crate::RegisterSpec for MMCTIRrs {
    type Ux = u32;
}
///`read()` method returns [`mmctir::R`](R) reader structure
impl crate::Readable for MMCTIRrs {}
///`reset()` method sets MMCTIR to value 0
impl crate::Resettable for MMCTIRrs {}
