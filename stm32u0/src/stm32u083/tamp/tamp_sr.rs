///Register `TAMP_SR` reader
pub type R = crate::R<TAMP_SRrs>;
///Field `TAMP1F` reader - TAMP1 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP1 input.
pub type TAMP1F_R = crate::BitReader;
///Field `TAMP2F` reader - TAMP2 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP2 input.
pub type TAMP2F_R = crate::BitReader;
///Field `TAMP3F` reader - TAMP3 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP3 input.
pub type TAMP3F_R = crate::BitReader;
///Field `TAMP4F` reader - TAMP4 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP4 input.
pub type TAMP4F_R = crate::BitReader;
///Field `TAMP5F` reader - TAMP5 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP5 input.
pub type TAMP5F_R = crate::BitReader;
///Field `ITAMP3F` reader - Internal tamper 3 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 3.
pub type ITAMP3F_R = crate::BitReader;
///Field `ITAMP4F` reader - Internal tamper 4 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 4.
pub type ITAMP4F_R = crate::BitReader;
///Field `ITAMP5F` reader - Internal tamper 5 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 5.
pub type ITAMP5F_R = crate::BitReader;
///Field `ITAMP6F` reader - Internal tamper 6 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 6.
pub type ITAMP6F_R = crate::BitReader;
impl R {
    ///Bit 0 - TAMP1 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP1 input.
    #[inline(always)]
    pub fn tamp1f(&self) -> TAMP1F_R {
        TAMP1F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TAMP2 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP2 input.
    #[inline(always)]
    pub fn tamp2f(&self) -> TAMP2F_R {
        TAMP2F_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TAMP3 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP3 input.
    #[inline(always)]
    pub fn tamp3f(&self) -> TAMP3F_R {
        TAMP3F_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TAMP4 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP4 input.
    #[inline(always)]
    pub fn tamp4f(&self) -> TAMP4F_R {
        TAMP4F_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TAMP5 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP5 input.
    #[inline(always)]
    pub fn tamp5f(&self) -> TAMP5F_R {
        TAMP5F_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 18 - Internal tamper 3 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 3.
    #[inline(always)]
    pub fn itamp3f(&self) -> ITAMP3F_R {
        ITAMP3F_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Internal tamper 4 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 4.
    #[inline(always)]
    pub fn itamp4f(&self) -> ITAMP4F_R {
        ITAMP4F_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Internal tamper 5 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 5.
    #[inline(always)]
    pub fn itamp5f(&self) -> ITAMP5F_R {
        ITAMP5F_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Internal tamper 6 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 6.
    #[inline(always)]
    pub fn itamp6f(&self) -> ITAMP6F_R {
        ITAMP6F_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TAMP_SR")
            .field("tamp1f", &self.tamp1f())
            .field("tamp2f", &self.tamp2f())
            .field("tamp3f", &self.tamp3f())
            .field("tamp4f", &self.tamp4f())
            .field("tamp5f", &self.tamp5f())
            .field("itamp3f", &self.itamp3f())
            .field("itamp4f", &self.itamp4f())
            .field("itamp5f", &self.itamp5f())
            .field("itamp6f", &self.itamp6f())
            .finish()
    }
}
/**TAMP status register

You can [`read`](crate::Reg::read) this register and get [`tamp_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TAMP:TAMP_SR)*/
pub struct TAMP_SRrs;
impl crate::RegisterSpec for TAMP_SRrs {
    type Ux = u32;
}
///`read()` method returns [`tamp_sr::R`](R) reader structure
impl crate::Readable for TAMP_SRrs {}
///`reset()` method sets TAMP_SR to value 0
impl crate::Resettable for TAMP_SRrs {
    const RESET_VALUE: u32 = 0;
}