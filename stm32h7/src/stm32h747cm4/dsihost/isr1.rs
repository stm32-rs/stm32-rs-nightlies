///Register `ISR1` reader
pub type R = crate::R<ISR1rs>;
///Field `TOHSTX` reader - Timeout high
pub type TOHSTX_R = crate::BitReader;
///Field `TOLPRX` reader - Timeout low
pub type TOLPRX_R = crate::BitReader;
///Field `ECCSE` reader - ECC single
pub type ECCSE_R = crate::BitReader;
///Field `ECCME` reader - ECC multi
pub type ECCME_R = crate::BitReader;
///Field `CRCE` reader - CRC error
pub type CRCE_R = crate::BitReader;
///Field `PSE` reader - Packet size error
pub type PSE_R = crate::BitReader;
///Field `EOTPE` reader - EoTp error
pub type EOTPE_R = crate::BitReader;
///Field `LPWRE` reader - LTDC payload write error
pub type LPWRE_R = crate::BitReader;
///Field `GCWRE` reader - Generic command write error
pub type GCWRE_R = crate::BitReader;
///Field `GPWRE` reader - Generic payload write error
pub type GPWRE_R = crate::BitReader;
///Field `GPTXE` reader - Generic payload transmit error
pub type GPTXE_R = crate::BitReader;
///Field `GPRDE` reader - Generic payload read error
pub type GPRDE_R = crate::BitReader;
///Field `GPRXE` reader - Generic payload receive error
pub type GPRXE_R = crate::BitReader;
impl R {
    ///Bit 0 - Timeout high
    #[inline(always)]
    pub fn tohstx(&self) -> TOHSTX_R {
        TOHSTX_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Timeout low
    #[inline(always)]
    pub fn tolprx(&self) -> TOLPRX_R {
        TOLPRX_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ECC single
    #[inline(always)]
    pub fn eccse(&self) -> ECCSE_R {
        ECCSE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ECC multi
    #[inline(always)]
    pub fn eccme(&self) -> ECCME_R {
        ECCME_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CRC error
    #[inline(always)]
    pub fn crce(&self) -> CRCE_R {
        CRCE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Packet size error
    #[inline(always)]
    pub fn pse(&self) -> PSE_R {
        PSE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - EoTp error
    #[inline(always)]
    pub fn eotpe(&self) -> EOTPE_R {
        EOTPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - LTDC payload write error
    #[inline(always)]
    pub fn lpwre(&self) -> LPWRE_R {
        LPWRE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Generic command write error
    #[inline(always)]
    pub fn gcwre(&self) -> GCWRE_R {
        GCWRE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Generic payload write error
    #[inline(always)]
    pub fn gpwre(&self) -> GPWRE_R {
        GPWRE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Generic payload transmit error
    #[inline(always)]
    pub fn gptxe(&self) -> GPTXE_R {
        GPTXE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Generic payload read error
    #[inline(always)]
    pub fn gprde(&self) -> GPRDE_R {
        GPRDE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Generic payload receive error
    #[inline(always)]
    pub fn gprxe(&self) -> GPRXE_R {
        GPRXE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR1")
            .field("tohstx", &self.tohstx())
            .field("tolprx", &self.tolprx())
            .field("eccse", &self.eccse())
            .field("eccme", &self.eccme())
            .field("crce", &self.crce())
            .field("pse", &self.pse())
            .field("eotpe", &self.eotpe())
            .field("lpwre", &self.lpwre())
            .field("gcwre", &self.gcwre())
            .field("gpwre", &self.gpwre())
            .field("gptxe", &self.gptxe())
            .field("gprde", &self.gprde())
            .field("gprxe", &self.gprxe())
            .finish()
    }
}
/**DSI Host interrupt and status register 1

You can [`read`](crate::Reg::read) this register and get [`isr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#DSIHOST:ISR1)*/
pub struct ISR1rs;
impl crate::RegisterSpec for ISR1rs {
    type Ux = u32;
}
///`read()` method returns [`isr1::R`](R) reader structure
impl crate::Readable for ISR1rs {}
///`reset()` method sets ISR1 to value 0
impl crate::Resettable for ISR1rs {}
