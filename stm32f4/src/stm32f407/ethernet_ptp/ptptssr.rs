///Register `PTPTSSR` reader
pub type R = crate::R<PTPTSSRrs>;
///Field `TSSO` reader - TSSO
pub type TSSO_R = crate::BitReader;
///Field `TSTTR` reader - TSTTR
pub type TSTTR_R = crate::BitReader;
impl R {
    ///Bit 0 - TSSO
    #[inline(always)]
    pub fn tsso(&self) -> TSSO_R {
        TSSO_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TSTTR
    #[inline(always)]
    pub fn tsttr(&self) -> TSTTR_R {
        TSTTR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTPTSSR")
            .field("tsso", &self.tsso())
            .field("tsttr", &self.tsttr())
            .finish()
    }
}
/**Ethernet PTP time stamp status register

You can [`read`](crate::Reg::read) this register and get [`ptptssr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F407.html#Ethernet_PTP:PTPTSSR)*/
pub struct PTPTSSRrs;
impl crate::RegisterSpec for PTPTSSRrs {
    type Ux = u32;
}
///`read()` method returns [`ptptssr::R`](R) reader structure
impl crate::Readable for PTPTSSRrs {}
///`reset()` method sets PTPTSSR to value 0
impl crate::Resettable for PTPTSSRrs {}
