///Register `FNR` reader
pub type R = crate::R<FNRrs>;
///Field `FN` reader - Frame number
pub type FN_R = crate::FieldReader<u16>;
///Field `LSOF` reader - Lost SOF
pub type LSOF_R = crate::FieldReader;
///Field `LCK` reader - Locked
pub type LCK_R = crate::BitReader;
///Field `RXDM` reader - Receive data - line status
pub type RXDM_R = crate::BitReader;
///Field `RXDP` reader - Receive data + line status
pub type RXDP_R = crate::BitReader;
impl R {
    ///Bits 0:10 - Frame number
    #[inline(always)]
    pub fn fn_(&self) -> FN_R {
        FN_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 11:12 - Lost SOF
    #[inline(always)]
    pub fn lsof(&self) -> LSOF_R {
        LSOF_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bit 13 - Locked
    #[inline(always)]
    pub fn lck(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Receive data - line status
    #[inline(always)]
    pub fn rxdm(&self) -> RXDM_R {
        RXDM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Receive data + line status
    #[inline(always)]
    pub fn rxdp(&self) -> RXDP_R {
        RXDP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FNR")
            .field("fn_", &self.fn_())
            .field("lsof", &self.lsof())
            .field("lck", &self.lck())
            .field("rxdm", &self.rxdm())
            .field("rxdp", &self.rxdp())
            .finish()
    }
}
/**USB frame number register

You can [`read`](crate::Reg::read) this register and get [`fnr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#USB:FNR)*/
pub struct FNRrs;
impl crate::RegisterSpec for FNRrs {
    type Ux = u32;
}
///`read()` method returns [`fnr::R`](R) reader structure
impl crate::Readable for FNRrs {}
///`reset()` method sets FNR to value 0
impl crate::Resettable for FNRrs {}
