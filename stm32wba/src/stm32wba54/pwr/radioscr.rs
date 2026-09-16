///Register `RADIOSCR` reader
pub type R = crate::R<RADIOSCRrs>;
///Field `MODE` reader - 2.4 GHz RADIO operating mode. 1x: 2.4 GHz RADIO active mode
pub type MODE_R = crate::FieldReader;
///Field `PHYMODE` reader - 2.4 GHz RADIO PHY operating mode
pub type PHYMODE_R = crate::BitReader;
///Field `ENCMODE` reader - 2.4 GHz RADIO encryption function operating mode
pub type ENCMODE_R = crate::BitReader;
///Field `RFVDDHPA` reader - 2.4 GHz RADIO VDDHPA control word. Bits \[3:0\] see Table 81: PA output power table format for definition. Bit \[4\] rf_event.
pub type RFVDDHPA_R = crate::FieldReader;
///Field `REGPARDYVDDRFPA` reader - Ready bit for VsubDDHPA/sub voltage level when selecting VDDRFPA input. Note: REGPARDYVDDRFPA does not allow to detect correct VsubDDHPA/sub voltage level when request to lower the level.
pub type REGPARDYVDDRFPA_R = crate::BitReader;
impl R {
    ///Bits 0:1 - 2.4 GHz RADIO operating mode. 1x: 2.4 GHz RADIO active mode
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - 2.4 GHz RADIO PHY operating mode
    #[inline(always)]
    pub fn phymode(&self) -> PHYMODE_R {
        PHYMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - 2.4 GHz RADIO encryption function operating mode
    #[inline(always)]
    pub fn encmode(&self) -> ENCMODE_R {
        ENCMODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 8:12 - 2.4 GHz RADIO VDDHPA control word. Bits \[3:0\] see Table 81: PA output power table format for definition. Bit \[4\] rf_event.
    #[inline(always)]
    pub fn rfvddhpa(&self) -> RFVDDHPA_R {
        RFVDDHPA_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bit 15 - Ready bit for VsubDDHPA/sub voltage level when selecting VDDRFPA input. Note: REGPARDYVDDRFPA does not allow to detect correct VsubDDHPA/sub voltage level when request to lower the level.
    #[inline(always)]
    pub fn regpardyvddrfpa(&self) -> REGPARDYVDDRFPA_R {
        REGPARDYVDDRFPA_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RADIOSCR")
            .field("mode", &self.mode())
            .field("phymode", &self.phymode())
            .field("encmode", &self.encmode())
            .field("rfvddhpa", &self.rfvddhpa())
            .field("regpardyvddrfpa", &self.regpardyvddrfpa())
            .finish()
    }
}
/**PWR 2.4 GHz RADIO status and control register

You can [`read`](crate::Reg::read) this register and get [`radioscr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#PWR:RADIOSCR)*/
pub struct RADIOSCRrs;
impl crate::RegisterSpec for RADIOSCRrs {
    type Ux = u32;
}
///`read()` method returns [`radioscr::R`](R) reader structure
impl crate::Readable for RADIOSCRrs {}
///`reset()` method sets RADIOSCR to value 0
impl crate::Resettable for RADIOSCRrs {}
