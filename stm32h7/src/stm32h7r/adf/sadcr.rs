///Register `SADCR` reader
pub type R = crate::R<SADCRrs>;
///Register `SADCR` writer
pub type W = crate::W<SADCRrs>;
///Field `SADEN` reader - Sound activity detector enable This bit is set and cleared by software. It is used to enable/disable the SAD.
pub type SADEN_R = crate::BitReader;
///Field `SADEN` writer - Sound activity detector enable This bit is set and cleared by software. It is used to enable/disable the SAD.
pub type SADEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATCAP` reader - Data capture mode This field is set and cleared by software. It is used to define in which conditions, the samples provided by DLFT0 are stored into the memory. 1x: Samples from DFLT0 transfered into memory when SAD and DFLT0 are enabled Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
pub type DATCAP_R = crate::FieldReader;
///Field `DATCAP` writer - Data capture mode This field is set and cleared by software. It is used to define in which conditions, the samples provided by DLFT0 are stored into the memory. 1x: Samples from DFLT0 transfered into memory when SAD and DFLT0 are enabled Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
pub type DATCAP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DETCFG` reader - Sound trigger event configuration This bit is set and cleared by software. It is used to define if the sddet_evt event is generated only when the SAD enters to MONITOR state or when the SAD enters or exits the DETECT state. Note: This bit can be write-protected (see Section 46.4.13: Register protection for details).
pub type DETCFG_R = crate::BitReader;
///Field `DETCFG` writer - Sound trigger event configuration This bit is set and cleared by software. It is used to define if the sddet_evt event is generated only when the SAD enters to MONITOR state or when the SAD enters or exits the DETECT state. Note: This bit can be write-protected (see Section 46.4.13: Register protection for details).
pub type DETCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SADST` reader - SAD state This field is set and cleared by hardware. It indicates the SAD state and is meaningful only when SADEN = 1. The SAD state can be: - LEARN when the SAD is in learning phase or in SDLVL computation mode - MONITOR when the SAD is in monitoring phase - DETECT when the SAD detects a sound
pub type SADST_R = crate::FieldReader;
///Field `HYSTEN` reader - Hysteresis enable This bit is set and cleared by software. It is used to enable/disable the hysteresis function (see Table 419 for details). This bit must be kept to 0 when SADMOD\[1:0\] = 1x. Note: This bit can be write-protected (see Section 46.4.13: Register protection for details).
pub type HYSTEN_R = crate::BitReader;
///Field `HYSTEN` writer - Hysteresis enable This bit is set and cleared by software. It is used to enable/disable the hysteresis function (see Table 419 for details). This bit must be kept to 0 when SADMOD\[1:0\] = 1x. Note: This bit can be write-protected (see Section 46.4.13: Register protection for details).
pub type HYSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRSIZE` reader - Frame size This field is set and cleared by software. it is used to define the size of one frame and also to define how many samples are taken into account to compute the short-term signal level. 11x: 512 PCM samples used to compute the short-term signal level Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
pub type FRSIZE_R = crate::FieldReader;
///Field `FRSIZE` writer - Frame size This field is set and cleared by software. it is used to define the size of one frame and also to define how many samples are taken into account to compute the short-term signal level. 11x: 512 PCM samples used to compute the short-term signal level Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
pub type FRSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SADMOD` reader - SAD working mode This field is set and cleared by software. It is used to define the way the SAD works. The SAD triggers when the sound level (SDLVL) is bigger than the defined threshold. In this mode, the SAD works like a voice activity detector. The SAD triggers when the sound level (SDLVL) is bigger than the defined threshold. In this mode, the SAD works like a sound detector. 1x: Threshold value given by 4 x ANMIN\[12:0\] The SAD triggers when the estimated ambient noise (ANLVL), multiplied by the gain selected by SNTHR\[3:0\] is bigger than the defined threshold. In this mode, the SAD is working like an ambient noise estimator. Hysteresis function cannot be used in this mode. Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
pub type SADMOD_R = crate::FieldReader;
///Field `SADMOD` writer - SAD working mode This field is set and cleared by software. It is used to define the way the SAD works. The SAD triggers when the sound level (SDLVL) is bigger than the defined threshold. In this mode, the SAD works like a voice activity detector. The SAD triggers when the sound level (SDLVL) is bigger than the defined threshold. In this mode, the SAD works like a sound detector. 1x: Threshold value given by 4 x ANMIN\[12:0\] The SAD triggers when the estimated ambient noise (ANLVL), multiplied by the gain selected by SNTHR\[3:0\] is bigger than the defined threshold. In this mode, the SAD is working like an ambient noise estimator. Hysteresis function cannot be used in this mode. Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
pub type SADMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SADACTIVE` reader - SAD Active flag This bit is set and cleared by hardware. It is used to check if the SAD is effectively enabled (active) or not. The protected fields and registers of this function can only be updated when the SADACTIVE is set to 0 (see Section 46.4.13: Register protection for details). The delay between a transition on SADEN and a transition on SADACTIVE is two periods of AHB clock and two periods of adf_proc_ck.
pub type SADACTIVE_R = crate::BitReader;
impl R {
    ///Bit 0 - Sound activity detector enable This bit is set and cleared by software. It is used to enable/disable the SAD.
    #[inline(always)]
    pub fn saden(&self) -> SADEN_R {
        SADEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Data capture mode This field is set and cleared by software. It is used to define in which conditions, the samples provided by DLFT0 are stored into the memory. 1x: Samples from DFLT0 transfered into memory when SAD and DFLT0 are enabled Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn datcap(&self) -> DATCAP_R {
        DATCAP_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 3 - Sound trigger event configuration This bit is set and cleared by software. It is used to define if the sddet_evt event is generated only when the SAD enters to MONITOR state or when the SAD enters or exits the DETECT state. Note: This bit can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn detcfg(&self) -> DETCFG_R {
        DETCFG_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - SAD state This field is set and cleared by hardware. It indicates the SAD state and is meaningful only when SADEN = 1. The SAD state can be: - LEARN when the SAD is in learning phase or in SDLVL computation mode - MONITOR when the SAD is in monitoring phase - DETECT when the SAD detects a sound
    #[inline(always)]
    pub fn sadst(&self) -> SADST_R {
        SADST_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 7 - Hysteresis enable This bit is set and cleared by software. It is used to enable/disable the hysteresis function (see Table 419 for details). This bit must be kept to 0 when SADMOD\[1:0\] = 1x. Note: This bit can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn hysten(&self) -> HYSTEN_R {
        HYSTEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:10 - Frame size This field is set and cleared by software. it is used to define the size of one frame and also to define how many samples are taken into account to compute the short-term signal level. 11x: 512 PCM samples used to compute the short-term signal level Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn frsize(&self) -> FRSIZE_R {
        FRSIZE_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:13 - SAD working mode This field is set and cleared by software. It is used to define the way the SAD works. The SAD triggers when the sound level (SDLVL) is bigger than the defined threshold. In this mode, the SAD works like a voice activity detector. The SAD triggers when the sound level (SDLVL) is bigger than the defined threshold. In this mode, the SAD works like a sound detector. 1x: Threshold value given by 4 x ANMIN\[12:0\] The SAD triggers when the estimated ambient noise (ANLVL), multiplied by the gain selected by SNTHR\[3:0\] is bigger than the defined threshold. In this mode, the SAD is working like an ambient noise estimator. Hysteresis function cannot be used in this mode. Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn sadmod(&self) -> SADMOD_R {
        SADMOD_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 31 - SAD Active flag This bit is set and cleared by hardware. It is used to check if the SAD is effectively enabled (active) or not. The protected fields and registers of this function can only be updated when the SADACTIVE is set to 0 (see Section 46.4.13: Register protection for details). The delay between a transition on SADEN and a transition on SADACTIVE is two periods of AHB clock and two periods of adf_proc_ck.
    #[inline(always)]
    pub fn sadactive(&self) -> SADACTIVE_R {
        SADACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SADCR")
            .field("saden", &self.saden())
            .field("datcap", &self.datcap())
            .field("detcfg", &self.detcfg())
            .field("sadst", &self.sadst())
            .field("hysten", &self.hysten())
            .field("frsize", &self.frsize())
            .field("sadmod", &self.sadmod())
            .field("sadactive", &self.sadactive())
            .finish()
    }
}
impl W {
    ///Bit 0 - Sound activity detector enable This bit is set and cleared by software. It is used to enable/disable the SAD.
    #[inline(always)]
    pub fn saden(&mut self) -> SADEN_W<'_, SADCRrs> {
        SADEN_W::new(self, 0)
    }
    ///Bits 1:2 - Data capture mode This field is set and cleared by software. It is used to define in which conditions, the samples provided by DLFT0 are stored into the memory. 1x: Samples from DFLT0 transfered into memory when SAD and DFLT0 are enabled Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn datcap(&mut self) -> DATCAP_W<'_, SADCRrs> {
        DATCAP_W::new(self, 1)
    }
    ///Bit 3 - Sound trigger event configuration This bit is set and cleared by software. It is used to define if the sddet_evt event is generated only when the SAD enters to MONITOR state or when the SAD enters or exits the DETECT state. Note: This bit can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn detcfg(&mut self) -> DETCFG_W<'_, SADCRrs> {
        DETCFG_W::new(self, 3)
    }
    ///Bit 7 - Hysteresis enable This bit is set and cleared by software. It is used to enable/disable the hysteresis function (see Table 419 for details). This bit must be kept to 0 when SADMOD\[1:0\] = 1x. Note: This bit can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn hysten(&mut self) -> HYSTEN_W<'_, SADCRrs> {
        HYSTEN_W::new(self, 7)
    }
    ///Bits 8:10 - Frame size This field is set and cleared by software. it is used to define the size of one frame and also to define how many samples are taken into account to compute the short-term signal level. 11x: 512 PCM samples used to compute the short-term signal level Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn frsize(&mut self) -> FRSIZE_W<'_, SADCRrs> {
        FRSIZE_W::new(self, 8)
    }
    ///Bits 12:13 - SAD working mode This field is set and cleared by software. It is used to define the way the SAD works. The SAD triggers when the sound level (SDLVL) is bigger than the defined threshold. In this mode, the SAD works like a voice activity detector. The SAD triggers when the sound level (SDLVL) is bigger than the defined threshold. In this mode, the SAD works like a sound detector. 1x: Threshold value given by 4 x ANMIN\[12:0\] The SAD triggers when the estimated ambient noise (ANLVL), multiplied by the gain selected by SNTHR\[3:0\] is bigger than the defined threshold. In this mode, the SAD is working like an ambient noise estimator. Hysteresis function cannot be used in this mode. Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn sadmod(&mut self) -> SADMOD_W<'_, SADCRrs> {
        SADMOD_W::new(self, 12)
    }
}
/**ADF SAD control register

You can [`read`](crate::Reg::read) this register and get [`sadcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sadcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#ADF:SADCR)*/
pub struct SADCRrs;
impl crate::RegisterSpec for SADCRrs {
    type Ux = u32;
}
///`read()` method returns [`sadcr::R`](R) reader structure
impl crate::Readable for SADCRrs {}
///`write(|w| ..)` method takes [`sadcr::W`](W) writer structure
impl crate::Writable for SADCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SADCR to value 0
impl crate::Resettable for SADCRrs {}
