///Register `RISR` reader
pub type R = crate::R<RISRrs>;
///Field `INRIS` reader - Input FIFO service raw interrupt status This read-only bit is set by hardware when an input FIFO flag (IFNF or IFEM) is set in CRYP_SR register, regardless of the INIM mask bit value in CRYP_IMSCR register.
pub type INRIS_R = crate::BitReader;
///Field `OUTRIS` reader - Output FIFO service raw interrupt status This read-only bit is set by hardware when an output FIFO flag (OFFU or OFNE) is set in CRYP_SR register, regardless of the OUTIM mask bit value in CRYP_IMSCR register.
pub type OUTRIS_R = crate::BitReader;
impl R {
    ///Bit 0 - Input FIFO service raw interrupt status This read-only bit is set by hardware when an input FIFO flag (IFNF or IFEM) is set in CRYP_SR register, regardless of the INIM mask bit value in CRYP_IMSCR register.
    #[inline(always)]
    pub fn inris(&self) -> INRIS_R {
        INRIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Output FIFO service raw interrupt status This read-only bit is set by hardware when an output FIFO flag (OFFU or OFNE) is set in CRYP_SR register, regardless of the OUTIM mask bit value in CRYP_IMSCR register.
    #[inline(always)]
    pub fn outris(&self) -> OUTRIS_R {
        OUTRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RISR")
            .field("inris", &self.inris())
            .field("outris", &self.outris())
            .finish()
    }
}
/**CRYP raw interrupt status register

You can [`read`](crate::Reg::read) this register and get [`risr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#CRYP:RISR)*/
pub struct RISRrs;
impl crate::RegisterSpec for RISRrs {
    type Ux = u32;
}
///`read()` method returns [`risr::R`](R) reader structure
impl crate::Readable for RISRrs {}
///`reset()` method sets RISR to value 0x01
impl crate::Resettable for RISRrs {
    const RESET_VALUE: u32 = 0x01;
}
