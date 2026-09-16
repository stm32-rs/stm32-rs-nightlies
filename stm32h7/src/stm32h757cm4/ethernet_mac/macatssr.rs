///Register `MACATSSR` reader
pub type R = crate::R<MACATSSRrs>;
///Field `AUXTSHI` reader - Auxiliary Timestamp
pub type AUXTSHI_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Auxiliary Timestamp
    #[inline(always)]
    pub fn auxtshi(&self) -> AUXTSHI_R {
        AUXTSHI_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACATSSR")
            .field("auxtshi", &self.auxtshi())
            .finish()
    }
}
/**Auxiliary timestamp seconds register

You can [`read`](crate::Reg::read) this register and get [`macatssr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#Ethernet_MAC:MACATSSR)*/
pub struct MACATSSRrs;
impl crate::RegisterSpec for MACATSSRrs {
    type Ux = u32;
}
///`read()` method returns [`macatssr::R`](R) reader structure
impl crate::Readable for MACATSSRrs {}
///`reset()` method sets MACATSSR to value 0
impl crate::Resettable for MACATSSRrs {}
