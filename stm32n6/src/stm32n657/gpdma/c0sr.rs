///Register `C0SR` reader
pub type R = crate::R<C0SRrs>;
///Field `IDLEF` reader - idle flag
pub type IDLEF_R = crate::BitReader;
///Field `TCF` reader - transfer complete flag
pub type TCF_R = crate::BitReader;
///Field `HTF` reader - half transfer flag
pub type HTF_R = crate::BitReader;
///Field `DTEF` reader - data transfer error flag
pub type DTEF_R = crate::BitReader;
///Field `ULEF` reader - update link transfer error flag
pub type ULEF_R = crate::BitReader;
///Field `USEF` reader - user setting error flag
pub type USEF_R = crate::BitReader;
///Field `SUSPF` reader - completed suspension flag
pub type SUSPF_R = crate::BitReader;
///Field `TOF` reader - trigger overrun flag
pub type TOF_R = crate::BitReader;
///Field `FIFOL` reader - monitored FIFO level
pub type FIFOL_R = crate::FieldReader;
impl R {
    ///Bit 0 - idle flag
    #[inline(always)]
    pub fn idlef(&self) -> IDLEF_R {
        IDLEF_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - transfer complete flag
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - half transfer flag
    #[inline(always)]
    pub fn htf(&self) -> HTF_R {
        HTF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - data transfer error flag
    #[inline(always)]
    pub fn dtef(&self) -> DTEF_R {
        DTEF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - update link transfer error flag
    #[inline(always)]
    pub fn ulef(&self) -> ULEF_R {
        ULEF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - user setting error flag
    #[inline(always)]
    pub fn usef(&self) -> USEF_R {
        USEF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - completed suspension flag
    #[inline(always)]
    pub fn suspf(&self) -> SUSPF_R {
        SUSPF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - trigger overrun flag
    #[inline(always)]
    pub fn tof(&self) -> TOF_R {
        TOF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 16:23 - monitored FIFO level
    #[inline(always)]
    pub fn fifol(&self) -> FIFOL_R {
        FIFOL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C0SR")
            .field("idlef", &self.idlef())
            .field("tcf", &self.tcf())
            .field("htf", &self.htf())
            .field("dtef", &self.dtef())
            .field("ulef", &self.ulef())
            .field("usef", &self.usef())
            .field("suspf", &self.suspf())
            .field("tof", &self.tof())
            .field("fifol", &self.fifol())
            .finish()
    }
}
/**GPDMA channel 0 status register

You can [`read`](crate::Reg::read) this register and get [`c0sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPDMA:C0SR)*/
pub struct C0SRrs;
impl crate::RegisterSpec for C0SRrs {
    type Ux = u32;
}
///`read()` method returns [`c0sr::R`](R) reader structure
impl crate::Readable for C0SRrs {}
///`reset()` method sets C0SR to value 0x01
impl crate::Resettable for C0SRrs {
    const RESET_VALUE: u32 = 0x01;
}
