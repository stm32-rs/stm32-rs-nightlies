///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `RNGEN` reader - True random number generator enable
pub type RNGEN_R = crate::BitReader;
///Field `RNGEN` writer - True random number generator enable
pub type RNGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IE` reader - Interrupt enable
pub type IE_R = crate::BitReader;
///Field `IE` writer - Interrupt enable
pub type IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CED` reader - Clock error detection The clock error detection cannot be enabled nor disabled on-the-fly when the RNG is enabled, that is to enable or disable CED, the RNG must be disabled. Writing this bit is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
pub type CED_R = crate::BitReader;
///Field `CED` writer - Clock error detection The clock error detection cannot be enabled nor disabled on-the-fly when the RNG is enabled, that is to enable or disable CED, the RNG must be disabled. Writing this bit is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
pub type CED_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARDIS` reader - Auto reset disable When auto-reset is enabled the application still need to clear the SEIS bit after a noise source error. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
pub type ARDIS_R = crate::BitReader;
///Field `ARDIS` writer - Auto reset disable When auto-reset is enabled the application still need to clear the SEIS bit after a noise source error. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
pub type ARDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNG_CONFIG3` reader - RNG configuration 3 Reserved to the RNG configuration (bitfield 3). Refer to RNG_CONFIG1 bitfield for details. If the NISTC bit is cleared in this register RNG_CONFIG3 bitfield values are ignored by RNG.
pub type RNG_CONFIG3_R = crate::FieldReader;
///Field `RNG_CONFIG3` writer - RNG configuration 3 Reserved to the RNG configuration (bitfield 3). Refer to RNG_CONFIG1 bitfield for details. If the NISTC bit is cleared in this register RNG_CONFIG3 bitfield values are ignored by RNG.
pub type RNG_CONFIG3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `NISTC` reader - NIST custom two conditioning loops are performed and 256 bits of noise source are used. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
pub type NISTC_R = crate::BitReader;
///Field `NISTC` writer - NIST custom two conditioning loops are performed and 256 bits of noise source are used. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
pub type NISTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNG_CONFIG2` reader - RNG configuration 2 Reserved to the RNG configuration (bitfield 2). Bit 13 can be set when RNG power consumption is critical. See Section120.3.8: RNG low-power use. Refer to the RNG_CONFIG1 bitfield for details.
pub type RNG_CONFIG2_R = crate::FieldReader;
///Field `RNG_CONFIG2` writer - RNG configuration 2 Reserved to the RNG configuration (bitfield 2). Bit 13 can be set when RNG power consumption is critical. See Section120.3.8: RNG low-power use. Refer to the RNG_CONFIG1 bitfield for details.
pub type RNG_CONFIG2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CLKDIV` reader - Clock divider factor This value used to configure an internal programmable divider (from 1 to 16) acting on the incoming RNG clock. These bits can be written only when the core is disabled (RNGEN1=10). ... Writing these bits is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
pub type CLKDIV_R = crate::FieldReader;
///Field `CLKDIV` writer - Clock divider factor This value used to configure an internal programmable divider (from 1 to 16) acting on the incoming RNG clock. These bits can be written only when the core is disabled (RNGEN1=10). ... Writing these bits is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RNG_CONFIG1` reader - RNG configuration 1 Reserved to the RNG configuration (bitfield 1). Must be initialized using the recommended value documented in Section120.6: RNG entropy source validation. Writing any bit of RNG_CONFIG1 is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
pub type RNG_CONFIG1_R = crate::FieldReader;
///Field `RNG_CONFIG1` writer - RNG configuration 1 Reserved to the RNG configuration (bitfield 1). Must be initialized using the recommended value documented in Section120.6: RNG entropy source validation. Writing any bit of RNG_CONFIG1 is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
pub type RNG_CONFIG1_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `CONDRST` reader - Conditioning soft reset Write 1 and then write 0 to reset the conditioning logic, clear all the FIFOs and start a new RNG initialization process, with RNG_SR cleared. Registers RNG_CR and RNG_HTCR are not changed by CONDRST. This bit must be set to 1 in the same access that set any configuration bits \[29:4\]. In other words, when CONDRST bit is set to 1 correct configuration in bits \[29:4\] must also be written. When CONDRST is set to 0 by the software, its value goes to 0 when the reset process is done. It takes about 2 AHB clock cycles + 2 RNG clock cycles.
pub type CONDRST_R = crate::BitReader;
///Field `CONDRST` writer - Conditioning soft reset Write 1 and then write 0 to reset the conditioning logic, clear all the FIFOs and start a new RNG initialization process, with RNG_SR cleared. Registers RNG_CR and RNG_HTCR are not changed by CONDRST. This bit must be set to 1 in the same access that set any configuration bits \[29:4\]. In other words, when CONDRST bit is set to 1 correct configuration in bits \[29:4\] must also be written. When CONDRST is set to 0 by the software, its value goes to 0 when the reset process is done. It takes about 2 AHB clock cycles + 2 RNG clock cycles.
pub type CONDRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CONFIGLOCK` reader - RNG Config lock This bitfield is set once: if this bit is set it can only be reset to 0 if RNG is reset.
pub type CONFIGLOCK_R = crate::BitReader;
///Field `CONFIGLOCK` writer - RNG Config lock This bitfield is set once: if this bit is set it can only be reset to 0 if RNG is reset.
pub type CONFIGLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - True random number generator enable
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Interrupt enable
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - Clock error detection The clock error detection cannot be enabled nor disabled on-the-fly when the RNG is enabled, that is to enable or disable CED, the RNG must be disabled. Writing this bit is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
    #[inline(always)]
    pub fn ced(&self) -> CED_R {
        CED_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - Auto reset disable When auto-reset is enabled the application still need to clear the SEIS bit after a noise source error. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
    #[inline(always)]
    pub fn ardis(&self) -> ARDIS_R {
        ARDIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - RNG configuration 3 Reserved to the RNG configuration (bitfield 3). Refer to RNG_CONFIG1 bitfield for details. If the NISTC bit is cleared in this register RNG_CONFIG3 bitfield values are ignored by RNG.
    #[inline(always)]
    pub fn rng_config3(&self) -> RNG_CONFIG3_R {
        RNG_CONFIG3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - NIST custom two conditioning loops are performed and 256 bits of noise source are used. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
    #[inline(always)]
    pub fn nistc(&self) -> NISTC_R {
        NISTC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:15 - RNG configuration 2 Reserved to the RNG configuration (bitfield 2). Bit 13 can be set when RNG power consumption is critical. See Section120.3.8: RNG low-power use. Refer to the RNG_CONFIG1 bitfield for details.
    #[inline(always)]
    pub fn rng_config2(&self) -> RNG_CONFIG2_R {
        RNG_CONFIG2_R::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bits 16:19 - Clock divider factor This value used to configure an internal programmable divider (from 1 to 16) acting on the incoming RNG clock. These bits can be written only when the core is disabled (RNGEN1=10). ... Writing these bits is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:25 - RNG configuration 1 Reserved to the RNG configuration (bitfield 1). Must be initialized using the recommended value documented in Section120.6: RNG entropy source validation. Writing any bit of RNG_CONFIG1 is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
    #[inline(always)]
    pub fn rng_config1(&self) -> RNG_CONFIG1_R {
        RNG_CONFIG1_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    ///Bit 30 - Conditioning soft reset Write 1 and then write 0 to reset the conditioning logic, clear all the FIFOs and start a new RNG initialization process, with RNG_SR cleared. Registers RNG_CR and RNG_HTCR are not changed by CONDRST. This bit must be set to 1 in the same access that set any configuration bits \[29:4\]. In other words, when CONDRST bit is set to 1 correct configuration in bits \[29:4\] must also be written. When CONDRST is set to 0 by the software, its value goes to 0 when the reset process is done. It takes about 2 AHB clock cycles + 2 RNG clock cycles.
    #[inline(always)]
    pub fn condrst(&self) -> CONDRST_R {
        CONDRST_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - RNG Config lock This bitfield is set once: if this bit is set it can only be reset to 0 if RNG is reset.
    #[inline(always)]
    pub fn configlock(&self) -> CONFIGLOCK_R {
        CONFIGLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("rngen", &self.rngen())
            .field("ie", &self.ie())
            .field("ced", &self.ced())
            .field("ardis", &self.ardis())
            .field("rng_config3", &self.rng_config3())
            .field("nistc", &self.nistc())
            .field("rng_config2", &self.rng_config2())
            .field("clkdiv", &self.clkdiv())
            .field("rng_config1", &self.rng_config1())
            .field("condrst", &self.condrst())
            .field("configlock", &self.configlock())
            .finish()
    }
}
impl W {
    ///Bit 2 - True random number generator enable
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W<CRrs> {
        RNGEN_W::new(self, 2)
    }
    ///Bit 3 - Interrupt enable
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W<CRrs> {
        IE_W::new(self, 3)
    }
    ///Bit 5 - Clock error detection The clock error detection cannot be enabled nor disabled on-the-fly when the RNG is enabled, that is to enable or disable CED, the RNG must be disabled. Writing this bit is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
    #[inline(always)]
    pub fn ced(&mut self) -> CED_W<CRrs> {
        CED_W::new(self, 5)
    }
    ///Bit 7 - Auto reset disable When auto-reset is enabled the application still need to clear the SEIS bit after a noise source error. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
    #[inline(always)]
    pub fn ardis(&mut self) -> ARDIS_W<CRrs> {
        ARDIS_W::new(self, 7)
    }
    ///Bits 8:11 - RNG configuration 3 Reserved to the RNG configuration (bitfield 3). Refer to RNG_CONFIG1 bitfield for details. If the NISTC bit is cleared in this register RNG_CONFIG3 bitfield values are ignored by RNG.
    #[inline(always)]
    pub fn rng_config3(&mut self) -> RNG_CONFIG3_W<CRrs> {
        RNG_CONFIG3_W::new(self, 8)
    }
    ///Bit 12 - NIST custom two conditioning loops are performed and 256 bits of noise source are used. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
    #[inline(always)]
    pub fn nistc(&mut self) -> NISTC_W<CRrs> {
        NISTC_W::new(self, 12)
    }
    ///Bits 13:15 - RNG configuration 2 Reserved to the RNG configuration (bitfield 2). Bit 13 can be set when RNG power consumption is critical. See Section120.3.8: RNG low-power use. Refer to the RNG_CONFIG1 bitfield for details.
    #[inline(always)]
    pub fn rng_config2(&mut self) -> RNG_CONFIG2_W<CRrs> {
        RNG_CONFIG2_W::new(self, 13)
    }
    ///Bits 16:19 - Clock divider factor This value used to configure an internal programmable divider (from 1 to 16) acting on the incoming RNG clock. These bits can be written only when the core is disabled (RNGEN1=10). ... Writing these bits is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W<CRrs> {
        CLKDIV_W::new(self, 16)
    }
    ///Bits 20:25 - RNG configuration 1 Reserved to the RNG configuration (bitfield 1). Must be initialized using the recommended value documented in Section120.6: RNG entropy source validation. Writing any bit of RNG_CONFIG1 is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.
    #[inline(always)]
    pub fn rng_config1(&mut self) -> RNG_CONFIG1_W<CRrs> {
        RNG_CONFIG1_W::new(self, 20)
    }
    ///Bit 30 - Conditioning soft reset Write 1 and then write 0 to reset the conditioning logic, clear all the FIFOs and start a new RNG initialization process, with RNG_SR cleared. Registers RNG_CR and RNG_HTCR are not changed by CONDRST. This bit must be set to 1 in the same access that set any configuration bits \[29:4\]. In other words, when CONDRST bit is set to 1 correct configuration in bits \[29:4\] must also be written. When CONDRST is set to 0 by the software, its value goes to 0 when the reset process is done. It takes about 2 AHB clock cycles + 2 RNG clock cycles.
    #[inline(always)]
    pub fn condrst(&mut self) -> CONDRST_W<CRrs> {
        CONDRST_W::new(self, 30)
    }
    ///Bit 31 - RNG Config lock This bitfield is set once: if this bit is set it can only be reset to 0 if RNG is reset.
    #[inline(always)]
    pub fn configlock(&mut self) -> CONFIGLOCK_W<CRrs> {
        CONFIGLOCK_W::new(self, 31)
    }
}
/**RNG control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RNG:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0x0080_0d00
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x0080_0d00;
}
