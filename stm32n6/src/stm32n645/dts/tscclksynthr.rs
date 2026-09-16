///Register `TSCCLKSYNTHR` reader
pub type R = crate::R<TSCCLKSYNTHRrs>;
///Register `TSCCLKSYNTHR` writer
pub type W = crate::W<TSCCLKSYNTHRrs>;
///Field `CLK_SYNTH_LO` reader - Synthesized clk_ts low period
pub type CLK_SYNTH_LO_R = crate::FieldReader;
///Field `CLK_SYNTH_LO` writer - Synthesized clk_ts low period
pub type CLK_SYNTH_LO_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CLK_SYNTH_HI` reader - Synthesized clk_ts high period
pub type CLK_SYNTH_HI_R = crate::FieldReader;
///Field `CLK_SYNTH_HI` writer - Synthesized clk_ts high period
pub type CLK_SYNTH_HI_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CLK_SYNTH_HOLD` reader - SDA master-to-SDA slave output hold delay/SDA slave-to-SDA master input setup delay
pub type CLK_SYNTH_HOLD_R = crate::FieldReader;
///Field `CLK_SYNTH_HOLD` writer - SDA master-to-SDA slave output hold delay/SDA slave-to-SDA master input setup delay
pub type CLK_SYNTH_HOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CLK_SYTH_EN` reader - Synthesized clk_ts enable bit
pub type CLK_SYTH_EN_R = crate::BitReader;
///Field `CLK_SYTH_EN` writer - Synthesized clk_ts enable bit
pub type CLK_SYTH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Synthesized clk_ts low period
    #[inline(always)]
    pub fn clk_synth_lo(&self) -> CLK_SYNTH_LO_R {
        CLK_SYNTH_LO_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Synthesized clk_ts high period
    #[inline(always)]
    pub fn clk_synth_hi(&self) -> CLK_SYNTH_HI_R {
        CLK_SYNTH_HI_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:19 - SDA master-to-SDA slave output hold delay/SDA slave-to-SDA master input setup delay
    #[inline(always)]
    pub fn clk_synth_hold(&self) -> CLK_SYNTH_HOLD_R {
        CLK_SYNTH_HOLD_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bit 24 - Synthesized clk_ts enable bit
    #[inline(always)]
    pub fn clk_syth_en(&self) -> CLK_SYTH_EN_R {
        CLK_SYTH_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSCCLKSYNTHR")
            .field("clk_synth_lo", &self.clk_synth_lo())
            .field("clk_synth_hi", &self.clk_synth_hi())
            .field("clk_synth_hold", &self.clk_synth_hold())
            .field("clk_syth_en", &self.clk_syth_en())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Synthesized clk_ts low period
    #[inline(always)]
    pub fn clk_synth_lo(&mut self) -> CLK_SYNTH_LO_W<'_, TSCCLKSYNTHRrs> {
        CLK_SYNTH_LO_W::new(self, 0)
    }
    ///Bits 8:15 - Synthesized clk_ts high period
    #[inline(always)]
    pub fn clk_synth_hi(&mut self) -> CLK_SYNTH_HI_W<'_, TSCCLKSYNTHRrs> {
        CLK_SYNTH_HI_W::new(self, 8)
    }
    ///Bits 16:19 - SDA master-to-SDA slave output hold delay/SDA slave-to-SDA master input setup delay
    #[inline(always)]
    pub fn clk_synth_hold(&mut self) -> CLK_SYNTH_HOLD_W<'_, TSCCLKSYNTHRrs> {
        CLK_SYNTH_HOLD_W::new(self, 16)
    }
    ///Bit 24 - Synthesized clk_ts enable bit
    #[inline(always)]
    pub fn clk_syth_en(&mut self) -> CLK_SYTH_EN_W<'_, TSCCLKSYNTHRrs> {
        CLK_SYTH_EN_W::new(self, 24)
    }
}
/**DTS TSC clock synthesizer register

You can [`read`](crate::Reg::read) this register and get [`tscclksynthr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscclksynthr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DTS:TSCCLKSYNTHR)*/
pub struct TSCCLKSYNTHRrs;
impl crate::RegisterSpec for TSCCLKSYNTHRrs {
    type Ux = u32;
}
///`read()` method returns [`tscclksynthr::R`](R) reader structure
impl crate::Readable for TSCCLKSYNTHRrs {}
///`write(|w| ..)` method takes [`tscclksynthr::W`](W) writer structure
impl crate::Writable for TSCCLKSYNTHRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TSCCLKSYNTHR to value 0x0001_0000
impl crate::Resettable for TSCCLKSYNTHRrs {
    const RESET_VALUE: u32 = 0x0001_0000;
}
