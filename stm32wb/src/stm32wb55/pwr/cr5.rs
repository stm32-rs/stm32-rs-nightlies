///Register `CR5` reader
pub type R = crate::R<CR5rs>;
///Register `CR5` writer
pub type W = crate::W<CR5rs>;
///Field `SMPSVOS` reader - SMPS step-down converter voltage output scaling
pub type SMPSVOS_R = crate::FieldReader;
///Field `SMPSVOS` writer - SMPS step-down converter voltage output scaling
pub type SMPSVOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SMPSSC` reader - SMPS step-down converter supply startup current selection
pub type SMPSSC_R = crate::FieldReader;
///Field `SMPSSC` writer - SMPS step-down converter supply startup current selection
pub type SMPSSC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `BORHC` reader - BORH configuration selection
pub type BORHC_R = crate::BitReader;
///Field `BORHC` writer - BORH configuration selection
pub type BORHC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPSEN` reader - Enable SMPS step-down converter SMPS mode enabled
pub type SMPSEN_R = crate::BitReader;
///Field `SMPSEN` writer - Enable SMPS step-down converter SMPS mode enabled
pub type SMPSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - SMPS step-down converter voltage output scaling
    #[inline(always)]
    pub fn smpsvos(&self) -> SMPSVOS_R {
        SMPSVOS_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:6 - SMPS step-down converter supply startup current selection
    #[inline(always)]
    pub fn smpssc(&self) -> SMPSSC_R {
        SMPSSC_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 8 - BORH configuration selection
    #[inline(always)]
    pub fn borhc(&self) -> BORHC_R {
        BORHC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 15 - Enable SMPS step-down converter SMPS mode enabled
    #[inline(always)]
    pub fn smpsen(&self) -> SMPSEN_R {
        SMPSEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR5")
            .field("smpsen", &self.smpsen())
            .field("borhc", &self.borhc())
            .field("smpssc", &self.smpssc())
            .field("smpsvos", &self.smpsvos())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - SMPS step-down converter voltage output scaling
    #[inline(always)]
    pub fn smpsvos(&mut self) -> SMPSVOS_W<'_, CR5rs> {
        SMPSVOS_W::new(self, 0)
    }
    ///Bits 4:6 - SMPS step-down converter supply startup current selection
    #[inline(always)]
    pub fn smpssc(&mut self) -> SMPSSC_W<'_, CR5rs> {
        SMPSSC_W::new(self, 4)
    }
    ///Bit 8 - BORH configuration selection
    #[inline(always)]
    pub fn borhc(&mut self) -> BORHC_W<'_, CR5rs> {
        BORHC_W::new(self, 8)
    }
    ///Bit 15 - Enable SMPS step-down converter SMPS mode enabled
    #[inline(always)]
    pub fn smpsen(&mut self) -> SMPSEN_W<'_, CR5rs> {
        SMPSEN_W::new(self, 15)
    }
}
/**Power control register 5

You can [`read`](crate::Reg::read) this register and get [`cr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#PWR:CR5)*/
pub struct CR5rs;
impl crate::RegisterSpec for CR5rs {
    type Ux = u32;
}
///`read()` method returns [`cr5::R`](R) reader structure
impl crate::Readable for CR5rs {}
///`write(|w| ..)` method takes [`cr5::W`](W) writer structure
impl crate::Writable for CR5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR5 to value 0x4270
impl crate::Resettable for CR5rs {
    const RESET_VALUE: u32 = 0x4270;
}
