#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `RNGEN` reader - True random number generator enable"]
pub type RNGEN_R = crate::BitReader;
#[doc = "Field `RNGEN` writer - True random number generator enable"]
pub type RNGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE` reader - Interrupt Enable"]
pub type IE_R = crate::BitReader;
#[doc = "Field `IE` writer - Interrupt Enable"]
pub type IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CED` reader - Clock error detection The clock error detection cannot be enabled nor disabled on-the-fly when the RNG is enabled, i.e. to enable or disable CED the RNG must be disabled. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK=1."]
pub type CED_R = crate::BitReader;
#[doc = "Field `CED` writer - Clock error detection The clock error detection cannot be enabled nor disabled on-the-fly when the RNG is enabled, i.e. to enable or disable CED the RNG must be disabled. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK=1."]
pub type CED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARDIS` reader - Auto reset disable When auto-reset is enabled application still need to clear SEIS bit after a noise source error. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK=1."]
pub type ARDIS_R = crate::BitReader;
#[doc = "Field `ARDIS` writer - Auto reset disable When auto-reset is enabled application still need to clear SEIS bit after a noise source error. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK=1."]
pub type ARDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNG_CONFIG3` reader - RNG configuration 3 Reserved to the RNG configuration (bitfield 3). Refer to RNG_CONFIG1 bitfield for details. If NISTC bit is cleared in this register RNG_CONFIG3 bitfield values are ignored by RNG."]
pub type RNG_CONFIG3_R = crate::FieldReader;
#[doc = "Field `RNG_CONFIG3` writer - RNG configuration 3 Reserved to the RNG configuration (bitfield 3). Refer to RNG_CONFIG1 bitfield for details. If NISTC bit is cleared in this register RNG_CONFIG3 bitfield values are ignored by RNG."]
pub type RNG_CONFIG3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NISTC` reader - Non NIST compliant four conditioning loops are performed and 512 bits of noise source are used. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK=1."]
pub type NISTC_R = crate::BitReader;
#[doc = "Field `NISTC` writer - Non NIST compliant four conditioning loops are performed and 512 bits of noise source are used. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK=1."]
pub type NISTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNG_CONFIG2` reader - RNG configuration 2 Reserved to the RNG configuration (bitfield 2). Refer to RNG_CONFIG1 bitfield for details."]
pub type RNG_CONFIG2_R = crate::FieldReader;
#[doc = "Field `RNG_CONFIG2` writer - RNG configuration 2 Reserved to the RNG configuration (bitfield 2). Refer to RNG_CONFIG1 bitfield for details."]
pub type RNG_CONFIG2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CLKDIV` reader - Clock divider factor This value used to configure an internal programmable divider (from 1 to 16) acting on the incoming RNG clock. These bits can be written only when the core is disabled (RNGEN=0). ... Writing these bits is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK=1."]
pub type CLKDIV_R = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - Clock divider factor This value used to configure an internal programmable divider (from 1 to 16) acting on the incoming RNG clock. These bits can be written only when the core is disabled (RNGEN=0). ... Writing these bits is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK=1."]
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RNG_CONFIG1` reader - RNG configuration 1 Reserved to the RNG configuration (bitfield 1). Must be initialized using the recommended value documented in . Writing any bit of RNG_CONFIG1 is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK=1."]
pub type RNG_CONFIG1_R = crate::FieldReader;
#[doc = "Field `RNG_CONFIG1` writer - RNG configuration 1 Reserved to the RNG configuration (bitfield 1). Must be initialized using the recommended value documented in . Writing any bit of RNG_CONFIG1 is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK=1."]
pub type RNG_CONFIG1_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CONDRST` reader - Conditioning soft reset Write 1 and then write 0 to reset the conditioning logic, clear all the FIFOs and start a new RNG initialization process, with RNG_SR cleared. Registers RNG_CR and RNG_NSCR are not changed by CONDRST. This bit must be set to 1 in the same access that set any configuration bits \\[29:4\\]. In other words, when CONDRST bit is set to 1 correct configuration in bits \\[29:4\\]
must also be written. When CONDRST is set to 0 by software its value goes to 0 when the reset process is done. It takes about 2 AHB clock cycles + 2 RNG clock cycles."]
pub type CONDRST_R = crate::BitReader;
#[doc = "Field `CONDRST` writer - Conditioning soft reset Write 1 and then write 0 to reset the conditioning logic, clear all the FIFOs and start a new RNG initialization process, with RNG_SR cleared. Registers RNG_CR and RNG_NSCR are not changed by CONDRST. This bit must be set to 1 in the same access that set any configuration bits \\[29:4\\]. In other words, when CONDRST bit is set to 1 correct configuration in bits \\[29:4\\]
must also be written. When CONDRST is set to 0 by software its value goes to 0 when the reset process is done. It takes about 2 AHB clock cycles + 2 RNG clock cycles."]
pub type CONDRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONFIGLOCK` reader - RNG Config Lock This bitfield is set once: if this bit is set it can only be reset to 0 if RNG is reset."]
pub type CONFIGLOCK_R = crate::BitReader;
#[doc = "Field `CONFIGLOCK` writer - RNG Config Lock This bitfield is set once: if this bit is set it can only be reset to 0 if RNG is reset."]
pub type CONFIGLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - True random number generator enable"]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Enable"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock error detection The clock error detection cannot be enabled nor disabled on-the-fly when the RNG is enabled, i.e. to enable or disable CED the RNG must be disabled. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK=1."]
    #[inline(always)]
    pub fn ced(&self) -> CED_R {
        CED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Auto reset disable When auto-reset is enabled application still need to clear SEIS bit after a noise source error. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK=1."]
    #[inline(always)]
    pub fn ardis(&self) -> ARDIS_R {
        ARDIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - RNG configuration 3 Reserved to the RNG configuration (bitfield 3). Refer to RNG_CONFIG1 bitfield for details. If NISTC bit is cleared in this register RNG_CONFIG3 bitfield values are ignored by RNG."]
    #[inline(always)]
    pub fn rng_config3(&self) -> RNG_CONFIG3_R {
        RNG_CONFIG3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Non NIST compliant four conditioning loops are performed and 512 bits of noise source are used. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK=1."]
    #[inline(always)]
    pub fn nistc(&self) -> NISTC_R {
        NISTC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - RNG configuration 2 Reserved to the RNG configuration (bitfield 2). Refer to RNG_CONFIG1 bitfield for details."]
    #[inline(always)]
    pub fn rng_config2(&self) -> RNG_CONFIG2_R {
        RNG_CONFIG2_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Clock divider factor This value used to configure an internal programmable divider (from 1 to 16) acting on the incoming RNG clock. These bits can be written only when the core is disabled (RNGEN=0). ... Writing these bits is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK=1."]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:25 - RNG configuration 1 Reserved to the RNG configuration (bitfield 1). Must be initialized using the recommended value documented in . Writing any bit of RNG_CONFIG1 is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK=1."]
    #[inline(always)]
    pub fn rng_config1(&self) -> RNG_CONFIG1_R {
        RNG_CONFIG1_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Conditioning soft reset Write 1 and then write 0 to reset the conditioning logic, clear all the FIFOs and start a new RNG initialization process, with RNG_SR cleared. Registers RNG_CR and RNG_NSCR are not changed by CONDRST. This bit must be set to 1 in the same access that set any configuration bits \\[29:4\\]. In other words, when CONDRST bit is set to 1 correct configuration in bits \\[29:4\\]
must also be written. When CONDRST is set to 0 by software its value goes to 0 when the reset process is done. It takes about 2 AHB clock cycles + 2 RNG clock cycles."]
    #[inline(always)]
    pub fn condrst(&self) -> CONDRST_R {
        CONDRST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - RNG Config Lock This bitfield is set once: if this bit is set it can only be reset to 0 if RNG is reset."]
    #[inline(always)]
    pub fn configlock(&self) -> CONFIGLOCK_R {
        CONFIGLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - True random number generator enable"]
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RNGEN_W<CRrs> {
        RNGEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<CRrs> {
        IE_W::new(self, 3)
    }
    #[doc = "Bit 5 - Clock error detection The clock error detection cannot be enabled nor disabled on-the-fly when the RNG is enabled, i.e. to enable or disable CED the RNG must be disabled. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK=1."]
    #[inline(always)]
    #[must_use]
    pub fn ced(&mut self) -> CED_W<CRrs> {
        CED_W::new(self, 5)
    }
    #[doc = "Bit 7 - Auto reset disable When auto-reset is enabled application still need to clear SEIS bit after a noise source error. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK=1."]
    #[inline(always)]
    #[must_use]
    pub fn ardis(&mut self) -> ARDIS_W<CRrs> {
        ARDIS_W::new(self, 7)
    }
    #[doc = "Bits 8:11 - RNG configuration 3 Reserved to the RNG configuration (bitfield 3). Refer to RNG_CONFIG1 bitfield for details. If NISTC bit is cleared in this register RNG_CONFIG3 bitfield values are ignored by RNG."]
    #[inline(always)]
    #[must_use]
    pub fn rng_config3(&mut self) -> RNG_CONFIG3_W<CRrs> {
        RNG_CONFIG3_W::new(self, 8)
    }
    #[doc = "Bit 12 - Non NIST compliant four conditioning loops are performed and 512 bits of noise source are used. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK=1."]
    #[inline(always)]
    #[must_use]
    pub fn nistc(&mut self) -> NISTC_W<CRrs> {
        NISTC_W::new(self, 12)
    }
    #[doc = "Bits 13:15 - RNG configuration 2 Reserved to the RNG configuration (bitfield 2). Refer to RNG_CONFIG1 bitfield for details."]
    #[inline(always)]
    #[must_use]
    pub fn rng_config2(&mut self) -> RNG_CONFIG2_W<CRrs> {
        RNG_CONFIG2_W::new(self, 13)
    }
    #[doc = "Bits 16:19 - Clock divider factor This value used to configure an internal programmable divider (from 1 to 16) acting on the incoming RNG clock. These bits can be written only when the core is disabled (RNGEN=0). ... Writing these bits is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK=1."]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<CRrs> {
        CLKDIV_W::new(self, 16)
    }
    #[doc = "Bits 20:25 - RNG configuration 1 Reserved to the RNG configuration (bitfield 1). Must be initialized using the recommended value documented in . Writing any bit of RNG_CONFIG1 is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK=1."]
    #[inline(always)]
    #[must_use]
    pub fn rng_config1(&mut self) -> RNG_CONFIG1_W<CRrs> {
        RNG_CONFIG1_W::new(self, 20)
    }
    #[doc = "Bit 30 - Conditioning soft reset Write 1 and then write 0 to reset the conditioning logic, clear all the FIFOs and start a new RNG initialization process, with RNG_SR cleared. Registers RNG_CR and RNG_NSCR are not changed by CONDRST. This bit must be set to 1 in the same access that set any configuration bits \\[29:4\\]. In other words, when CONDRST bit is set to 1 correct configuration in bits \\[29:4\\]
must also be written. When CONDRST is set to 0 by software its value goes to 0 when the reset process is done. It takes about 2 AHB clock cycles + 2 RNG clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn condrst(&mut self) -> CONDRST_W<CRrs> {
        CONDRST_W::new(self, 30)
    }
    #[doc = "Bit 31 - RNG Config Lock This bitfield is set once: if this bit is set it can only be reset to 0 if RNG is reset."]
    #[inline(always)]
    #[must_use]
    pub fn configlock(&mut self) -> CONFIGLOCK_W<CRrs> {
        CONFIGLOCK_W::new(self, 31)
    }
}
#[doc = "RNG control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0x0087_1f00"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x0087_1f00;
}
