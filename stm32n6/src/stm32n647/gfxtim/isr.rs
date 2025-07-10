///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Field `AFCOF` reader - absolute frame counter overflow flag
pub type AFCOF_R = crate::BitReader;
///Field `ALCOF` reader - absolute line counter overflow flag
pub type ALCOF_R = crate::BitReader;
///Field `TEF` reader - tearing-effect flag
pub type TEF_R = crate::BitReader;
///Field `AFCC1F` reader - absolute frame counter compare 1 flag
pub type AFCC1F_R = crate::BitReader;
///Field `ALCC1F` reader - absolute line counter compare 1 flag
pub type ALCC1F_R = crate::BitReader;
///Field `ALCC2F` reader - absolute line counter compare 2 flag
pub type ALCC2F_R = crate::BitReader;
///Field `RFC1RF` reader - relative frame counter 1 reload flag
pub type RFC1RF_R = crate::BitReader;
///Field `RFC2RF` reader - relative frame counter 2 reload flag
pub type RFC2RF_R = crate::BitReader;
///Field `EV1F` reader - event 1 flag
pub type EV1F_R = crate::BitReader;
///Field `EV2F` reader - event 2 flag
pub type EV2F_R = crate::BitReader;
///Field `EV3F` reader - event 3 flag
pub type EV3F_R = crate::BitReader;
///Field `EV4F` reader - event 4 flag
pub type EV4F_R = crate::BitReader;
///Field `WDGAF` reader - watchdog alarm flag
pub type WDGAF_R = crate::BitReader;
///Field `WDGPF` reader - watchdog pre-alarm flag
pub type WDGPF_R = crate::BitReader;
impl R {
    ///Bit 0 - absolute frame counter overflow flag
    #[inline(always)]
    pub fn afcof(&self) -> AFCOF_R {
        AFCOF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - absolute line counter overflow flag
    #[inline(always)]
    pub fn alcof(&self) -> ALCOF_R {
        ALCOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - tearing-effect flag
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - absolute frame counter compare 1 flag
    #[inline(always)]
    pub fn afcc1f(&self) -> AFCC1F_R {
        AFCC1F_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - absolute line counter compare 1 flag
    #[inline(always)]
    pub fn alcc1f(&self) -> ALCC1F_R {
        ALCC1F_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - absolute line counter compare 2 flag
    #[inline(always)]
    pub fn alcc2f(&self) -> ALCC2F_R {
        ALCC2F_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - relative frame counter 1 reload flag
    #[inline(always)]
    pub fn rfc1rf(&self) -> RFC1RF_R {
        RFC1RF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - relative frame counter 2 reload flag
    #[inline(always)]
    pub fn rfc2rf(&self) -> RFC2RF_R {
        RFC2RF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - event 1 flag
    #[inline(always)]
    pub fn ev1f(&self) -> EV1F_R {
        EV1F_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - event 2 flag
    #[inline(always)]
    pub fn ev2f(&self) -> EV2F_R {
        EV2F_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - event 3 flag
    #[inline(always)]
    pub fn ev3f(&self) -> EV3F_R {
        EV3F_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - event 4 flag
    #[inline(always)]
    pub fn ev4f(&self) -> EV4F_R {
        EV4F_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 24 - watchdog alarm flag
    #[inline(always)]
    pub fn wdgaf(&self) -> WDGAF_R {
        WDGAF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - watchdog pre-alarm flag
    #[inline(always)]
    pub fn wdgpf(&self) -> WDGPF_R {
        WDGPF_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("afcof", &self.afcof())
            .field("alcof", &self.alcof())
            .field("tef", &self.tef())
            .field("afcc1f", &self.afcc1f())
            .field("alcc1f", &self.alcc1f())
            .field("alcc2f", &self.alcc2f())
            .field("rfc1rf", &self.rfc1rf())
            .field("rfc2rf", &self.rfc2rf())
            .field("ev1f", &self.ev1f())
            .field("ev2f", &self.ev2f())
            .field("ev3f", &self.ev3f())
            .field("ev4f", &self.ev4f())
            .field("wdgaf", &self.wdgaf())
            .field("wdgpf", &self.wdgpf())
            .finish()
    }
}
/**GFXTIM interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#GFXTIM:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
