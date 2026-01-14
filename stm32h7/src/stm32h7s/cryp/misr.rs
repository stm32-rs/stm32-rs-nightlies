///Register `MISR` reader
pub type R = crate::R<MISRrs>;
///Field `INMIS` reader - Input FIFO service masked interrupt status This read-only bit is set by hardware when an input FIFO flag (IFNF or IFEM) is set in CRYP_SR register. If the INIM mask bit is cleared in CRYP_IMSCR register, the INMIS bit stays cleared (masked). The INMIS bit is cleared by writing data to the input FIFO until IFEM flag is cleared (there is at least one word in input FIFO), or by clearing CRYPEN, When CRYP is disabled, INMIS bit stays low even if the input FIFO is empty.
pub type INMIS_R = crate::BitReader;
///Field `OUTMIS` reader - Output FIFO service masked interrupt status This read-only bit is set by hardware when an output FIFO flag (OFFU or OFNE) is set in CRYP_SR register. If the OUTIM mask bit is cleared in CRYP_IMSCR register, the OUTMIS bit stays cleared (masked). The OUTMIS bit is cleared by reading data from the output FIFO until OFNE flag is cleared (output FIFO empty). It is not cleared by disabling CRYP with CRYPEN bit.
pub type OUTMIS_R = crate::BitReader;
impl R {
    ///Bit 0 - Input FIFO service masked interrupt status This read-only bit is set by hardware when an input FIFO flag (IFNF or IFEM) is set in CRYP_SR register. If the INIM mask bit is cleared in CRYP_IMSCR register, the INMIS bit stays cleared (masked). The INMIS bit is cleared by writing data to the input FIFO until IFEM flag is cleared (there is at least one word in input FIFO), or by clearing CRYPEN, When CRYP is disabled, INMIS bit stays low even if the input FIFO is empty.
    #[inline(always)]
    pub fn inmis(&self) -> INMIS_R {
        INMIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Output FIFO service masked interrupt status This read-only bit is set by hardware when an output FIFO flag (OFFU or OFNE) is set in CRYP_SR register. If the OUTIM mask bit is cleared in CRYP_IMSCR register, the OUTMIS bit stays cleared (masked). The OUTMIS bit is cleared by reading data from the output FIFO until OFNE flag is cleared (output FIFO empty). It is not cleared by disabling CRYP with CRYPEN bit.
    #[inline(always)]
    pub fn outmis(&self) -> OUTMIS_R {
        OUTMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISR")
            .field("inmis", &self.inmis())
            .field("outmis", &self.outmis())
            .finish()
    }
}
/**CRYP masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#CRYP:MISR)*/
pub struct MISRrs;
impl crate::RegisterSpec for MISRrs {
    type Ux = u32;
}
///`read()` method returns [`misr::R`](R) reader structure
impl crate::Readable for MISRrs {}
///`reset()` method sets MISR to value 0
impl crate::Resettable for MISRrs {}
