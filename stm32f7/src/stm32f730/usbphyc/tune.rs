///Register `TUNE` reader
pub type R = crate::R<TUNErs>;
///Register `TUNE` writer
pub type W = crate::W<TUNErs>;
///Field `INCURREN` reader - Controls the current boosting function
pub type INCURREN_R = crate::BitReader;
///Field `INCURREN` writer - Controls the current boosting function
pub type INCURREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INCURRINT` reader - Controls PHY current boosting
pub type INCURRINT_R = crate::BitReader;
///Field `INCURRINT` writer - Controls PHY current boosting
pub type INCURRINT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LFSCAPEN` reader - : Enables the Low Full Speed feedback capacitor
pub type LFSCAPEN_R = crate::BitReader;
///Field `LFSCAPEN` writer - : Enables the Low Full Speed feedback capacitor
pub type LFSCAPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSDRVSLEW` reader - Controls the HS driver slew rate
pub type HSDRVSLEW_R = crate::BitReader;
///Field `HSDRVSLEW` writer - Controls the HS driver slew rate
pub type HSDRVSLEW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSDRVDCCUR` reader - Decreases the HS driver DC level
pub type HSDRVDCCUR_R = crate::BitReader;
///Field `HSDRVDCCUR` writer - Decreases the HS driver DC level
pub type HSDRVDCCUR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSDRVDCLEV` reader - Increases the HS Driver DC level. Not applicable during the HS Test J and Test K data transfer
pub type HSDRVDCLEV_R = crate::BitReader;
///Field `HSDRVDCLEV` writer - Increases the HS Driver DC level. Not applicable during the HS Test J and Test K data transfer
pub type HSDRVDCLEV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSDRVCURINCR` reader - Enable the HS driver current increase feature
pub type HSDRVCURINCR_R = crate::BitReader;
///Field `HSDRVCURINCR` writer - Enable the HS driver current increase feature
pub type HSDRVCURINCR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSDRVRFADJ` reader - Tuning pin to adjust the full speed rise/fall time
pub type FSDRVRFADJ_R = crate::BitReader;
///Field `FSDRVRFADJ` writer - Tuning pin to adjust the full speed rise/fall time
pub type FSDRVRFADJ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSDRVRFRED` reader - High Speed rise-fall reduction enable
pub type HSDRVRFRED_R = crate::BitReader;
///Field `HSDRVRFRED` writer - High Speed rise-fall reduction enable
pub type HSDRVRFRED_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSDRVCHKITRM` reader - HS Driver current trimming pins for choke compensation
pub type HSDRVCHKITRM_R = crate::FieldReader;
///Field `HSDRVCHKITRM` writer - HS Driver current trimming pins for choke compensation
pub type HSDRVCHKITRM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HSDRVCHKZTRM` reader - Controls the PHY bus HS driver impedance tuning for choke compensation
pub type HSDRVCHKZTRM_R = crate::FieldReader;
///Field `HSDRVCHKZTRM` writer - Controls the PHY bus HS driver impedance tuning for choke compensation
pub type HSDRVCHKZTRM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SQLCHCTL` reader - Adjust the squelch DC threshold value
pub type SQLCHCTL_R = crate::FieldReader;
///Field `SQLCHCTL` writer - Adjust the squelch DC threshold value
pub type SQLCHCTL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `HDRXGNEQEN` reader - Enables the HS Rx Gain Equalizer
pub type HDRXGNEQEN_R = crate::BitReader;
///Field `HDRXGNEQEN` writer - Enables the HS Rx Gain Equalizer
pub type HDRXGNEQEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STAGSEL` reader - HS Tx staggering enable
pub type STAGSEL_R = crate::BitReader;
///Field `STAGSEL` writer - HS Tx staggering enable
pub type STAGSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSFALLPREEM` reader - HS Fall time control of single ended signals during pre-emphasis
pub type HSFALLPREEM_R = crate::BitReader;
///Field `HSFALLPREEM` writer - HS Fall time control of single ended signals during pre-emphasis
pub type HSFALLPREEM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSRXOFF` reader - : HS Receiver Offset adjustment
pub type HSRXOFF_R = crate::FieldReader;
///Field `HSRXOFF` writer - : HS Receiver Offset adjustment
pub type HSRXOFF_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SHTCCTCTLPROT` reader - Enables the short circuit protection circuitry in LS/FS driver
pub type SHTCCTCTLPROT_R = crate::BitReader;
///Field `SHTCCTCTLPROT` writer - Enables the short circuit protection circuitry in LS/FS driver
pub type SHTCCTCTLPROT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SQLBYP` reader - This pin is used to bypass the squelch inter-locking circuitry
pub type SQLBYP_R = crate::BitReader;
///Field `SQLBYP` writer - This pin is used to bypass the squelch inter-locking circuitry
pub type SQLBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Controls the current boosting function
    #[inline(always)]
    pub fn incurren(&self) -> INCURREN_R {
        INCURREN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Controls PHY current boosting
    #[inline(always)]
    pub fn incurrint(&self) -> INCURRINT_R {
        INCURRINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - : Enables the Low Full Speed feedback capacitor
    #[inline(always)]
    pub fn lfscapen(&self) -> LFSCAPEN_R {
        LFSCAPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Controls the HS driver slew rate
    #[inline(always)]
    pub fn hsdrvslew(&self) -> HSDRVSLEW_R {
        HSDRVSLEW_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Decreases the HS driver DC level
    #[inline(always)]
    pub fn hsdrvdccur(&self) -> HSDRVDCCUR_R {
        HSDRVDCCUR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Increases the HS Driver DC level. Not applicable during the HS Test J and Test K data transfer
    #[inline(always)]
    pub fn hsdrvdclev(&self) -> HSDRVDCLEV_R {
        HSDRVDCLEV_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Enable the HS driver current increase feature
    #[inline(always)]
    pub fn hsdrvcurincr(&self) -> HSDRVCURINCR_R {
        HSDRVCURINCR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Tuning pin to adjust the full speed rise/fall time
    #[inline(always)]
    pub fn fsdrvrfadj(&self) -> FSDRVRFADJ_R {
        FSDRVRFADJ_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - High Speed rise-fall reduction enable
    #[inline(always)]
    pub fn hsdrvrfred(&self) -> HSDRVRFRED_R {
        HSDRVRFRED_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:12 - HS Driver current trimming pins for choke compensation
    #[inline(always)]
    pub fn hsdrvchkitrm(&self) -> HSDRVCHKITRM_R {
        HSDRVCHKITRM_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    ///Bits 13:14 - Controls the PHY bus HS driver impedance tuning for choke compensation
    #[inline(always)]
    pub fn hsdrvchkztrm(&self) -> HSDRVCHKZTRM_R {
        HSDRVCHKZTRM_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bits 15:16 - Adjust the squelch DC threshold value
    #[inline(always)]
    pub fn sqlchctl(&self) -> SQLCHCTL_R {
        SQLCHCTL_R::new(((self.bits >> 15) & 3) as u8)
    }
    ///Bit 17 - Enables the HS Rx Gain Equalizer
    #[inline(always)]
    pub fn hdrxgneqen(&self) -> HDRXGNEQEN_R {
        HDRXGNEQEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - HS Tx staggering enable
    #[inline(always)]
    pub fn stagsel(&self) -> STAGSEL_R {
        STAGSEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - HS Fall time control of single ended signals during pre-emphasis
    #[inline(always)]
    pub fn hsfallpreem(&self) -> HSFALLPREEM_R {
        HSFALLPREEM_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:21 - : HS Receiver Offset adjustment
    #[inline(always)]
    pub fn hsrxoff(&self) -> HSRXOFF_R {
        HSRXOFF_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 22 - Enables the short circuit protection circuitry in LS/FS driver
    #[inline(always)]
    pub fn shtcctctlprot(&self) -> SHTCCTCTLPROT_R {
        SHTCCTCTLPROT_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - This pin is used to bypass the squelch inter-locking circuitry
    #[inline(always)]
    pub fn sqlbyp(&self) -> SQLBYP_R {
        SQLBYP_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TUNE")
            .field("incurren", &self.incurren())
            .field("incurrint", &self.incurrint())
            .field("lfscapen", &self.lfscapen())
            .field("hsdrvslew", &self.hsdrvslew())
            .field("hsdrvdccur", &self.hsdrvdccur())
            .field("hsdrvdclev", &self.hsdrvdclev())
            .field("hsdrvcurincr", &self.hsdrvcurincr())
            .field("fsdrvrfadj", &self.fsdrvrfadj())
            .field("hsdrvrfred", &self.hsdrvrfred())
            .field("hsdrvchkitrm", &self.hsdrvchkitrm())
            .field("hsdrvchkztrm", &self.hsdrvchkztrm())
            .field("sqlchctl", &self.sqlchctl())
            .field("hdrxgneqen", &self.hdrxgneqen())
            .field("stagsel", &self.stagsel())
            .field("hsfallpreem", &self.hsfallpreem())
            .field("hsrxoff", &self.hsrxoff())
            .field("shtcctctlprot", &self.shtcctctlprot())
            .field("sqlbyp", &self.sqlbyp())
            .finish()
    }
}
impl W {
    ///Bit 0 - Controls the current boosting function
    #[inline(always)]
    pub fn incurren(&mut self) -> INCURREN_W<'_, TUNErs> {
        INCURREN_W::new(self, 0)
    }
    ///Bit 1 - Controls PHY current boosting
    #[inline(always)]
    pub fn incurrint(&mut self) -> INCURRINT_W<'_, TUNErs> {
        INCURRINT_W::new(self, 1)
    }
    ///Bit 2 - : Enables the Low Full Speed feedback capacitor
    #[inline(always)]
    pub fn lfscapen(&mut self) -> LFSCAPEN_W<'_, TUNErs> {
        LFSCAPEN_W::new(self, 2)
    }
    ///Bit 3 - Controls the HS driver slew rate
    #[inline(always)]
    pub fn hsdrvslew(&mut self) -> HSDRVSLEW_W<'_, TUNErs> {
        HSDRVSLEW_W::new(self, 3)
    }
    ///Bit 4 - Decreases the HS driver DC level
    #[inline(always)]
    pub fn hsdrvdccur(&mut self) -> HSDRVDCCUR_W<'_, TUNErs> {
        HSDRVDCCUR_W::new(self, 4)
    }
    ///Bit 5 - Increases the HS Driver DC level. Not applicable during the HS Test J and Test K data transfer
    #[inline(always)]
    pub fn hsdrvdclev(&mut self) -> HSDRVDCLEV_W<'_, TUNErs> {
        HSDRVDCLEV_W::new(self, 5)
    }
    ///Bit 6 - Enable the HS driver current increase feature
    #[inline(always)]
    pub fn hsdrvcurincr(&mut self) -> HSDRVCURINCR_W<'_, TUNErs> {
        HSDRVCURINCR_W::new(self, 6)
    }
    ///Bit 7 - Tuning pin to adjust the full speed rise/fall time
    #[inline(always)]
    pub fn fsdrvrfadj(&mut self) -> FSDRVRFADJ_W<'_, TUNErs> {
        FSDRVRFADJ_W::new(self, 7)
    }
    ///Bit 8 - High Speed rise-fall reduction enable
    #[inline(always)]
    pub fn hsdrvrfred(&mut self) -> HSDRVRFRED_W<'_, TUNErs> {
        HSDRVRFRED_W::new(self, 8)
    }
    ///Bits 9:12 - HS Driver current trimming pins for choke compensation
    #[inline(always)]
    pub fn hsdrvchkitrm(&mut self) -> HSDRVCHKITRM_W<'_, TUNErs> {
        HSDRVCHKITRM_W::new(self, 9)
    }
    ///Bits 13:14 - Controls the PHY bus HS driver impedance tuning for choke compensation
    #[inline(always)]
    pub fn hsdrvchkztrm(&mut self) -> HSDRVCHKZTRM_W<'_, TUNErs> {
        HSDRVCHKZTRM_W::new(self, 13)
    }
    ///Bits 15:16 - Adjust the squelch DC threshold value
    #[inline(always)]
    pub fn sqlchctl(&mut self) -> SQLCHCTL_W<'_, TUNErs> {
        SQLCHCTL_W::new(self, 15)
    }
    ///Bit 17 - Enables the HS Rx Gain Equalizer
    #[inline(always)]
    pub fn hdrxgneqen(&mut self) -> HDRXGNEQEN_W<'_, TUNErs> {
        HDRXGNEQEN_W::new(self, 17)
    }
    ///Bit 18 - HS Tx staggering enable
    #[inline(always)]
    pub fn stagsel(&mut self) -> STAGSEL_W<'_, TUNErs> {
        STAGSEL_W::new(self, 18)
    }
    ///Bit 19 - HS Fall time control of single ended signals during pre-emphasis
    #[inline(always)]
    pub fn hsfallpreem(&mut self) -> HSFALLPREEM_W<'_, TUNErs> {
        HSFALLPREEM_W::new(self, 19)
    }
    ///Bits 20:21 - : HS Receiver Offset adjustment
    #[inline(always)]
    pub fn hsrxoff(&mut self) -> HSRXOFF_W<'_, TUNErs> {
        HSRXOFF_W::new(self, 20)
    }
    ///Bit 22 - Enables the short circuit protection circuitry in LS/FS driver
    #[inline(always)]
    pub fn shtcctctlprot(&mut self) -> SHTCCTCTLPROT_W<'_, TUNErs> {
        SHTCCTCTLPROT_W::new(self, 22)
    }
    ///Bit 23 - This pin is used to bypass the squelch inter-locking circuitry
    #[inline(always)]
    pub fn sqlbyp(&mut self) -> SQLBYP_W<'_, TUNErs> {
        SQLBYP_W::new(self, 23)
    }
}
/**USBPHYC tuning control register

You can [`read`](crate::Reg::read) this register and get [`tune::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tune::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F730.html#USBPHYC:TUNE)*/
pub struct TUNErs;
impl crate::RegisterSpec for TUNErs {
    type Ux = u32;
}
///`read()` method returns [`tune::R`](R) reader structure
impl crate::Readable for TUNErs {}
///`write(|w| ..)` method takes [`tune::W`](W) writer structure
impl crate::Writable for TUNErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TUNE to value 0x04
impl crate::Resettable for TUNErs {
    const RESET_VALUE: u32 = 0x04;
}
